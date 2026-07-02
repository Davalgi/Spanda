//! Administrator user directory — operator accounts linked to RBAC roles and API keys.
//!
use crate::handlers::{bad_request, json_ok, now_ms, unauthorized};
use crate::state::ControlCenterState;
use serde::{Deserialize, Serialize};
use spanda_deploy_http::HttpResponse;
use spanda_security::{RbacContext, Role};
use std::fs;
use std::path::PathBuf;

const API_VERSION: &str = "v1";

/// One operator account in the Control Center user directory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdminUser {
    pub user_id: String,
    pub display_name: String,
    #[serde(default)]
    pub email: Option<String>,
    pub role: String,
    #[serde(default)]
    pub api_key_id: Option<String>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub created_at_ms: f64,
    pub updated_at_ms: f64,
}

fn default_enabled() -> bool {
    true
}

/// Persisted user directory store.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdminUserStore {
    pub users: Vec<AdminUser>,
}

impl AdminUserStore {
    pub fn find(&self, user_id: &str) -> Option<&AdminUser> {
        self.users.iter().find(|user| user.user_id == user_id)
    }

    pub fn find_mut(&mut self, user_id: &str) -> Option<&mut AdminUser> {
        self.users.iter_mut().find(|user| user.user_id == user_id)
    }
}

fn users_path() -> PathBuf {
    crate::persistence::default_state_dir().join("admin-users.json")
}

pub fn hydrate_admin_users(state: &mut ControlCenterState) {
    let path = users_path();
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(store) = serde_json::from_str::<AdminUserStore>(&content) {
            state.admin_user_store = store;
        }
    }
}

pub fn persist_admin_users(state: &ControlCenterState) -> Result<(), String> {
    let path = users_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(
        path,
        serde_json::to_string_pretty(&state.admin_user_store).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}

fn require_admin(ctx: Option<&RbacContext>) -> bool {
    matches!(ctx, Some(c) if c.role == Role::Administrator)
}

fn seed_users_from_config(state: &mut ControlCenterState) {
    if !state.admin_user_store.users.is_empty() {
        return;
    }
    let Some(resolved) = state.resolved.as_ref() else {
        return;
    };
    let now = now_ms();
    for human in &resolved.human_registry.humans {
        state.admin_user_store.users.push(AdminUser {
            user_id: human.id.clone(),
            display_name: human
                .display_name
                .clone()
                .unwrap_or_else(|| human.id.clone()),
            email: None,
            role: if human.role.is_empty() {
                "operator".into()
            } else {
                human.role.clone()
            },
            api_key_id: None,
            enabled: true,
            created_at_ms: now,
            updated_at_ms: now,
        });
    }
    if !state.admin_user_store.users.is_empty() {
        let _ = persist_admin_users(state);
    }
}

/// GET /v1/admin/users
pub fn admin_users_list(state: &mut ControlCenterState, ctx: Option<&RbacContext>) -> HttpResponse {
    if !require_admin(ctx) {
        return unauthorized();
    }
    seed_users_from_config(state);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "users": state.admin_user_store.users,
        "count": state.admin_user_store.users.len(),
        "persist_path": users_path().display().to_string(),
    }))
}

#[derive(Debug, Deserialize)]
struct CreateAdminUserRequest {
    user_id: String,
    display_name: String,
    #[serde(default)]
    email: Option<String>,
    role: String,
    #[serde(default)]
    api_key_id: Option<String>,
}

/// POST /v1/admin/users
pub fn admin_users_create(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !require_admin(ctx) {
        return unauthorized();
    }
    let request: CreateAdminUserRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    if request.user_id.trim().is_empty() {
        return bad_request("user_id required");
    }
    if Role::parse(&request.role) == Role::Guest {
        return bad_request("invalid role");
    }
    if state.admin_user_store.find(&request.user_id).is_some() {
        return bad_request("user_id already exists");
    }
    let now = now_ms();
    let user = AdminUser {
        user_id: request.user_id,
        display_name: request.display_name,
        email: request.email,
        role: request.role,
        api_key_id: request.api_key_id,
        enabled: true,
        created_at_ms: now,
        updated_at_ms: now,
    };
    state.admin_user_store.users.push(user.clone());
    if let Err(error) = persist_admin_users(state) {
        state.admin_user_store.users.pop();
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "user": user,
    }))
}

#[derive(Debug, Deserialize)]
struct PatchAdminUserRequest {
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    api_key_id: Option<String>,
    #[serde(default)]
    enabled: Option<bool>,
}

/// PATCH /v1/admin/users/{user_id}
pub fn admin_users_patch(
    state: &mut ControlCenterState,
    user_id: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !require_admin(ctx) {
        return unauthorized();
    }
    let request: PatchAdminUserRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    let Some(user) = state.admin_user_store.find_mut(user_id) else {
        return crate::admin_ops::not_found_response();
    };
    if let Some(name) = request.display_name {
        user.display_name = name;
    }
    if let Some(email) = request.email {
        user.email = Some(email);
    }
    if let Some(role) = request.role {
        if Role::parse(&role) == Role::Guest {
            return bad_request("invalid role");
        }
        user.role = role;
    }
    if let Some(key_id) = request.api_key_id {
        user.api_key_id = Some(key_id);
    }
    if let Some(enabled) = request.enabled {
        user.enabled = enabled;
    }
    user.updated_at_ms = now_ms();
    if let Err(error) = persist_admin_users(state) {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "user_id": user_id,
    }))
}

/// DELETE /v1/admin/users/{user_id}
pub fn admin_users_delete(
    state: &mut ControlCenterState,
    user_id: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !require_admin(ctx) {
        return unauthorized();
    }
    let before = state.admin_user_store.users.len();
    state
        .admin_user_store
        .users
        .retain(|user| user.user_id != user_id);
    if state.admin_user_store.users.len() == before {
        return crate::admin_ops::not_found_response();
    }
    if let Err(error) = persist_admin_users(state) {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "deleted": user_id,
    }))
}

pub fn admin_users_list_json(state: &mut ControlCenterState, ctx: Option<&RbacContext>) -> String {
    admin_users_list(state, ctx).body
}

pub fn route_admin_users(
    state: &mut ControlCenterState,
    path: &str,
    method: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> Option<HttpResponse> {
    if path == "/v1/admin/users" && method == "GET" {
        return Some(admin_users_list(state, ctx));
    }
    if path == "/v1/admin/users" && method == "POST" {
        return Some(admin_users_create(state, body, ctx));
    }
    let rest = path.strip_prefix("/v1/admin/users/")?;
    match method {
        "PATCH" => Some(admin_users_patch(state, rest, body, ctx)),
        "DELETE" => Some(admin_users_delete(state, rest, ctx)),
        _ => None,
    }
}
