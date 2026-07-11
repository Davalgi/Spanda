//! REST API tests for NOW differentiation program endpoints.

use spanda_api::handlers::handle_request;
use spanda_api::state::ControlCenterState;
use spanda_deploy_http::HttpRequest;
use std::path::PathBuf;

fn differentiation_program() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/showcase/differentiation/warehouse.sd")
}

fn handle_get(state: &mut ControlCenterState, path: &str) -> spanda_deploy_http::HttpResponse {
    let (response, _) = handle_request(
        state,
        &HttpRequest {
            method: "GET".into(),
            path: path.into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    response
}

fn handle_post(
    state: &mut ControlCenterState,
    path: &str,
    body: &str,
) -> spanda_deploy_http::HttpResponse {
    let (response, _) = handle_request(
        state,
        &HttpRequest {
            method: "POST".into(),
            path: path.into(),
            body: body.into(),
            authorization: None,
        },
        "",
    );
    response
}

fn differentiation_state() -> ControlCenterState {
    let root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/showcase/differentiation");
    let mut state = ControlCenterState::new().with_config_path(root);
    state.program_path = Some(differentiation_program());
    state
}

#[test]
fn program_contract_verify_returns_report() {
    let mut state = differentiation_state();
    let response = handle_post(&mut state, "/v1/programs/contract/verify", r#"{}"#);
    assert_eq!(response.status, 200, "{}", response.body);
    let body: serde_json::Value = serde_json::from_str(&response.body).unwrap();
    assert!(body.get("report").is_some());
    assert!(body.get("passed").is_some());
}

#[test]
fn program_explain_program_mode() {
    let mut state = differentiation_state();
    let response = handle_post(&mut state, "/v1/programs/explain", r#"{"mode":"program"}"#);
    assert_eq!(response.status, 200, "{}", response.body);
    let body: serde_json::Value = serde_json::from_str(&response.body).unwrap();
    assert!(body["report"]["sections"].is_array());
}

#[test]
fn program_source_returns_loaded_program() {
    let mut state = differentiation_state();
    let response = handle_get(&mut state, "/v1/programs/source");
    assert_eq!(response.status, 200, "{}", response.body);
    let body: serde_json::Value = serde_json::from_str(&response.body).unwrap();
    assert!(body["source"].as_str().unwrap_or("").contains("robot"));
}

#[test]
fn program_assure_and_diagnose_return_reports() {
    let mut state = differentiation_state();
    let assure = handle_post(&mut state, "/v1/programs/assure", "{}");
    assert_eq!(assure.status, 200, "{}", assure.body);
    let assure_body: serde_json::Value = serde_json::from_str(&assure.body).unwrap();
    assert!(assure_body.get("passed").is_some() || assure_body.get("report").is_some());

    let diagnose = handle_post(&mut state, "/v1/programs/diagnose", "{}");
    assert_eq!(diagnose.status, 200, "{}", diagnose.body);
    let diagnose_body: serde_json::Value = serde_json::from_str(&diagnose.body).unwrap();
    assert!(
        diagnose_body.get("passed").is_some()
            || diagnose_body.get("report").is_some()
            || diagnose_body.get("diagnoses").is_some()
            || diagnose_body.get("findings").is_some()
            || diagnose_body.as_object().map(|o| !o.is_empty()).unwrap_or(false)
    );
}

#[test]
fn program_audit_decisions_from_trace() {
    let trail = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/showcase/differentiation/decision_trail");
    let trace = trail.join("main.trace");
    if !trace.exists() {
        return;
    }
    let mut state = ControlCenterState::new().with_config_path(trail.clone());
    state.program_path = Some(trail.join("main.sd"));
    let body = format!(r#"{{"file":"{}"}}"#, trace.display());
    let response = handle_post(&mut state, "/v1/programs/audit/decisions", &body);
    assert_eq!(response.status, 200, "{}", response.body);
    let json: serde_json::Value = serde_json::from_str(&response.body).unwrap();
    assert!(json.get("decision_count").is_some());
}
