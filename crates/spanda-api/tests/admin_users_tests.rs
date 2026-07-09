//! User directory API tests.

mod common;

use common::TempStateDirGuard;
use spanda_api::admin_users::{admin_users_create, admin_users_list};
use spanda_api::ControlCenterState;
use spanda_security::{RbacContext, Role};

fn admin_ctx() -> RbacContext {
    RbacContext::api_key("admin", Role::Administrator, "default")
}

#[test]
fn admin_users_create_and_list() {
    let _state = TempStateDirGuard::new();
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
}
