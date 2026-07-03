//! Control Center enhancement APIs — fleet map, config history, admin OIDC/Slack, chaos, deploy gate.
//!
use crate::handlers::{bad_request, json_ok, now_ms, unauthorized};
use crate::state::ControlCenterState;
use serde::{Deserialize, Serialize};
use spanda_chaos::{default_injections, run_chaos_experiment, ChaosExperimentOptions};
use spanda_config::{default_snapshots_dir, list_config_snapshots};
use spanda_deploy_http::HttpResponse;
use spanda_fleet::remote::{default_fleet_agents_path, load_fleet_agent_registry};
use spanda_readiness::{
    evaluate_deployment_gates, DeploymentGatePolicy, ReadinessOptions,
};
use spanda_security::{ApiKeyStore, RbacAction, RbacContext, Role};
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

const API_VERSION: &str = "v1";

/// Supported chaos injection labels for catalog responses.
fn supported_injection_catalog() -> Vec<&'static str> {
    vec![
        "gps-failure",
        "camera-failure",
        "lidar-failure",
        "connectivity-failure",
        "provider-failure",
        "package-failure",
        "battery-failure",
    ]
}

fn hash_to_grid(id: &str) -> (f64, f64) {
    // Map stable ids without coordinates onto a 0–100 grid.
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    id.hash(&mut hasher);
    let hash = hasher.finish();
    let x = (hash % 101) as f64;
    let y = ((hash / 101) % 101) as f64;
    (x, y)
}

fn parse_lat_lng_pair(lat: &str, lng: &str) -> Option<(f64, f64)> {
    let lat = lat.parse::<f64>().ok()?;
    let lng = lng.parse::<f64>().ok()?;
    Some((lng, lat))
}

fn coords_from_metadata(metadata: &HashMap<String, String>) -> Option<(f64, f64)> {
    // Prefer explicit lat/lng metadata keys when present.
    let lat = metadata
        .get("lat")
        .or_else(|| metadata.get("latitude"))
        .map(String::as_str)?;
    let lng = metadata
        .get("lng")
        .or_else(|| metadata.get("longitude"))
        .or_else(|| metadata.get("lon"))
        .map(String::as_str)?;
    parse_lat_lng_pair(lat, lng)
}

fn coords_from_location(
    location: &spanda_config::EntityLocation,
) -> Option<(f64, f64)> {
    let coords = location.coordinates.as_ref()?;
    if let Some(table) = coords.as_table() {
        let lat = table
            .get("lat")
            .or_else(|| table.get("latitude"))
            .and_then(|value| value.as_float().or_else(|| value.as_integer().map(|n| n as f64)))?;
        let lng = table
            .get("lng")
            .or_else(|| table.get("longitude"))
            .or_else(|| table.get("lon"))
            .and_then(|value| value.as_float().or_else(|| value.as_integer().map(|n| n as f64)))?;
        return Some((lng, lat));
    }
    if let Some(array) = coords.as_array() {
        if array.len() >= 2 {
            let lng = array[0]
                .as_float()
                .or_else(|| array[0].as_integer().map(|n| n as f64))?;
            let lat = array[1]
                .as_float()
                .or_else(|| array[1].as_integer().map(|n| n as f64))?;
            return Some((lng, lat));
        }
    }
    None
}

fn push_marker(
    markers: &mut Vec<serde_json::Value>,
    id: &str,
    label: &str,
    kind: &str,
    coords: Option<(f64, f64)>,
    status: Option<&str>,
    fleet_id: Option<&str>,
) {
    let (x, y) = coords.unwrap_or_else(|| hash_to_grid(id));
    let mut marker = serde_json::json!({
        "id": id,
        "label": label,
        "kind": kind,
        "x": x,
        "y": y,
    });
    if let Some(status) = status {
        marker["status"] = serde_json::json!(status);
    }
    if let Some(fleet_id) = fleet_id {
        marker["fleet_id"] = serde_json::json!(fleet_id);
    }
    markers.push(marker);
}

/// GET /v1/fleet/map — fleet markers from robots, agents, entities, and devices.
pub fn fleet_map_json(state: &ControlCenterState) -> HttpResponse {
    // Build map markers from configured fleet resources and entity locations.
    //
    // Parameters:
    // - `state` — Control Center runtime state
    //
    // Returns:
    // HTTP 200 JSON with version and marker list.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = fleet_map_json(&state);

    let mut markers = Vec::new();
    let fleet_id = state.resolved.as_ref().and_then(|resolved| resolved.fleet_id());

    if let Some(resolved) = state.resolved.as_ref() {
        if let Some(fleet) = resolved.device_tree.fleet.as_ref() {
            for robot in &fleet.robots {
                push_marker(
                    &mut markers,
                    &robot.id,
                    &robot.id,
                    "robot",
                    None,
                    Some("configured"),
                    fleet_id.as_deref(),
                );
            }
        }
    }

    let agents = load_fleet_agent_registry(&default_fleet_agents_path());
    for agent in &agents.agents {
        push_marker(
            &mut markers,
            &agent.robot_name,
            &agent.robot_name,
            "fleet_agent",
            None,
            Some("registered"),
            fleet_id.as_deref(),
        );
    }

    let registry = state.entity_registry();
    for entity in registry.list() {
        let label = entity
            .display_name
            .as_deref()
            .or(entity.name.as_deref())
            .unwrap_or(&entity.id);
        let coords = entity
            .location
            .as_ref()
            .and_then(coords_from_location)
            .or_else(|| coords_from_metadata(&entity.metadata));
        push_marker(
            &mut markers,
            &entity.id,
            label,
            "entity",
            coords,
            Some(entity.health_status.as_str()),
            fleet_id.as_deref(),
        );
    }

    for device in state.device_registry().pool_entries() {
        push_marker(
            &mut markers,
            &device.id,
            device.logical_name.as_deref().unwrap_or(&device.id),
            "device",
            None,
            Some(device.lifecycle_state.as_str()),
            fleet_id.as_deref(),
        );
    }

    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "markers": markers,
    }))
}

/// GET /v1/config/history — configuration snapshots plus config-related audit mutations.
pub fn config_history_json(_state: &ControlCenterState) -> HttpResponse {
    // Merge snapshot metadata with config-related mutation audit events.
    //
    // Parameters:
    // - `state` — Control Center runtime state
    //
    // Returns:
    // HTTP 200 JSON history timeline.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = config_history_json(&state);

    let mut history = Vec::new();
    let dir = default_snapshots_dir();
    let snapshots = list_config_snapshots(&dir).unwrap_or_default();
    for snapshot in snapshots {
        history.push(serde_json::json!({
            "id": format!("snapshot-{}", snapshot.id),
            "timestamp": snapshot.created_at_ms,
            "action": "snapshot",
            "snapshot_id": snapshot.id,
        }));
    }

    let audit_path = crate::audit_log::default_mutation_audit_path();
    if let Ok(lines) = crate::audit_log::read_mutation_audit_lines(&audit_path) {
        for line in lines {
            let payload_raw = line
                .get("payload")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let payload: serde_json::Value =
                serde_json::from_str(payload_raw).unwrap_or_else(|_| serde_json::json!({}));
            let path = payload
                .get("path")
                .and_then(|value| value.as_str())
                .unwrap_or("");
            let haystack = format!("{path} {payload_raw}").to_ascii_lowercase();
            if !haystack.contains("config") && !haystack.contains("snapshot") {
                continue;
            }
            let timestamp = line
                .get("timestamp_ms")
                .and_then(|value| value.as_f64())
                .unwrap_or_else(now_ms);
            let action = payload
                .get("method")
                .and_then(|value| value.as_str())
                .unwrap_or("mutation");
            let snapshot_id = payload
                .get("snapshot_id")
                .and_then(|value| value.as_str())
                .map(str::to_string);
            let actor = payload
                .get("actor_key_id")
                .and_then(|value| value.as_str())
                .map(str::to_string);
            let id = line
                .get("id")
                .and_then(|value| value.as_str())
                .unwrap_or("audit");
            let mut entry = serde_json::json!({
                "id": id,
                "timestamp": timestamp,
                "action": action,
            });
            if let Some(snapshot_id) = snapshot_id {
                entry["snapshot_id"] = serde_json::json!(snapshot_id);
            }
            if let Some(actor) = actor {
                entry["actor"] = serde_json::json!(actor);
            }
            history.push(entry);
        }
    }

    history.sort_by(|left, right| {
        let left_ts = left.get("timestamp").and_then(|value| value.as_f64()).unwrap_or(0.0);
        let right_ts = right
            .get("timestamp")
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0);
        right_ts
            .partial_cmp(&left_ts)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "history": history,
    }))
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct AdminOidcConfig {
    #[serde(default)]
    enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    issuer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    client_secret: Option<String>,
    #[serde(default)]
    client_secret_set: bool,
    #[serde(default)]
    group_role_map: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_sync_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    oauth_state: Option<String>,
    #[serde(default, skip_serializing)]
    oauth_code_verifier: Option<String>,
}

fn admin_oidc_path() -> PathBuf {
    crate::persistence::default_state_dir().join("admin-oidc.json")
}

fn load_admin_oidc() -> AdminOidcConfig {
    let path = admin_oidc_path();
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(config) = serde_json::from_str::<AdminOidcConfig>(&content) {
            return config;
        }
    }
    AdminOidcConfig::default()
}

fn persist_admin_oidc(config: &AdminOidcConfig) -> Result<(), String> {
    let path = admin_oidc_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(
        path,
        serde_json::to_string_pretty(config).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}

fn require_admin(ctx: Option<&RbacContext>) -> bool {
    matches!(ctx, Some(c) if c.role == Role::Administrator)
}

fn valid_issuer_url(issuer: &str) -> bool {
    issuer.starts_with("https://")
        && issuer.len() > 8
        && !issuer.contains(' ')
        && issuer.chars().any(|ch| ch != '/')
}

fn url_encode_component(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

fn fresh_oauth_state() -> String {
    format!("spanda-{:x}", now_ms() as u128)
}

fn pkce_verifier() -> String {
    format!(
        "spanda-pkce-{:x}-{:x}",
        now_ms() as u128,
        std::process::id() as u128
    )
}

fn pkce_challenge(verifier: &str) -> String {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    use sha2::{Digest, Sha256};

    // Hash the verifier with SHA-256 for the S256 PKCE challenge.
    let digest = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}

fn fetch_oidc_discovery(issuer: &str) -> Option<serde_json::Value> {
    let discovery_url = format!(
        "{}/.well-known/openid-configuration",
        issuer.trim_end_matches('/')
    );
    let response = ureq::get(&discovery_url).call().ok()?;
    let body = response.into_string().ok()?;
    serde_json::from_str(&body).ok()
}

fn userinfo_to_directory_entry(userinfo: &serde_json::Value) -> Option<crate::admin_users::OidcDirectoryEntry> {
    let user_id = userinfo
        .get("sub")
        .or_else(|| userinfo.get("user_id"))
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())?;
    let display_name = userinfo
        .get("name")
        .or_else(|| userinfo.get("preferred_username"))
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let email = userinfo
        .get("email")
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let groups = userinfo
        .get("groups")
        .and_then(|value| value.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str().map(str::to_string))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Some(crate::admin_users::OidcDirectoryEntry {
        user_id: user_id.to_string(),
        display_name,
        email,
        role: None,
        groups,
    })
}

/// GET /v1/admin/oidc — OIDC integration configuration (no secrets).
pub fn admin_oidc_get(ctx: Option<&RbacContext>) -> HttpResponse {
    // Return persisted OIDC settings for the admin console.
    //
    // Parameters:
    // - `ctx` — optional RBAC context
    //
    // Returns:
    // HTTP 200 JSON config summary.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = admin_oidc_get(ctx.as_ref());

    if !require_admin(ctx) {
        return unauthorized();
    }
    let config = load_admin_oidc();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "enabled": config.enabled,
        "issuer": config.issuer,
        "client_id": config.client_id,
        "client_secret_set": config.client_secret_set,
        "group_role_map": config.group_role_map,
        "last_sync_at": config.last_sync_at,
        "redirect_uri": config.redirect_uri,
        "oauth_ready": config.client_id.is_some() && config.client_secret_set,
        "persist_path": admin_oidc_path().display().to_string(),
    }))
}

#[derive(Debug, Deserialize)]
struct AdminOidcPutRequest {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    issuer: Option<String>,
    #[serde(default)]
    client_id: Option<String>,
    #[serde(default)]
    client_secret: Option<String>,
    #[serde(default)]
    group_role_map: HashMap<String, String>,
    #[serde(default)]
    redirect_uri: Option<String>,
}

/// PUT /v1/admin/oidc — update OIDC integration settings.
pub fn admin_oidc_put(body: &str, ctx: Option<&RbacContext>) -> HttpResponse {
    // Persist OIDC settings from the admin console.
    //
    // Parameters:
    // - `body` — JSON request body
    // - `ctx` — optional RBAC context
    //
    // Returns:
    // HTTP 200 on success.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = admin_oidc_put(body, ctx.as_ref());

    if !ApiKeyStore::check(ctx, RbacAction::Deploy) {
        return unauthorized();
    }
    let request: AdminOidcPutRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    if let Some(issuer) = request.issuer.as_deref() {
        if !issuer.is_empty() && !valid_issuer_url(issuer) {
            return bad_request("issuer must be a valid https URL");
        }
    }
    let mut config = load_admin_oidc();
    config.enabled = request.enabled;
    config.issuer = request.issuer;
    config.client_id = request.client_id;
    config.group_role_map = request.group_role_map;
    if let Some(redirect_uri) = request.redirect_uri {
        if !redirect_uri.is_empty() {
            config.redirect_uri = Some(redirect_uri);
        }
    }
    if let Some(secret) = request.client_secret {
        if !secret.is_empty() {
            config.client_secret = Some(secret);
            config.client_secret_set = true;
        }
    }
    if let Err(error) = persist_admin_oidc(&config) {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "client_secret_set": config.client_secret_set,
    }))
}

/// POST /v1/admin/oidc/sync — fetch OIDC discovery and import directory users.
pub fn admin_oidc_sync(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    // Fetch OpenID discovery metadata and upsert users from the request directory payload.
    //
    // Parameters:
    // - `state` — mutable Control Center state (user directory)
    // - `body` — optional JSON `{ "directory": [...] }`
    // - `ctx` — optional RBAC context
    //
    // Returns:
    // HTTP 200 with import counts on success.
    //
    // Options:
    // Set `SPANDA_OIDC_DIRECTORY_JSON` to a file path for default directory entries.
    //
    // Example:
    // let response = admin_oidc_sync(state, body, ctx.as_ref());

    if !ApiKeyStore::check(ctx, RbacAction::Deploy) {
        return unauthorized();
    }
    let mut config = load_admin_oidc();
    let Some(issuer) = config.issuer.as_deref() else {
        return bad_request("issuer not configured");
    };
    if !valid_issuer_url(issuer) {
        return bad_request("issuer must be a valid https URL");
    }
    let discovery_url = format!(
        "{}/.well-known/openid-configuration",
        issuer.trim_end_matches('/')
    );
    let mut discovery: serde_json::Value = serde_json::json!({});
    if let Ok(response) = ureq::get(&discovery_url).call() {
        if let Ok(body) = response.into_string() {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&body) {
                discovery = parsed;
            }
        }
    }
    #[derive(Debug, Deserialize, Default)]
    struct OidcSyncRequest {
        #[serde(default)]
        directory: Vec<crate::admin_users::OidcDirectoryEntry>,
    }
    let mut request: OidcSyncRequest = serde_json::from_str(body).unwrap_or_default();
    if request.directory.is_empty() {
        if let Ok(path) = std::env::var("SPANDA_OIDC_DIRECTORY_JSON") {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(entries) =
                    serde_json::from_str::<Vec<crate::admin_users::OidcDirectoryEntry>>(&content)
                {
                    request.directory = entries;
                }
            }
        }
    }
    let (created, updated) = if request.directory.is_empty() {
        (0usize, 0usize)
    } else {
        crate::admin_users::import_oidc_directory(state, &request.directory, &config.group_role_map)
    };
    config.last_sync_at = Some(now_ms());
    if let Err(error) = persist_admin_oidc(&config) {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "last_sync_at": config.last_sync_at,
        "discovery": discovery,
        "users_created": created,
        "users_updated": updated,
    }))
}

#[derive(Debug, Deserialize, Default)]
struct AdminOidcAuthorizeRequest {
    #[serde(default)]
    redirect_uri: Option<String>,
}

/// POST /v1/admin/oidc/authorize-url — build OIDC authorization URL for admin OAuth.
pub fn admin_oidc_authorize_url(body: &str, ctx: Option<&RbacContext>) -> HttpResponse {
    // Generate an OAuth authorization URL from OpenID discovery metadata.
    //
    // Parameters:
    // - `body` — optional JSON `{ "redirect_uri": "..." }`
    // - `ctx` — optional RBAC context
    //
    // Returns:
    // HTTP 200 with `authorize_url` and `state` on success.
    //
    // Options:
    // `SPANDA_OIDC_REDIRECT_URI` overrides the default redirect URI.
    //
    // Example:
    // let response = admin_oidc_authorize_url(body, ctx.as_ref());

    if !ApiKeyStore::check(ctx, RbacAction::Deploy) {
        return unauthorized();
    }
    let request: AdminOidcAuthorizeRequest =
        serde_json::from_str(body).unwrap_or_default();
    let mut config = load_admin_oidc();
    let Some(issuer) = config.issuer.as_deref() else {
        return bad_request("issuer not configured");
    };
    let Some(client_id) = config.client_id.as_deref() else {
        return bad_request("client_id not configured");
    };
    let discovery = fetch_oidc_discovery(issuer).unwrap_or_else(|| serde_json::json!({}));
    let authorization_endpoint = discovery
        .get("authorization_endpoint")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    if authorization_endpoint.is_empty() {
        return bad_request("authorization_endpoint missing from discovery document");
    }
    let redirect_uri = request
        .redirect_uri
        .or_else(|| config.redirect_uri.clone())
        .or_else(|| std::env::var("SPANDA_OIDC_REDIRECT_URI").ok())
        .unwrap_or_else(|| "http://127.0.0.1:8080/admin/oauth/oidc/callback".into());
    let state = fresh_oauth_state();
    let verifier = pkce_verifier();
    let challenge = pkce_challenge(&verifier);
    config.oauth_state = Some(state.clone());
    config.oauth_code_verifier = Some(verifier);
    config.redirect_uri = Some(redirect_uri.clone());
    if let Err(error) = persist_admin_oidc(&config) {
        return bad_request(&error);
    }
    let authorize_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        authorization_endpoint,
        url_encode_component(client_id),
        url_encode_component(&redirect_uri),
        url_encode_component("openid profile email"),
        url_encode_component(&state),
        url_encode_component(&challenge),
    );
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "authorize_url": authorize_url,
        "state": state,
        "redirect_uri": redirect_uri,
        "pkce": true,
    }))
}

#[derive(Debug, Deserialize)]
struct AdminOidcOAuthCallbackRequest {
    code: String,
    #[serde(default)]
    state: Option<String>,
}

/// POST /v1/admin/oidc/oauth/callback — exchange authorization code and import userinfo.
pub fn admin_oidc_oauth_callback(
    state: &mut ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    // Exchange an OAuth authorization code for tokens and import the user profile.
    //
    // Parameters:
    // - `state` — mutable Control Center state (user directory)
    // - `body` — JSON `{ "code": "...", "state": "..." }`
    // - `ctx` — optional RBAC context
    //
    // Returns:
    // HTTP 200 with import counts on success.
    //
    // Options:
    // Requires `client_secret` saved via PUT /v1/admin/oidc.
    //
    // Example:
    // let response = admin_oidc_oauth_callback(state, body, ctx.as_ref());

    if !ApiKeyStore::check(ctx, RbacAction::Deploy) {
        return unauthorized();
    }
    let request: AdminOidcOAuthCallbackRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    if request.code.trim().is_empty() {
        return bad_request("code required");
    }
    let mut config = load_admin_oidc();
    if let Some(expected) = config.oauth_state.as_deref() {
        if request.state.as_deref() != Some(expected) {
            return bad_request("oauth state mismatch");
        }
    }
    let Some(issuer) = config.issuer.as_deref() else {
        return bad_request("issuer not configured");
    };
    let Some(client_id) = config.client_id.as_deref() else {
        return bad_request("client_id not configured");
    };
    let Some(client_secret) = config.client_secret.as_deref() else {
        return bad_request("client_secret not configured");
    };
    let redirect_uri = config
        .redirect_uri
        .clone()
        .or_else(|| std::env::var("SPANDA_OIDC_REDIRECT_URI").ok())
        .unwrap_or_else(|| "http://127.0.0.1:8080/admin/oauth/oidc/callback".into());
    let discovery = fetch_oidc_discovery(issuer).unwrap_or_else(|| serde_json::json!({}));
    let token_endpoint = discovery
        .get("token_endpoint")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    if token_endpoint.is_empty() {
        return bad_request("token_endpoint missing from discovery document");
    }
    let form = {
        let mut body = format!(
            "grant_type=authorization_code&code={}&redirect_uri={}&client_id={}&client_secret={}",
            url_encode_component(request.code.trim()),
            url_encode_component(&redirect_uri),
            url_encode_component(client_id),
            url_encode_component(client_secret),
        );
        if let Some(verifier) = config.oauth_code_verifier.as_deref() {
            body.push_str("&code_verifier=");
            body.push_str(&url_encode_component(verifier));
        }
        body
    };
    let token_response = match ureq::post(token_endpoint)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&form)
    {
        Ok(response) => response.into_string().unwrap_or_default(),
        Err(error) => return bad_request(&format!("token exchange failed: {error}")),
    };
    let token_json: serde_json::Value = serde_json::from_str(&token_response)
        .unwrap_or_else(|_| serde_json::json!({ "raw": token_response }));
    let access_token = token_json
        .get("access_token")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    if access_token.is_empty() {
        return bad_request("access_token missing from token response");
    }
    let userinfo_endpoint = discovery
        .get("userinfo_endpoint")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    let userinfo = if userinfo_endpoint.is_empty() {
        serde_json::json!({})
    } else {
        match ureq::get(userinfo_endpoint)
            .set("Authorization", &format!("Bearer {access_token}"))
            .call()
        {
            Ok(response) => response
                .into_string()
                .ok()
                .and_then(|body| serde_json::from_str(&body).ok())
                .unwrap_or_else(|| serde_json::json!({})),
            Err(_) => serde_json::json!({}),
        }
    };
    let mut created = 0usize;
    let mut updated = 0usize;
    if let Some(entry) = userinfo_to_directory_entry(&userinfo) {
        let counts =
            crate::admin_users::import_oidc_directory(state, &[entry], &config.group_role_map);
        created = counts.0;
        updated = counts.1;
    }
    config.oauth_state = None;
    config.oauth_code_verifier = None;
    config.last_sync_at = Some(now_ms());
    if let Err(error) = persist_admin_oidc(&config) {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "users_created": created,
        "users_updated": updated,
        "userinfo": userinfo,
        "last_sync_at": config.last_sync_at,
    }))
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct AdminSlackConfig {
    #[serde(default)]
    configured: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    team_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    webhook_url: Option<String>,
    #[serde(default)]
    webhook_url_set: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    oauth_client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    oauth_client_secret: Option<String>,
    #[serde(default)]
    oauth_client_secret_set: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    oauth_redirect_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    oauth_state: Option<String>,
}

fn admin_slack_path() -> PathBuf {
    crate::persistence::default_state_dir().join("admin-slack.json")
}

fn load_admin_slack() -> AdminSlackConfig {
    let path = admin_slack_path();
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(config) = serde_json::from_str::<AdminSlackConfig>(&content) {
            return config;
        }
    }
    AdminSlackConfig::default()
}

fn persist_admin_slack(config: &AdminSlackConfig) -> Result<(), String> {
    let path = admin_slack_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(
        path,
        serde_json::to_string_pretty(config).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}

/// GET /v1/admin/slack — Slack OAuth wizard status.
pub fn admin_slack_get(ctx: Option<&RbacContext>) -> HttpResponse {
    // Return Slack integration status for the setup wizard.
    //
    // Parameters:
    // - `ctx` — optional RBAC context
    //
    // Returns:
    // HTTP 200 JSON status.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = admin_slack_get(ctx.as_ref());

    if !require_admin(ctx) {
        return unauthorized();
    }
    let config = load_admin_slack();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "configured": config.configured,
        "team_name": config.team_name,
        "webhook_url_set": config.webhook_url_set,
        "oauth_client_id": config.oauth_client_id,
        "oauth_client_secret_set": config.oauth_client_secret_set,
        "oauth_redirect_uri": config.oauth_redirect_uri,
        "persist_path": admin_slack_path().display().to_string(),
    }))
}

#[derive(Debug, Deserialize)]
struct AdminSlackPostRequest {
    #[serde(default)]
    webhook_url: Option<String>,
    #[serde(default)]
    oauth_client_id: Option<String>,
    #[serde(default)]
    oauth_client_secret: Option<String>,
    #[serde(default)]
    team_name: Option<String>,
    #[serde(default)]
    oauth_redirect_uri: Option<String>,
}

/// POST /v1/admin/slack — configure Slack OAuth wizard fields.
pub fn admin_slack_post(body: &str, ctx: Option<&RbacContext>) -> HttpResponse {
    // Persist Slack webhook and OAuth client settings.
    //
    // Parameters:
    // - `body` — JSON request body
    // - `ctx` — optional RBAC context
    //
    // Returns:
    // HTTP 200 on success.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = admin_slack_post(body, ctx.as_ref());

    if !require_admin(ctx) {
        return unauthorized();
    }
    let request: AdminSlackPostRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    let mut config = load_admin_slack();
    if let Some(webhook_url) = request.webhook_url {
        if !webhook_url.is_empty() {
            config.webhook_url = Some(webhook_url);
            config.webhook_url_set = true;
        }
    }
    if let Some(oauth_client_id) = request.oauth_client_id {
        if !oauth_client_id.is_empty() {
            config.oauth_client_id = Some(oauth_client_id);
        }
    }
    if let Some(oauth_client_secret) = request.oauth_client_secret {
        if !oauth_client_secret.is_empty() {
            config.oauth_client_secret = Some(oauth_client_secret);
            config.oauth_client_secret_set = true;
        }
    }
    if let Some(oauth_redirect_uri) = request.oauth_redirect_uri {
        if !oauth_redirect_uri.is_empty() {
            config.oauth_redirect_uri = Some(oauth_redirect_uri);
        }
    }
    if let Some(team_name) = request.team_name {
        if !team_name.is_empty() {
            config.team_name = Some(team_name);
        }
    }
    config.configured = config.webhook_url_set || config.oauth_client_id.is_some();
    if let Err(error) = persist_admin_slack(&config) {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "configured": config.configured,
        "webhook_url_set": config.webhook_url_set,
        "oauth_client_id": config.oauth_client_id,
        "oauth_client_secret_set": config.oauth_client_secret_set,
    }))
}

#[derive(Debug, Deserialize, Default)]
struct AdminSlackOAuthUrlRequest {
    #[serde(default)]
    redirect_uri: Option<String>,
}

/// POST /v1/admin/slack/oauth-url — build Slack app OAuth authorization URL.
pub fn admin_slack_oauth_url(body: &str, ctx: Option<&RbacContext>) -> HttpResponse {
    // Generate a Slack OAuth v2 authorization URL for the setup wizard.
    //
    // Parameters:
    // - `body` — optional JSON `{ "redirect_uri": "..." }`
    // - `ctx` — optional RBAC context
    //
    // Returns:
    // HTTP 200 with `authorize_url` and `state`.
    //
    // Options:
    // `SPANDA_SLACK_OAUTH_REDIRECT_URI` overrides the default redirect URI.
    //
    // Example:
    // let response = admin_slack_oauth_url(body, ctx.as_ref());

    if !require_admin(ctx) {
        return unauthorized();
    }
    let request: AdminSlackOAuthUrlRequest = serde_json::from_str(body).unwrap_or_default();
    let mut config = load_admin_slack();
    let Some(client_id) = config.oauth_client_id.as_deref() else {
        return bad_request("oauth_client_id not configured");
    };
    let redirect_uri = request
        .redirect_uri
        .or_else(|| config.oauth_redirect_uri.clone())
        .or_else(|| std::env::var("SPANDA_SLACK_OAUTH_REDIRECT_URI").ok())
        .unwrap_or_else(|| "http://127.0.0.1:8080/admin/oauth/slack/callback".into());
    let state = fresh_oauth_state();
    config.oauth_state = Some(state.clone());
    config.oauth_redirect_uri = Some(redirect_uri.clone());
    if let Err(error) = persist_admin_slack(&config) {
        return bad_request(&error);
    }
    let authorize_url = format!(
        "https://slack.com/oauth/v2/authorize?client_id={}&scope={}&redirect_uri={}&state={}",
        url_encode_component(client_id),
        url_encode_component("incoming-webhook,chat:write"),
        url_encode_component(&redirect_uri),
        url_encode_component(&state),
    );
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "authorize_url": authorize_url,
        "state": state,
        "redirect_uri": redirect_uri,
    }))
}

#[derive(Debug, Deserialize)]
struct AdminSlackOAuthCallbackRequest {
    code: String,
    #[serde(default)]
    state: Option<String>,
}

/// POST /v1/admin/slack/oauth/callback — exchange Slack OAuth code and store webhook.
pub fn admin_slack_oauth_callback(body: &str, ctx: Option<&RbacContext>) -> HttpResponse {
    // Exchange a Slack OAuth authorization code for team and webhook metadata.
    //
    // Parameters:
    // - `body` — JSON `{ "code": "...", "state": "..." }`
    // - `ctx` — optional RBAC context
    //
    // Returns:
    // HTTP 200 with Slack team and webhook status on success.
    //
    // Options:
    // Requires OAuth client id/secret saved via POST /v1/admin/slack.
    //
    // Example:
    // let response = admin_slack_oauth_callback(body, ctx.as_ref());

    if !require_admin(ctx) {
        return unauthorized();
    }
    let request: AdminSlackOAuthCallbackRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    if request.code.trim().is_empty() {
        return bad_request("code required");
    }
    let mut config = load_admin_slack();
    if let Some(expected) = config.oauth_state.as_deref() {
        if request.state.as_deref() != Some(expected) {
            return bad_request("oauth state mismatch");
        }
    }
    let Some(client_id) = config.oauth_client_id.as_deref() else {
        return bad_request("oauth_client_id not configured");
    };
    let Some(client_secret) = config.oauth_client_secret.as_deref() else {
        return bad_request("oauth_client_secret not configured");
    };
    let redirect_uri = config
        .oauth_redirect_uri
        .clone()
        .or_else(|| std::env::var("SPANDA_SLACK_OAUTH_REDIRECT_URI").ok())
        .unwrap_or_else(|| "http://127.0.0.1:8080/admin/oauth/slack/callback".into());
    let form = format!(
        "code={}&client_id={}&client_secret={}&redirect_uri={}",
        url_encode_component(request.code.trim()),
        url_encode_component(client_id),
        url_encode_component(client_secret),
        url_encode_component(&redirect_uri),
    );
    let response_body = match ureq::post("https://slack.com/api/oauth.v2.access")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&form)
    {
        Ok(response) => response.into_string().unwrap_or_default(),
        Err(error) => return bad_request(&format!("slack oauth failed: {error}")),
    };
    let parsed: serde_json::Value =
        serde_json::from_str(&response_body).unwrap_or_else(|_| serde_json::json!({ "raw": response_body }));
    if parsed.get("ok").and_then(|value| value.as_bool()) != Some(true) {
        return bad_request(
            parsed
                .get("error")
                .and_then(|value| value.as_str())
                .unwrap_or("slack oauth rejected"),
        );
    }
    if let Some(team_name) = parsed
        .get("team")
        .and_then(|team| team.get("name"))
        .and_then(|value| value.as_str())
    {
        config.team_name = Some(team_name.to_string());
    }
    if let Some(webhook_url) = parsed
        .get("incoming_webhook")
        .and_then(|hook| hook.get("url"))
        .and_then(|value| value.as_str())
    {
        config.webhook_url = Some(webhook_url.to_string());
        config.webhook_url_set = true;
    }
    config.oauth_state = None;
    config.configured = true;
    if let Err(error) = persist_admin_slack(&config) {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "configured": config.configured,
        "team_name": config.team_name,
        "webhook_url_set": config.webhook_url_set,
    }))
}

/// GET /v1/chaos/injections — supported chaos injection catalog.
pub fn chaos_catalog_json() -> HttpResponse {
    // Return the supported chaos injection labels.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // HTTP 200 JSON injection catalog.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = chaos_catalog_json();

    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "injections": supported_injection_catalog(),
    }))
}

/// POST /v1/chaos/simulate — run chaos experiment when a program is loaded.
pub fn chaos_simulate_json(state: &ControlCenterState, body: &str) -> HttpResponse {
    // Run chaos injections against the loaded program or return catalog guidance.
    //
    // Parameters:
    // - `state` — Control Center runtime state
    // - `body` — JSON body with optional `injections` array
    //
    // Returns:
    // HTTP 200 JSON chaos report or catalog fallback.
    //
    // Options:
    // Request body `injections`.
    //
    // Example:
    // let response = chaos_simulate_json(&state, body);

    let payload: serde_json::Value = serde_json::from_str(body).unwrap_or_else(|_| serde_json::json!({}));
    let requested: Vec<String> = payload
        .get("injections")
        .and_then(|value| value.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    let Some(program_path) = state.program_path.as_ref() else {
        return json_ok(&serde_json::json!({
            "version": API_VERSION,
            "ok": false,
            "message": "no program loaded; pass --program to Control Center to run chaos experiments",
            "injections": supported_injection_catalog(),
        }));
    };

    let (program, _source, label) = match crate::program::parse_program_file(program_path) {
        Ok(parsed) => parsed,
        Err(error) => return bad_request(&error),
    };

    let options = ChaosExperimentOptions {
        injections: if requested.is_empty() {
            default_injections(&program)
        } else {
            requested
        },
    };
    let report = run_chaos_experiment(&program, &label, &options);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ok": true,
        "report": report,
    }))
}

/// GET /v1/deploy/gate — deployment readiness summary and checklist.
pub fn deploy_gate_json(state: &ControlCenterState) -> HttpResponse {
    // Summarize deploy readiness from dashboard metrics and deployment gates.
    //
    // Parameters:
    // - `state` — Control Center runtime state
    //
    // Returns:
    // HTTP 200 JSON deploy gate summary.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = deploy_gate_json(&state);

    let pool = state.device_registry().pool_summary();
    let readiness = spanda_config::readiness_impact(&state.device_registry(), now_ms());
    let fleet = load_fleet_agent_registry(&default_fleet_agents_path());
    let mut checklist = vec![
        serde_json::json!({
            "name": "config_loaded",
            "passed": state.resolved.is_some(),
            "message": if state.resolved.is_some() { "configuration resolved" } else { "no --config loaded" },
        }),
        serde_json::json!({
            "name": "mission_ready",
            "passed": readiness.blocked_count == 0,
            "message": format!("blocked_devices={}", readiness.blocked_count),
        }),
        serde_json::json!({
            "name": "device_pool_healthy",
            "passed": pool.failed == 0,
            "message": format!("failed={}, degraded={}", pool.failed, pool.degraded),
        }),
        serde_json::json!({
            "name": "fleet_agents_registered",
            "passed": !fleet.agents.is_empty() || state.resolved.is_none(),
            "message": format!("agents={}", fleet.agents.len()),
        }),
    ];

    let mut gate_report = None;
    if let Some(program_path) = state.program_path.as_ref() {
        if let Ok((program, source, _label)) = crate::program::parse_program_file(program_path) {
            let report = evaluate_deployment_gates(
                &program,
                &source,
                &ReadinessOptions::default(),
                &DeploymentGatePolicy::default(),
            );
            for gate in &report.gates {
                checklist.push(serde_json::json!({
                    "name": gate.name,
                    "passed": gate.passed,
                    "message": gate.message,
                }));
            }
            gate_report = Some(report);
        }
    }

    let ready = checklist
        .iter()
        .all(|item| item.get("passed").and_then(|value| value.as_bool()).unwrap_or(false));

    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "ready": ready,
        "readiness": readiness,
        "device_pool": pool,
        "alert_count": state.alert_store.list().len(),
        "fleet_agent_count": fleet.agents.len(),
        "checklist": checklist,
        "gates": gate_report,
    }))
}
