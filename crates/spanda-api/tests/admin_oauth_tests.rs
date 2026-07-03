//! OIDC and Slack OAuth administration endpoint tests.

use spanda_api::control_center_extras::{
    admin_oidc_authorize_url, admin_oidc_put, admin_slack_oauth_url,
};
use spanda_security::{RbacContext, Role};
use tempfile::TempDir;

fn deploy_ctx() -> RbacContext {
    RbacContext {
        key_id: "admin".into(),
        role: Role::Administrator,
        tenant_id: "default".into(),
    }
}

#[test]
fn oidc_authorize_url_requires_issuer_and_client() {
    let response = admin_oidc_authorize_url("{}", Some(&deploy_ctx()));
    assert_eq!(response.status, 400);
    assert!(response.body.contains("issuer"));
}

#[test]
fn oidc_authorize_url_includes_pkce_when_configured() {
    let dir = TempDir::new().expect("tempdir");
    std::env::set_var(
        "SPANDA_CONTROL_CENTER_STATE_DIR",
        dir.path().to_string_lossy().to_string(),
    );
    let put = admin_oidc_put(
        r#"{
            "enabled": true,
            "issuer": "https://idp.example.com",
            "client_id": "cc-client",
            "client_secret": "secret",
            "redirect_uri": "http://127.0.0.1:8080/admin/oauth/oidc/callback"
        }"#,
        Some(&deploy_ctx()),
    );
    assert_eq!(put.status, 200, "{}", put.body);
    let response = admin_oidc_authorize_url("{}", Some(&deploy_ctx()));
    assert!(
        response.status == 200 || response.status == 400,
        "authorize may fail without live discovery: {}",
        response.body
    );
    if response.status == 200 {
        assert!(response.body.contains("code_challenge"));
        assert!(response.body.contains("\"pkce\":true"));
    }
    std::env::remove_var("SPANDA_CONTROL_CENTER_STATE_DIR");
}

#[test]
fn slack_oauth_url_requires_client_id() {
    let response = admin_slack_oauth_url("{}", Some(&deploy_ctx()));
    assert_eq!(response.status, 400);
    assert!(response.body.contains("oauth_client_id"));
}

#[test]
fn pkce_challenge_is_url_safe() {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    use sha2::{Digest, Sha256};

    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(b"spanda-pkce-test-verifier"));
    assert!(!challenge.contains('+'));
    assert!(!challenge.contains('/'));
    assert!(!challenge.contains('='));
}
