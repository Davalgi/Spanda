//! User directory API tests.

use spanda_api::admin_users::{admin_users_create, admin_users_list};
use spanda_api::ControlCenterState;
use spanda_security::{RbacContext, Role};
use tempfile::TempDir;

fn admin_ctx() -> RbacContext {
    RbacContext::api_key("admin", Role::Administrator, "default")
}

#[test]
fn admin_users_create_and_list() {
    let dir = TempDir::new().expect("temp dir");
    std::env::set_var(
        "SPANDA_CONTROL_CENTER_STATE_DIR",
        dir.path().to_string_lossy().to_string(),
    );
    let mut state = ControlCenterState::new();
    let create = admin_users_create(
        &mut state,
        r#"{"user_id":"op-1","display_name":"Operator One","role":"operator"}"#,
        Some(&admin_ctx()),
    );
    assert_eq!(create.status, 200, "{}", create.body);
    let list = admin_users_list(&mut state, Some(&admin_ctx()));
    assert_eq!(list.status, 200);
    assert!(list.body.contains("op-1"));
    std::env::remove_var("SPANDA_CONTROL_CENTER_STATE_DIR");
}
