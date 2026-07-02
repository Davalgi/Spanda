//! Persisted alert channel configuration for Control Center administrators.
//!
use crate::handlers::{bad_request, json_ok, unauthorized};
use crate::state::ControlCenterState;
use serde::{Deserialize, Serialize};
use spanda_deploy_http::HttpResponse;
use spanda_ops::AlertChannel;
use spanda_security::{ApiKeyStore, RbacAction, RbacContext, Role};
use std::fs;
use std::path::PathBuf;

const API_VERSION: &str = "v1";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlertChannelStore {
    pub channels: Vec<AlertChannel>,
    #[serde(default)]
    pub use_env_fallback: bool,
}

fn channels_path() -> PathBuf {
    crate::persistence::default_state_dir().join("alert-channels.json")
}

pub fn hydrate_alert_channels(state: &mut ControlCenterState) {
    let path = channels_path();
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(store) = serde_json::from_str::<AlertChannelStore>(&content) {
            if !store.channels.is_empty() || !store.use_env_fallback {
                state.alert_dispatcher.channels = store.channels.clone();
                state.alert_channel_store = store;
                return;
            }
        }
    }
    state.alert_channel_store = AlertChannelStore {
        channels: state.alert_dispatcher.channels.clone(),
        use_env_fallback: true,
    };
}

pub fn persist_alert_channels(state: &ControlCenterState) -> Result<(), String> {
    let path = channels_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(
        path,
        serde_json::to_string_pretty(&state.alert_channel_store).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}

fn require_admin(ctx: Option<&RbacContext>) -> bool {
    matches!(ctx, Some(c) if c.role == Role::Administrator)
}

/// GET /v1/admin/alert-channels
pub fn admin_alert_channels_get(
    state: &ControlCenterState,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Deploy) && !require_admin(ctx) {
        return unauthorized();
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "channels": state.alert_dispatcher.channels,
        "use_env_fallback": state.alert_channel_store.use_env_fallback,
        "persist_path": channels_path().display().to_string(),
    }))
}

#[derive(Debug, Deserialize)]
struct UpdateAlertChannelsRequest {
    channels: Vec<AlertChannel>,
    #[serde(default)]
    use_env_fallback: bool,
}

/// PUT /v1/admin/alert-channels
pub fn admin_alert_channels_put(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !require_admin(ctx) {
        return unauthorized();
    }
    let request: UpdateAlertChannelsRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    if request.channels.is_empty() {
        return bad_request("at least one channel required (use log for local-only)");
    }
    state.alert_dispatcher.channels = request.channels.clone();
    state.alert_channel_store = AlertChannelStore {
        channels: request.channels,
        use_env_fallback: request.use_env_fallback,
    };
    if let Err(error) = persist_alert_channels(state) {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "channel_count": state.alert_dispatcher.channels.len(),
    }))
}

pub fn admin_alert_channels_get_json(
    state: &ControlCenterState,
    ctx: Option<&RbacContext>,
) -> String {
    admin_alert_channels_get(state, ctx).body
}

pub fn admin_alert_channels_put_json(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    admin_alert_channels_put(state, body, ctx).body
}

pub fn route_alert_channels(
    state: &mut ControlCenterState,
    path: &str,
    method: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> Option<HttpResponse> {
    if path == "/v1/admin/alert-channels" && method == "GET" {
        return Some(admin_alert_channels_get(state, ctx));
    }
    if path == "/v1/admin/alert-channels" && method == "PUT" {
        return Some(admin_alert_channels_put(state, body, ctx));
    }
    None
}
