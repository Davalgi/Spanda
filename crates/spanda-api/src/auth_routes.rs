//! Public authentication routes — OIDC sign-in, session info, and refresh.
//!
use crate::handlers::{bad_request, json_ok, now_ms, unauthorized};
use crate::state::ControlCenterState;
use serde::Deserialize;
use spanda_deploy_http::HttpResponse;
use spanda_security::{AuthHandler, AuthKind, RbacContext, Role, SessionTokenIssuer};

const API_VERSION: &str = "v1";

/// Route `/v1/auth/*` endpoints.
pub fn route_auth(
    state: &mut ControlCenterState,
    path: &str,
    method: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> Option<HttpResponse> {
    let auth = state.auth.clone();
    match (path, method) {
        ("/v1/auth/config", "GET") => Some(auth_config()),
        ("/v1/auth/session", "GET") => Some(auth_session(ctx)),
        ("/v1/auth/session/refresh", "POST") => Some(auth_session_refresh(body, &auth)),
        ("/v1/auth/oidc/authorize-url", "POST") => Some(auth_oidc_authorize_url(body)),
        ("/v1/auth/oidc/callback", "POST") => Some(auth_oidc_callback(state, body, &auth)),
        _ => None,
    }
}

fn auth_config() -> HttpResponse {
    let config = crate::control_center_extras::oidc_public_config();
    let read_policy = spanda_security::ReadAuthPolicy::from_env();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "oidc_login_enabled": config.enabled && config.oauth_ready,
        "issuer": config.issuer,
        "session_ttl_secs": SessionTokenIssuer::from_env().ttl_secs(),
        "read_auth": {
            "require_sensitive_reads": read_policy.require_sensitive_reads,
            "require_all_reads": read_policy.require_all_reads,
        },
    }))
}

fn auth_session(ctx: Option<&RbacContext>) -> HttpResponse {
    let Some(context) = ctx else {
        return unauthorized();
    };
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "key_id": context.key_id,
        "role": context.role,
        "tenant_id": context.tenant_id,
        "auth_kind": context.auth_kind,
        "user_id": context.user_id,
    }))
}

#[derive(Debug, Deserialize, Default)]
struct SessionRefreshRequest {
    #[serde(default)]
    token: Option<String>,
}

fn auth_session_refresh(body: &str, auth: &AuthHandler) -> HttpResponse {
    let request: SessionRefreshRequest = serde_json::from_str(body).unwrap_or_default();
    let token = request.token.unwrap_or_default();
    if token.trim().is_empty() {
        return bad_request("token required");
    }
    let now = (now_ms() / 1000.0) as u64;
    match auth.sessions.refresh(token.trim(), now) {
        Ok(session_token) => json_ok(&serde_json::json!({
            "version": API_VERSION,
            "ok": true,
            "session_token": session_token,
            "expires_in_secs": auth.sessions.ttl_secs(),
        })),
        Err(error) => bad_request(&error),
    }
}

#[derive(Debug, Deserialize, Default)]
struct OidcAuthorizeRequest {
    #[serde(default)]
    redirect_uri: Option<String>,
}

fn auth_oidc_authorize_url(body: &str) -> HttpResponse {
    let request: OidcAuthorizeRequest = serde_json::from_str(body).unwrap_or_default();
    match crate::control_center_extras::oidc_build_authorize_url(request.redirect_uri, true) {
        Ok(payload) => json_ok(&payload),
        Err(error) => bad_request(&error),
    }
}

#[derive(Debug, Deserialize)]
struct OidcCallbackRequest {
    code: String,
    #[serde(default)]
    state: Option<String>,
}

fn auth_oidc_callback(
    state: &mut ControlCenterState,
    body: &str,
    auth: &AuthHandler,
) -> HttpResponse {
    let request: OidcCallbackRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    if request.code.trim().is_empty() {
        return bad_request("code required");
    }
    let userinfo = match crate::control_center_extras::oidc_exchange_code(
        request.code.trim(),
        request.state.as_deref(),
    ) {
        Ok(info) => info,
        Err(error) => return bad_request(&error),
    };
    let Some(entry) = crate::control_center_extras::oidc_userinfo_entry(&userinfo) else {
        return bad_request("userinfo missing sub");
    };
    let config = crate::control_center_extras::oidc_public_config();
    let (created, updated) = crate::admin_users::import_oidc_directory(
        state,
        &[entry.clone()],
        &config.group_role_map,
    );
    let role = state
        .admin_user_store
        .find(&entry.user_id)
        .map(|user| Role::parse(&user.role))
        .unwrap_or(Role::Operator);
    let now = (now_ms() / 1000.0) as u64;
    let session_token = match auth
        .sessions
        .issue(&entry.user_id, role, &state.tenant_id, now)
    {
        Ok(token) => token,
        Err(error) => return bad_request(&error),
    };
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "session_token": session_token,
        "expires_in_secs": auth.sessions.ttl_secs(),
        "auth_kind": AuthKind::Session,
        "user_id": entry.user_id,
        "role": format!("{role:?}"),
        "users_created": created,
        "users_updated": updated,
    }))
}
