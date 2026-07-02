//! JSON-RPC gateway dispatches SDK program methods.
use spanda_api::e3::rpc_gateway;
use spanda_api::state::ControlCenterState;
use spanda_security::{ApiKeyRecord, RbacContext, Role};

fn admin_ctx() -> RbacContext {
    RbacContext {
        key_id: "admin".into(),
        role: Role::Administrator,
        tenant_id: "default".into(),
    }
}

#[test]
fn rpc_gateway_lists_entities() {
    let mut state = ControlCenterState::new();
    let body = r#"{"method":"spanda.v1.ControlCenter/ListEntities","params":{}}"#;
    let resp = rpc_gateway(&mut state, body, None);
    assert_eq!(resp.status, 200);
    assert!(resp.body.contains("entities"));
}

#[test]
fn rpc_gateway_entity_graph() {
    let mut state = ControlCenterState::new();
    let body = r#"{"method":"spanda.v1.ControlCenter/GetEntityGraph","params":{}}"#;
    let resp = rpc_gateway(&mut state, body, None);
    assert_eq!(resp.status, 200);
    assert!(resp.body.contains("graph"));
}

#[test]
fn rpc_gateway_list_admin_api_keys_requires_administrator() {
    let mut state = ControlCenterState::new();
    state.api_keys.keys.push(ApiKeyRecord {
        key_id: "admin".into(),
        token: "admin-token".into(),
        role: Role::Administrator,
        label: None,
        tenant_id: "default".into(),
    });
    let body = r#"{"method":"spanda.v1.ControlCenter/ListAdminApiKeys","params":{}}"#;
    let denied = rpc_gateway(&mut state, body, None);
    assert_eq!(denied.status, 200);
    let denied_json: serde_json::Value = serde_json::from_str(&denied.body).expect("json");
    assert_eq!(denied_json["result"]["ok"], false);

    let allowed = rpc_gateway(&mut state, body, Some(&admin_ctx()));
    assert_eq!(allowed.status, 200);
    let allowed_json: serde_json::Value = serde_json::from_str(&allowed.body).expect("json");
    assert!(allowed_json["result"]["keys"].is_array());
}

#[test]
fn rpc_gateway_unknown_method_is_400() {
    let mut state = ControlCenterState::new();
    let resp = rpc_gateway(
        &mut state,
        r#"{"method":"spanda.v1.ControlCenter/Nope"}"#,
        None,
    );
    assert_eq!(resp.status, 400);
}
