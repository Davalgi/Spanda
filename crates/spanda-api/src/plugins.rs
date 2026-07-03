//! REST handlers for installed Control Center and CLI plugins.

use crate::admin_ops::not_found_response;
use crate::handlers::{bad_request, json_ok};
use crate::state::ControlCenterState;
use spanda_deploy_http::HttpResponse;
use spanda_plugin::runtime::PluginManager;
use std::fs;

const HOST_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Route plugin list and Control Center bundle subpaths.
pub fn route_plugins(state: &ControlCenterState, path: &str, method: &str) -> Option<HttpResponse> {
    if path == "/v1/plugins" && method == "GET" {
        return Some(list_all_plugins(state));
    }
    if path == "/v1/plugins/control-center" && method == "GET" {
        return Some(list_control_center_plugins(state));
    }
    let rest = path.strip_prefix("/v1/plugins/control-center/")?;
    if method != "GET" {
        return None;
    }
    let plugin_name = rest.strip_suffix("/bundle")?;
    if plugin_name.is_empty() || plugin_name.contains('/') {
        return None;
    }
    Some(control_center_plugin_bundle(state, plugin_name))
}

pub fn list_control_center_plugins(state: &ControlCenterState) -> HttpResponse {
    let Some(project_root) = state.project_root() else {
        return json_ok(&serde_json::json!({ "plugins": [] }));
    };
    let Ok(manager) = PluginManager::open(&project_root, HOST_VERSION) else {
        return bad_request("plugin store unavailable");
    };
    match manager.list_control_center_plugins() {
        Ok(plugins) => json_ok(&serde_json::json!({ "plugins": plugins })),
        Err(err) => bad_request(&err.to_string()),
    }
}

pub fn list_all_plugins(state: &ControlCenterState) -> HttpResponse {
    let Some(project_root) = state.project_root() else {
        return json_ok(&serde_json::json!({ "plugins": [] }));
    };
    let Ok(manager) = PluginManager::open(&project_root, HOST_VERSION) else {
        return bad_request("plugin store unavailable");
    };
    let plugins: Vec<_> = manager
        .store()
        .list()
        .into_iter()
        .map(|p| {
            serde_json::json!({
                "name": p.name,
                "version": p.version,
                "state": format!("{:?}", p.state).to_lowercase(),
                "plugin_type": p.plugin_type,
                "trust_tier": p.trust_tier,
            })
        })
        .collect();
    json_ok(&serde_json::json!({ "plugins": plugins }))
}

/// GET /v1/plugins/control-center/{plugin}/bundle — TypeScript panel artifact when present.
pub fn control_center_plugin_bundle(state: &ControlCenterState, plugin_name: &str) -> HttpResponse {
    let Some(project_root) = state.project_root() else {
        return not_found_response();
    };
    let Ok(manager) = PluginManager::open(&project_root, HOST_VERSION) else {
        return bad_request("plugin store unavailable");
    };
    let Ok(report) = manager.store().inspect(plugin_name) else {
        return not_found_response();
    };
    let manifest = &report.manifest;
    let bundle_path = manifest.artifact_path(&report.installed.install_path, "typescript");
    let content = match fs::read_to_string(&bundle_path) {
        Ok(text) => text,
        Err(_) => {
            return json_ok(&serde_json::json!({
                "plugin": plugin_name,
                "bundle_path": bundle_path.display().to_string(),
                "available": false,
                "hint": "Build index.js in the plugin install directory for TypeScript panels.",
            }));
        }
    };
    HttpResponse {
        status: 200,
        body: serde_json::json!({
            "version": "v1",
            "plugin": plugin_name,
            "available": true,
            "bundle": content,
        })
        .to_string(),
    }
}
