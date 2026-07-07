//! Multi-tenant isolation and HA persistence tests for Control Center.

use spanda_api::handlers::handle_request;
use spanda_api::persistence::persist_runtime_state;
use spanda_api::state::ControlCenterState;
use spanda_deploy_http::HttpRequest;
use spanda_ops::{Alert, AlertSeverity, AlertType};
use spanda_security::{ApiKeyStore, Role};
use std::path::PathBuf;
use std::sync::Mutex;
use tempfile::TempDir;

static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn tenant_endpoint_reports_instance_tenant() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    std::env::set_var("SPANDA_TENANT_ID", "acme");
    let mut state = ControlCenterState::new();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/tenant".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("\"tenant_id\":\"acme\""));
}

#[test]
fn tenant_mismatch_returns_403_for_authenticated_request() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    std::env::set_var("SPANDA_TENANT_ID", "acme");
    std::env::set_var("SPANDA_API_KEY", "tenant-mismatch-key");
    let mut state = ControlCenterState::new();
    state.api_keys.keys[0].tenant_id = "other".into();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/dashboard".into(),
            body: String::new(),
            authorization: Some("tenant-mismatch-key".into()),
        },
        "",
    );
    assert_eq!(response.status, 403);
    assert!(response.body.contains("tenant mismatch"));
}

#[test]
fn runtime_state_persists_alerts_and_traces() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let dir = TempDir::new().expect("temp dir");
    std::env::set_var(
        "SPANDA_CONTROL_CENTER_STATE_DIR",
        dir.path().to_string_lossy().to_string(),
    );

    let mut state = ControlCenterState::new();
    state.alert_store.push(Alert {
        id: "persist-alert-1".into(),
        alert_type: AlertType::Custom,
        severity: AlertSeverity::Info,
        message: "persisted".into(),
        source: "test".into(),
        timestamp_ms: 1.0,
        delivered_via: vec![],
    });
    state.trace_log.push(spanda_api::correlation::TraceRecord {
        correlation_id: "corr-1".into(),
        method: "GET".into(),
        path: "/v1/health".into(),
        status: 200,
        timestamp_ms: 1.0,
        duration_ms: Some(1.0),
    });
    persist_runtime_state(&state).expect("persist");

    let reloaded = ControlCenterState::new();
    assert_eq!(reloaded.alert_store.list_owned().len(), 1);
    assert_eq!(reloaded.trace_log.list_owned().len(), 1);
    assert_eq!(reloaded.alert_store.list_owned()[0].id, "persist-alert-1");
}

#[test]
fn runtime_state_persists_twin_cloud_snapshots() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let dir = TempDir::new().expect("temp dir");
    std::env::set_var(
        "SPANDA_CONTROL_CENTER_STATE_DIR",
        dir.path().to_string_lossy().to_string(),
    );

    let program = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/showcase/mission_twin/patrol.sd");
    std::env::set_var("SPANDA_API_KEY", "twin-cloud-persist-test");
    let mut state = ControlCenterState::new();
    state.program_path = Some(program);
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "POST".into(),
            path: "/v1/twins/sync".into(),
            body: "{}".into(),
            authorization: Some("twin-cloud-persist-test".into()),
        },
        "",
    );
    assert_eq!(response.status, 200, "{}", response.body);
    persist_runtime_state(&state).expect("persist");

    let reloaded = ControlCenterState::new();
    assert_eq!(reloaded.twin_cloud_store.list_owned().len(), 1);
    assert_eq!(
        reloaded
            .twin_cloud_store
            .get("patrol")
            .expect("patrol")
            .twin_id,
        "patrol"
    );
}

#[test]
fn recovery_history_persists_across_restart() {
    use spanda_api::persistence::persist_runtime_state;
    use spanda_api::recovery_ops::recovery_history;
    use spanda_recovery::{
        OrchestratorRecoveryEvidence, OrchestratorStrategy, RecoveryHistoryStore,
    };
    use spanda_runtime::recovery_types::RecoveryStatus;

    let dir = TempDir::new().expect("temp dir");
    std::env::set_var(
        "SPANDA_CONTROL_CENTER_STATE_DIR",
        dir.path().to_string_lossy().to_string(),
    );

    let mut state = ControlCenterState::new();
    state.recovery_history = RecoveryHistoryStore {
        evidence: vec![OrchestratorRecoveryEvidence {
            evidence_id: "ev-persist-1".into(),
            root_cause: "gps_loss".into(),
            strategy: OrchestratorStrategy::Reconnect,
            timeline: vec![],
            entities_involved: vec!["Rover".into()],
            safety_validation: "pass".into(),
            readiness_result: "pass".into(),
            trust_result: "pass".into(),
            operator_actions: vec![],
            automatic_decisions: vec![],
            mission_impact: "low".into(),
            duration_secs: 30,
            status: RecoveryStatus::Success,
            timestamp: "2026-01-01T00:00:00Z".into(),
        }],
    };
    persist_runtime_state(&state).expect("persist");

    let reloaded = ControlCenterState::new();
    let resp = recovery_history(&reloaded);
    assert_eq!(resp.status, 200);
    let json: serde_json::Value = serde_json::from_str(&resp.body).unwrap();
    assert_eq!(json["count"], 1);
}

#[test]
fn api_keys_file_merges_with_env_key() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let dir = TempDir::new().expect("temp dir");
    let keys_path = dir.path().join("keys.json");
    std::fs::write(
        &keys_path,
        serde_json::to_string(&vec![spanda_security::ApiKeyRecord {
            key_id: "file-key".into(),
            token: "file-token".into(),
            token_hash: None,
            role: Role::Operator,
            label: None,
            tenant_id: "default".into(),
        }])
        .expect("serialize"),
    )
    .expect("write keys");
    std::env::set_var(
        "SPANDA_API_KEYS_FILE",
        keys_path.to_string_lossy().to_string(),
    );
    std::env::set_var("SPANDA_API_KEY", "env-token");
    let store = ApiKeyStore::from_env_and_file();
    assert_eq!(store.keys.len(), 2);
}
