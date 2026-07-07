//! Administration API key CRUD and mission control endpoints.

use spanda_api::admin_ops::{admin_api_keys_create, admin_api_keys_list};
use spanda_api::ControlCenterState;
use spanda_security::{ApiKeyRecord, RbacContext, Role};
use tempfile::TempDir;

fn admin_ctx() -> RbacContext {
    RbacContext::api_key("admin", Role::Administrator, "default")
}

#[test]
fn admin_api_keys_create_and_list() {
    let dir = TempDir::new().expect("temp dir");
    let keys_path = dir.path().join("keys.json");
    std::env::set_var(
        "SPANDA_API_KEYS_FILE",
        keys_path.to_string_lossy().to_string(),
    );
    let mut state = ControlCenterState::new();
    state.api_keys.keys.push(ApiKeyRecord {
        key_id: "admin".into(),
        token: "admin-token".into(),
        token_hash: None,
        role: Role::Administrator,
        label: None,
        tenant_id: "default".into(),
    });
    let create = admin_api_keys_create(
        &mut state,
        r#"{"role":"operator","label":"ops"}"#,
        Some(&admin_ctx()),
    );
    assert_eq!(create.status, 200, "body: {}", create.body);
    assert!(create.body.contains("\"token\""));
    let list = admin_api_keys_list(&state, Some(&admin_ctx()));
    assert_eq!(list.status, 200);
    assert!(list.body.contains("operator"));
    std::env::remove_var("SPANDA_API_KEYS_FILE");
}

#[test]
fn admin_api_keys_list_requires_administrator() {
    let state = ControlCenterState::new();
    let operator = RbacContext::api_key("op", Role::Operator, "default");
    let list = admin_api_keys_list(&state, Some(&operator));
    assert_eq!(list.status, 401);
}
