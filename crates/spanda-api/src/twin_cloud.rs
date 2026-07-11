//! Twin Cloud SaaS REST handlers — mission twin snapshot registry.

use crate::handlers::{bad_request, ensure_rbac, json_ok, parse_query, tenant_forbidden, unauthorized};
use crate::persistence::persist_runtime_state;
use crate::program::parse_program_file;
use crate::state::ControlCenterState;
use spanda_deploy_http::HttpResponse;
use spanda_security::{RbacAction, RbacContext};
use spanda_twin_cloud::{
    build_snapshot_from_program, TwinCloudHistoryResponse, TwinCloudSnapshot,
    TwinCloudSyncResponse, TwinCloudUsageResponse, TWIN_CLOUD_API_VERSION,
};
use std::sync::atomic::Ordering;

pub fn route_twin_cloud(
    state: &mut ControlCenterState,
    path: &str,
    method: &str,
    query: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> Option<HttpResponse> {
    // Dispatch list and usage before path-parameter routes.
    if path == "/v1/twins" && method == "GET" {
        return Some(list_twins(state));
    }
    if path == "/v1/twins/usage" && method == "GET" {
        return Some(get_usage(state));
    }
    if path == "/v1/twins/sync" && method == "POST" {
        return Some(sync_twin(state, query, ctx));
    }
    if path == "/v1/twins/import-replay" && method == "POST" {
        return Some(import_replay(state, body, ctx));
    }
    let rest = path.strip_prefix("/v1/twins/")?;
    if rest.ends_with("/snapshots") && method == "POST" {
        let twin_id = rest.strip_suffix("/snapshots")?;
        return Some(push_snapshot(state, twin_id, body, ctx));
    }
    if rest.ends_with("/history") && method == "GET" {
        let twin_id = rest.strip_suffix("/history")?;
        return Some(get_twin_history(state, twin_id));
    }
    if !rest.contains('/') && method == "GET" {
        return Some(get_twin(state, rest));
    }
    None
}

pub fn list_twins_json(state: &ControlCenterState) -> String {
    list_twins(state).body
}

pub fn get_twin_usage_json(state: &ControlCenterState) -> String {
    // Serialize Twin Cloud usage meters for REST and gRPC callers.
    //
    // Parameters:
    // - `state` — Control Center runtime state
    //
    // Returns:
    // JSON body for `GET /v1/twins/usage` / `GetTwinUsage`.
    //
    // Options:
    // None.
    //
    // Example:
    // let json = get_twin_usage_json(&state);

    get_usage(state).body
}

pub fn get_twin_json(state: &ControlCenterState, twin_id: &str) -> String {
    get_twin(state, twin_id).body
}

pub fn get_twin_history_json(state: &ControlCenterState, twin_id: &str) -> String {
    get_twin_history(state, twin_id).body
}

pub fn sync_twin_json(
    state: &mut ControlCenterState,
    query: &str,
    ctx: Option<&RbacContext>,
) -> String {
    sync_twin(state, query, ctx).body
}

pub fn push_twin_snapshot_json(
    state: &mut ControlCenterState,
    twin_id: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    push_snapshot(state, twin_id, body, ctx).body
}

pub fn import_replay_json(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    import_replay(state, body, ctx).body
}

fn list_twins(state: &ControlCenterState) -> HttpResponse {
    json_ok(
        &state
            .twin_cloud_store
            .list_response(Some(state.tenant_id.as_str())),
    )
}

fn get_usage(state: &ControlCenterState) -> HttpResponse {
    // Build per-tenant meters from store-backed counts plus in-process API counters.
    let tenant = state.tenant_id.as_str();
    json_ok(&TwinCloudUsageResponse {
        version: TWIN_CLOUD_API_VERSION.into(),
        tenant_id: state.tenant_id.clone(),
        twin_count: state.twin_cloud_store.twin_count(Some(tenant)),
        snapshot_count: state.twin_cloud_store.snapshot_count(Some(tenant)),
        push_count: state.twin_cloud_push_count.load(Ordering::Relaxed),
        sync_count: state.twin_cloud_sync_count.load(Ordering::Relaxed),
        history_count: state.twin_cloud_history_count.load(Ordering::Relaxed),
    })
}

fn get_twin(state: &ControlCenterState, twin_id: &str) -> HttpResponse {
    let Some(snapshot) = state.twin_cloud_store.get(twin_id) else {
        return twin_not_found(twin_id);
    };

    // Reject cross-tenant reads so shared Control Center instances stay isolated.
    if snapshot.tenant_id != state.tenant_id {
        return tenant_forbidden();
    }
    json_ok(snapshot)
}

fn get_twin_history(state: &ControlCenterState, twin_id: &str) -> HttpResponse {
    let Some(latest) = state.twin_cloud_store.get(twin_id) else {
        return twin_not_found(twin_id);
    };

    // Reject cross-tenant history so other tenants cannot probe snapshot rings.
    if latest.tenant_id != state.tenant_id {
        return tenant_forbidden();
    }
    let snapshots: Vec<TwinCloudSnapshot> = state
        .twin_cloud_store
        .history(twin_id)
        .into_iter()
        .cloned()
        .collect();
    state
        .twin_cloud_history_count
        .fetch_add(1, Ordering::Relaxed);
    json_ok(&TwinCloudHistoryResponse {
        version: TWIN_CLOUD_API_VERSION.into(),
        twin_id: twin_id.to_string(),
        snapshots,
    })
}

fn twin_not_found(twin_id: &str) -> HttpResponse {
    HttpResponse {
        status: 404,
        body: serde_json::json!({
            "ok": false,
            "error": format!("twin '{twin_id}' not found"),
        })
        .to_string(),
    }
}

fn push_snapshot(
    state: &mut ControlCenterState,
    twin_id: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if ensure_rbac(ctx, RbacAction::Operate).is_err() {
        return unauthorized();
    }
    let mut snapshot: TwinCloudSnapshot = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&format!("invalid snapshot JSON: {error}")),
    };
    if snapshot.twin_id.is_empty() {
        snapshot.twin_id = twin_id.to_string();
    } else if snapshot.twin_id != twin_id {
        return bad_request("snapshot twin_id must match path");
    }

    // Always stamp the instance tenant — never trust a client-supplied tenant_id.
    snapshot.tenant_id = state.tenant_id.clone();
    let stored = state.twin_cloud_store.upsert(snapshot);
    state.twin_cloud_push_count.fetch_add(1, Ordering::Relaxed);
    let _ = persist_runtime_state(state);
    json_ok(&TwinCloudSyncResponse {
        version: TWIN_CLOUD_API_VERSION.into(),
        twin_id: stored.twin_id.clone(),
        captured_at_ms: stored.captured_at_ms,
        snapshot: stored,
    })
}

fn sync_twin(
    state: &mut ControlCenterState,
    query: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if ensure_rbac(ctx, RbacAction::Operate).is_err() {
        return unauthorized();
    }
    let params = parse_query(query);
    let twin_id = params.get("twin_id").map(String::as_str);
    let (program, _source, label) = match load_program(state) {
        Ok(value) => value,
        Err(message) => return bad_request(&message),
    };
    let snapshot = build_snapshot_from_program(&program, &label, twin_id, state.tenant_id.as_str());
    let stored = state.twin_cloud_store.upsert(snapshot);
    state.twin_cloud_sync_count.fetch_add(1, Ordering::Relaxed);
    let _ = persist_runtime_state(state);
    json_ok(&TwinCloudSyncResponse {
        version: TWIN_CLOUD_API_VERSION.into(),
        twin_id: stored.twin_id.clone(),
        captured_at_ms: stored.captured_at_ms,
        snapshot: stored,
    })
}

fn import_replay(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if ensure_rbac(ctx, RbacAction::Operate).is_err() {
        return unauthorized();
    }
    let payload: serde_json::Value = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&format!("invalid replay JSON: {error}")),
    };
    let program_path = payload
        .get("program")
        .or_else(|| payload.get("source"))
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let twin_id = payload
        .get("twin_id")
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let snapshot = if let Some(path) = program_path {
        let source = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => return bad_request(&format!("read program failed: {error}")),
        };
        let tokens = match spanda_lexer::tokenize(&source) {
            Ok(value) => value,
            Err(error) => return bad_request(&error.to_string()),
        };
        let program = match spanda_parser::parse(tokens) {
            Ok(value) => value,
            Err(error) => return bad_request(&error.to_string()),
        };
        build_snapshot_from_program(
            &program,
            &path,
            twin_id.as_deref(),
            state.tenant_id.as_str(),
        )
    } else {
        return bad_request("replay JSON must include program or source path");
    };
    let stored = state.twin_cloud_store.upsert(snapshot);
    state.twin_cloud_push_count.fetch_add(1, Ordering::Relaxed);
    let _ = persist_runtime_state(state);
    json_ok(&serde_json::json!({
        "ok": true,
        "legacy": "SPANDA_CLOUD_UPLOAD_URL",
        "twin_id": stored.twin_id,
        "snapshot": stored,
    }))
}

fn load_program(
    state: &ControlCenterState,
) -> Result<(spanda_ast::nodes::Program, String, String), String> {
    let Some(path) = state.program_path.as_ref() else {
        return Err("no program loaded; use control-center serve --program <file.sd>".into());
    };
    parse_program_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_security::{ApiKeyRecord, ApiKeyStore, Role};
    use std::path::PathBuf;

    fn patrol_program() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/showcase/mission_twin/patrol.sd")
    }

    fn operator_store(token: &str) -> ApiKeyStore {
        ApiKeyStore {
            keys: vec![ApiKeyRecord {
                key_id: "test".into(),
                token: token.into(),
                token_hash: None,
                role: Role::Operator,
                label: None,
                tenant_id: "default".into(),
            }],
        }
    }

    #[test]
    fn twin_cloud_sync_stores_snapshot() {
        let program = patrol_program();
        assert!(program.exists());
        let mut state = ControlCenterState::new();
        state.program_path = Some(program);
        state.api_keys = operator_store("twin-cloud-sync-test");
        let ctx = state
            .api_keys
            .authenticate(Some("twin-cloud-sync-test"))
            .expect("auth ctx");
        let response = sync_twin(&mut state, "", Some(&ctx));
        assert_eq!(response.status, 200, "{}", response.body);
        let json: serde_json::Value = serde_json::from_str(&response.body).unwrap();
        assert_eq!(json["twin_id"], "patrol");
        assert_eq!(state.twin_cloud_sync_count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn twin_cloud_sync_requires_operate_rbac() {
        let program = patrol_program();
        let mut state = ControlCenterState::new();
        state.program_path = Some(program);
        let response = sync_twin(&mut state, "", None);
        assert_eq!(response.status, 401);
    }

    #[test]
    fn twin_cloud_usage_reports_store_and_counters() {
        let program = patrol_program();
        let mut state = ControlCenterState::new();
        state.program_path = Some(program);
        state.api_keys = operator_store("twin-cloud-usage-test");
        let ctx = state
            .api_keys
            .authenticate(Some("twin-cloud-usage-test"))
            .expect("auth ctx");
        let sync = sync_twin(&mut state, "", Some(&ctx));
        assert_eq!(sync.status, 200, "{}", sync.body);
        let history = get_twin_history(&state, "patrol");
        assert_eq!(history.status, 200, "{}", history.body);
        let usage = get_usage(&state);
        assert_eq!(usage.status, 200, "{}", usage.body);
        let json: serde_json::Value = serde_json::from_str(&usage.body).unwrap();
        assert_eq!(json["twin_count"], 1);
        assert!(json["snapshot_count"].as_u64().unwrap() >= 1);
        assert_eq!(json["sync_count"], 1);
        assert_eq!(json["history_count"], 1);
    }
}
