//! Authentication handler tests — hashed keys, read policy, session JWTs.

use spanda_api::handlers::handle_request;
use spanda_api::state::ControlCenterState;
use spanda_deploy_http::HttpRequest;
use spanda_security::{
    api_key_pepper, hash_api_key_token, ApiKeyRecord, ApiKeyStore, AuthHandler, Role,
    SessionTokenIssuer,
};
use std::sync::Mutex;
use tempfile::TempDir;

static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn hashed_api_key_authenticates() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let token = "hashed-key-test";
    let pepper = api_key_pepper();
    let mut store = ApiKeyStore::new();
    store.keys.push(ApiKeyRecord {
        key_id: "hashed".into(),
        token: String::new(),
        token_hash: Some(hash_api_key_token(token, &pepper)),
        role: Role::Operator,
        label: None,
        tenant_id: "default".into(),
    });
    let handler = AuthHandler::new();
    let ctx = handler
        .authenticate(&store, Some(token))
        .expect("authenticate hashed key");
    assert_eq!(ctx.role, Role::Operator);
}

#[test]
fn sensitive_read_requires_auth_when_flag_set() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    std::env::set_var("SPANDA_API_REQUIRE_AUTH_READS", "1");
    let mut state = ControlCenterState::new();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/dashboard".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 401);
    std::env::remove_var("SPANDA_API_REQUIRE_AUTH_READS");
}

#[test]
fn auth_config_endpoint_is_public() {
    let mut state = ControlCenterState::new();
    let (response, _) = handle_request(
        &mut state,
        &HttpRequest {
            method: "GET".into(),
            path: "/v1/auth/config".into(),
            body: String::new(),
            authorization: None,
        },
        "",
    );
    assert_eq!(response.status, 200);
    assert!(response.body.contains("oidc_login_enabled"));
}

#[test]
fn session_jwt_authenticates_via_handler() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    std::env::set_var("SPANDA_SESSION_JWT_SECRET", "auth-handler-test-secret");
    let issuer = SessionTokenIssuer::from_env();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let token = issuer
        .issue("operator-1", Role::Operator, "default", now)
        .expect("issue session");
    let handler = AuthHandler::new();
    let store = ApiKeyStore::new();
    let ctx = handler
        .authenticate(&store, Some(&format!("Bearer {token}")))
        .expect("session auth");
    assert_eq!(ctx.user_id.as_deref(), Some("operator-1"));
    std::env::remove_var("SPANDA_SESSION_JWT_SECRET");
}

#[test]
fn persisted_api_keys_store_hashes_only() {
    let _guard = ENV_TEST_LOCK.lock().unwrap();
    let dir = TempDir::new().expect("tempdir");
    let keys_path = dir.path().join("keys.json");
    std::env::set_var(
        "SPANDA_API_KEYS_FILE",
        keys_path.to_string_lossy().to_string(),
    );
    let mut store = ApiKeyStore::new();
    store.keys.push(ApiKeyRecord {
        key_id: "file".into(),
        token: String::new(),
        token_hash: Some(hash_api_key_token("persist-me", &api_key_pepper())),
        role: Role::Developer,
        label: None,
        tenant_id: "default".into(),
    });
    store.persist_file_keys().expect("persist");
    let raw = std::fs::read_to_string(&keys_path).expect("read keys");
    assert!(raw.contains("token_hash"));
    assert!(!raw.contains("persist-me"));
    std::env::remove_var("SPANDA_API_KEYS_FILE");
}
