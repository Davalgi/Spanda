//! Administration REST handlers — API keys, integrations metadata.
//!
use crate::handlers::{bad_request, json_ok, unauthorized};
use crate::state::ControlCenterState;
use serde::Deserialize;
use spanda_config::{
    default_entity_overlay_path, default_mission_approvals_path, load_mission_approval_queue,
    save_entity_overlay, EntityKind, EntityRecord,
};
use spanda_deploy_http::HttpResponse;
use spanda_security::{
    generate_api_key_token, ApiKeyRecord, ApiKeyStore, RbacAction, RbacContext, Role,
};

const API_VERSION: &str = "v1";

pub fn not_found_response() -> HttpResponse {
    HttpResponse {
        status: 404,
        body: serde_json::json!({ "ok": false, "error": "not found" }).to_string(),
    }
}

fn require_admin(ctx: Option<&RbacContext>) -> Result<(), HttpResponse> {
    match ctx {
        Some(c) if c.role == Role::Administrator => Ok(()),
        Some(_) => Err(unauthorized()),
        None => Err(unauthorized()),
    }
}

/// GET /v1/admin/api-keys — list key metadata (no token values).
pub fn admin_api_keys_list(state: &ControlCenterState, ctx: Option<&RbacContext>) -> HttpResponse {
    if require_admin(ctx).is_err() {
        return unauthorized();
    }
    let keys: Vec<_> = state
        .api_keys
        .keys
        .iter()
        .map(|key| {
            serde_json::json!({
                "key_id": key.key_id,
                "role": key.role,
                "label": key.label,
                "tenant_id": key.tenant_id,
                "source": if key.key_id == "env-default" { "env" } else { "file" },
            })
        })
        .collect();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "keys": keys,
        "count": keys.len(),
        "persist_path": ApiKeyStore::default_api_keys_file_path().display().to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub role: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub tenant_id: Option<String>,
}

/// POST /v1/admin/api-keys — create a key; token returned once.
pub fn admin_api_keys_create(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if require_admin(ctx).is_err() {
        return unauthorized();
    }
    let request: CreateApiKeyRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    let role = Role::parse(&request.role);
    if role == Role::Guest {
        return bad_request("role must be administrator, supervisor, developer, operator, safety_officer, or auditor");
    }
    let token = generate_api_key_token();
    let key_id = format!("key-{}", crate::handlers::now_ms() as u64);
    let tenant_id = request.tenant_id.unwrap_or_else(|| state.tenant_id.clone());
    let record = ApiKeyRecord {
        key_id: key_id.clone(),
        token: token.clone(),
        role,
        label: request.label,
        tenant_id,
    };
    state.api_keys.keys.push(record);
    if let Err(error) = state.api_keys.persist_file_keys() {
        state.api_keys.keys.pop();
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "key_id": key_id,
        "token": token,
        "role": role,
        "note": "Store the token now — it cannot be retrieved again.",
    }))
}

#[derive(Debug, Deserialize)]
pub struct PatchApiKeyRequest {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
}

/// PATCH /v1/admin/api-keys/{key_id}
pub fn admin_api_keys_patch(
    state: &mut ControlCenterState,
    key_id: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if require_admin(ctx).is_err() {
        return unauthorized();
    }
    if key_id == "env-default" {
        return bad_request("cannot modify SPANDA_API_KEY env-default key via API");
    }
    let request: PatchApiKeyRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    let Some(record) = state.api_keys.keys.iter_mut().find(|k| k.key_id == key_id) else {
        return not_found_response();
    };
    if let Some(role_str) = request.role {
        let role = Role::parse(&role_str);
        if role == Role::Guest {
            return bad_request("invalid role");
        }
        record.role = role;
    }
    if let Some(label) = request.label {
        record.label = Some(label);
    }
    if let Err(error) = state.api_keys.persist_file_keys() {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "key_id": key_id,
    }))
}

/// DELETE /v1/admin/api-keys/{key_id}
pub fn admin_api_keys_delete(
    state: &mut ControlCenterState,
    key_id: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if require_admin(ctx).is_err() {
        return unauthorized();
    }
    if key_id == "env-default" {
        return bad_request("cannot revoke SPANDA_API_KEY env-default key via API");
    }
    let before = state.api_keys.keys.len();
    state.api_keys.keys.retain(|k| k.key_id != key_id);
    if state.api_keys.keys.len() == before {
        return not_found_response();
    }
    if let Err(error) = state.api_keys.persist_file_keys() {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "revoked": key_id,
    }))
}

/// GET /v1/admin/integrations — alert channels and observability backends (read-only).
pub fn admin_integrations_summary(
    state: &ControlCenterState,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Deploy) {
        return unauthorized();
    }
    let channels: Vec<String> = state
        .alert_dispatcher
        .channels
        .iter()
        .map(|channel| format!("{channel:?}"))
        .collect();
    let mut observability = spanda_ops::observability_backend_summary();
    if let Some(url) = std::env::var("SPANDA_GRAFANA_URL").ok() {
        if let Some(obj) = observability.as_object_mut() {
            obj.insert("grafana_url".into(), serde_json::Value::String(url));
        }
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "alert_channels": channels,
        "observability": observability,
        "tenant_id": state.tenant_id,
        "api_keys_loaded": state.api_keys.keys.len(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct MissionControlRequest {
    pub mission_id: String,
    #[serde(default)]
    pub note: Option<String>,
}

fn find_mission_entity_id(state: &ControlCenterState, mission_id: &str) -> Option<String> {
    let registry = state.entity_registry();
    registry
        .list()
        .into_iter()
        .find(|entity| {
            entity.entity_type == EntityKind::Mission
                && (entity.id == mission_id
                    || entity.name.as_deref() == Some(mission_id)
                    || entity
                        .metadata
                        .get("mission_name")
                        .map(String::as_str)
                        .is_some_and(|name| name == mission_id))
        })
        .map(|entity| entity.id.clone())
}

fn set_mission_state(
    state: &mut ControlCenterState,
    mission_id: &str,
    new_state: &str,
) -> Result<EntityRecord, String> {
    let entity_id = find_mission_entity_id(state, mission_id)
        .ok_or_else(|| format!("mission '{mission_id}' not found"))?;
    let registry = state.entity_registry();
    let mut record = state
        .entity_overlay
        .entities
        .get(&entity_id)
        .cloned()
        .or_else(|| registry.get(&entity_id).cloned())
        .ok_or_else(|| format!("entity '{entity_id}' not found"))?;
    record
        .metadata
        .insert("mission_state".into(), new_state.into());
    let (health, readiness, lifecycle) = mission_status_to_entity_state(new_state);
    record.health_status = health;
    record.readiness_status = readiness;
    record.lifecycle_state = lifecycle;
    state
        .entity_overlay
        .entities
        .insert(entity_id.clone(), record.clone());
    state.entity_overlay.version = state.entity_overlay.version.saturating_add(1);
    save_entity_overlay(&default_entity_overlay_path(), &state.entity_overlay)
        .map_err(|error| error.to_string())?;
    Ok(record)
}

fn mission_status_to_entity_state(
    mission_state: &str,
) -> (
    spanda_config::EntityHealthStatus,
    spanda_config::EntityReadinessStatus,
    spanda_config::EntityLifecycleState,
) {
    match mission_state.to_ascii_lowercase().as_str() {
        "running" | "active" | "resumed" => (
            spanda_config::EntityHealthStatus::Healthy,
            spanda_config::EntityReadinessStatus::Ready,
            spanda_config::EntityLifecycleState::Active,
        ),
        "paused" => (
            spanda_config::EntityHealthStatus::Warning,
            spanda_config::EntityReadinessStatus::Partial,
            spanda_config::EntityLifecycleState::Suspended,
        ),
        "cancelled" | "canceled" => (
            spanda_config::EntityHealthStatus::Critical,
            spanda_config::EntityReadinessStatus::NotReady,
            spanda_config::EntityLifecycleState::Degraded,
        ),
        "completed" => (
            spanda_config::EntityHealthStatus::Healthy,
            spanda_config::EntityReadinessStatus::Ready,
            spanda_config::EntityLifecycleState::Archived,
        ),
        _ => (
            spanda_config::EntityHealthStatus::Unknown,
            spanda_config::EntityReadinessStatus::Unknown,
            spanda_config::EntityLifecycleState::Unknown,
        ),
    }
}

/// GET /v1/operator/missions — mission entities with runtime state.
pub fn operator_missions_list(state: &ControlCenterState) -> HttpResponse {
    let registry = state.entity_registry();
    let missions: Vec<_> = registry
        .list()
        .into_iter()
        .filter(|entity| entity.entity_type == EntityKind::Mission)
        .map(|entity| {
            serde_json::json!({
                "id": entity.id,
                "name": entity.name,
                "mission_state": entity.metadata.get("mission_state").cloned().unwrap_or_else(|| "unknown".into()),
                "lifecycle_state": format!("{:?}", entity.lifecycle_state),
                "readiness": format!("{:?}", entity.readiness_status),
            })
        })
        .collect();
    let path = default_mission_approvals_path();
    let queue = load_mission_approval_queue(&path).unwrap_or_default();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "missions": missions,
        "count": missions.len(),
        "pending_approvals": queue.requests.iter().filter(|r| {
            format!("{:?}", r.status).to_ascii_lowercase().contains("pending")
        }).count(),
    }))
}

/// POST /v1/operator/mission/pause
pub fn operator_mission_pause(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let request: MissionControlRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    match set_mission_state(state, &request.mission_id, "paused") {
        Ok(entity) => json_ok(&serde_json::json!({
            "version": API_VERSION,
            "ok": true,
            "mission_id": request.mission_id,
            "mission_state": "paused",
            "entity_id": entity.id,
            "note": request.note,
        })),
        Err(error) => bad_request(&error),
    }
}

/// POST /v1/operator/mission/resume
pub fn operator_mission_resume(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let request: MissionControlRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    match set_mission_state(state, &request.mission_id, "running") {
        Ok(entity) => json_ok(&serde_json::json!({
            "version": API_VERSION,
            "ok": true,
            "mission_id": request.mission_id,
            "mission_state": "running",
            "entity_id": entity.id,
            "note": request.note,
        })),
        Err(error) => bad_request(&error),
    }
}

/// POST /v1/operator/mission/cancel
pub fn operator_mission_cancel(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Shutdown) {
        return unauthorized();
    }
    let request: MissionControlRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    match set_mission_state(state, &request.mission_id, "cancelled") {
        Ok(entity) => json_ok(&serde_json::json!({
            "version": API_VERSION,
            "ok": true,
            "mission_id": request.mission_id,
            "mission_state": "cancelled",
            "entity_id": entity.id,
            "note": request.note,
        })),
        Err(error) => bad_request(&error),
    }
}

/// Route administration and mission-control subpaths.
pub fn route_admin(
    state: &mut ControlCenterState,
    path: &str,
    method: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> Option<HttpResponse> {
    if path == "/v1/admin/api-keys" && method == "GET" {
        return Some(admin_api_keys_list(state, ctx));
    }
    if path == "/v1/admin/api-keys" && method == "POST" {
        return Some(admin_api_keys_create(state, body, ctx));
    }
    if path == "/v1/admin/integrations" && method == "GET" {
        return Some(admin_integrations_summary(state, ctx));
    }
    if path == "/v1/operator/missions" && method == "GET" {
        return Some(operator_missions_list(state));
    }
    if path == "/v1/operator/mission/pause" && method == "POST" {
        return Some(operator_mission_pause(state, body, ctx));
    }
    if path == "/v1/operator/mission/resume" && method == "POST" {
        return Some(operator_mission_resume(state, body, ctx));
    }
    if path == "/v1/operator/mission/cancel" && method == "POST" {
        return Some(operator_mission_cancel(state, body, ctx));
    }
    if path == "/v1/admin/oidc" && method == "GET" {
        return Some(crate::control_center_extras::admin_oidc_get(ctx));
    }
    if path == "/v1/admin/oidc" && method == "PUT" {
        return Some(crate::control_center_extras::admin_oidc_put(body, ctx));
    }
    if path == "/v1/admin/oidc/sync" && method == "POST" {
        return Some(crate::control_center_extras::admin_oidc_sync(
            state, body, ctx,
        ));
    }
    if path == "/v1/admin/oidc/authorize-url" && method == "POST" {
        return Some(crate::control_center_extras::admin_oidc_authorize_url(
            body, ctx,
        ));
    }
    if path == "/v1/admin/oidc/oauth/callback" && method == "POST" {
        return Some(crate::control_center_extras::admin_oidc_oauth_callback(
            state, body, ctx,
        ));
    }
    if path == "/v1/admin/slack" && method == "GET" {
        return Some(crate::control_center_extras::admin_slack_get(ctx));
    }
    if path == "/v1/admin/slack" && method == "POST" {
        return Some(crate::control_center_extras::admin_slack_post(body, ctx));
    }
    if path == "/v1/admin/slack/oauth-url" && method == "POST" {
        return Some(crate::control_center_extras::admin_slack_oauth_url(
            body, ctx,
        ));
    }
    if path == "/v1/admin/slack/oauth/callback" && method == "POST" {
        return Some(crate::control_center_extras::admin_slack_oauth_callback(
            body, ctx,
        ));
    }
    let rest = path.strip_prefix("/v1/admin/api-keys/")?;
    match method {
        "PATCH" => Some(admin_api_keys_patch(state, rest, body, ctx)),
        "DELETE" => Some(admin_api_keys_delete(state, rest, ctx)),
        _ => None,
    }
}

pub fn admin_api_keys_list_json(state: &ControlCenterState, ctx: Option<&RbacContext>) -> String {
    admin_api_keys_list(state, ctx).body
}

pub fn admin_api_keys_create_json(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    admin_api_keys_create(state, body, ctx).body
}

pub fn admin_api_keys_patch_json(
    state: &mut ControlCenterState,
    key_id: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    admin_api_keys_patch(state, key_id, body, ctx).body
}

pub fn admin_api_keys_delete_json(
    state: &mut ControlCenterState,
    key_id: &str,
    ctx: Option<&RbacContext>,
) -> String {
    admin_api_keys_delete(state, key_id, ctx).body
}

pub fn admin_integrations_summary_json(
    state: &ControlCenterState,
    ctx: Option<&RbacContext>,
) -> String {
    admin_integrations_summary(state, ctx).body
}

pub fn operator_missions_list_json(state: &ControlCenterState) -> String {
    operator_missions_list(state).body
}

pub fn operator_mission_pause_json(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    operator_mission_pause(state, body, ctx).body
}

pub fn operator_mission_resume_json(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    operator_mission_resume(state, body, ctx).body
}

pub fn operator_mission_cancel_json(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    operator_mission_cancel(state, body, ctx).body
}
