//! API endpoint tests for device pool routes.

use spanda_api::handlers::handle_request;
use spanda_api::state::ControlCenterState;
use spanda_config::ConfigResolver;
use spanda_deploy_http::HttpRequest;
use std::path::PathBuf;

fn warehouse_state() -> ControlCenterState {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("spanda-config/tests/fixtures/warehouse");
    let resolved = ConfigResolver::new()
        .resolve_from_dir(&root)
        .expect("resolve warehouse");
    let mut state = ControlCenterState::new();
    state.resolved = Some(resolved);
    state
}

#[test]
fn devices_list_returns_pool() {
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/devices".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("devices"));
}

#[test]
fn device_get_by_id() {
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/devices/gps-001".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("gps-001"));
}

#[test]
fn robots_and_fleets_endpoints() {
    let mut state = warehouse_state();
    let (robots, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/robots".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(robots.status, 200);
    assert!(robots.body.contains("rover-001"));

    let (fleets, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/fleets".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(fleets.status, 200);
    assert!(fleets.body.contains("warehouse-fleet"));
}

#[test]
fn readiness_run_endpoint() {
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "POST".into(),
            path: "/v1/readiness/run".into(),
            body: "{}".into(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("mission_ready"));
}

#[test]
fn device_tree_endpoint() {
    let mut state = warehouse_state();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/device-tree".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("mapping"));
}
