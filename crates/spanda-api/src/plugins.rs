//! REST handlers for installed Control Center and CLI plugins.

use crate::admin_ops::not_found_response;
use crate::handlers::{bad_request, ensure_rbac, json_ok};
use crate::state::ControlCenterState;
use spanda_deploy_http::HttpResponse;
use spanda_plugin::registry::search_plugins;
use spanda_plugin::runtime::{InstalledPlugin, PluginInspectReport, PluginManager, PluginState};
use spanda_security::{RbacAction, RbacContext};
use std::fs;
use std::path::PathBuf;

const HOST_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Route plugin list, search, install/enable/disable, and Control Center bundle subpaths.
pub fn route_plugins(
    state: &ControlCenterState,
    path: &str,
    method: &str,
    query: &str,
    body: &str,
    ctx: Option<&RbacContext>,
) -> Option<HttpResponse> {
    // List every installed plugin with marketplace-friendly fields.
    if path == "/v1/plugins" && method == "GET" {
        return Some(list_all_plugins(state));
    }
    // Search the bundled plugin registry (CLI `spanda plugin search` parity).
    if path == "/v1/plugins/search" && method == "GET" {
        return Some(search_plugins_handler(query));
    }
    // Install from registry name or local path (CLI `spanda plugin install` parity).
    if path == "/v1/plugins/install" && method == "POST" {
        return Some(install_plugin(state, body, ctx));
    }
    // Enabled Control Center UI plugins for nav contribution.
    if path == "/v1/plugins/control-center" && method == "GET" {
        return Some(list_control_center_plugins(state));
    }
    // Bundle fetch for a Control Center UI plugin panel host.
    if let Some(rest) = path.strip_prefix("/v1/plugins/control-center/") {
        if method != "GET" {
            return None;
        }
        let plugin_name = rest.strip_suffix("/bundle")?;
        if plugin_name.is_empty() || plugin_name.contains('/') {
            return None;
        }
        return Some(control_center_plugin_bundle(state, plugin_name));
    }
    // Enable / disable by plugin name.
    if let Some(rest) = path.strip_prefix("/v1/plugins/") {
        if rest.contains('/') {
            let (name, action) = rest.split_once('/')?;
            if name.is_empty() || name.contains('/') {
                return None;
            }
            return match (action, method) {
                ("enable", "POST") => Some(set_plugin_state(state, name, true, ctx)),
                ("disable", "POST") => Some(set_plugin_state(state, name, false, ctx)),
                _ => None,
            };
        }
    }
    None
}

/// GET /v1/plugins/control-center — enabled Control Center UI plugins.
pub fn list_control_center_plugins(state: &ControlCenterState) -> HttpResponse {
    // Open the project plugin store and serialize inspect reports.
    //
    // Parameters:
    // - `state` — Control Center runtime state (project root)
    //
    // Returns:
    // JSON `{ "plugins": [PluginInspectReport, ...] }` or an empty list.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = list_control_center_plugins(state);

    let Some(project_root) = state.project_root() else {
        return json_ok(&serde_json::json!({ "plugins": [] }));
    };
    let Ok(manager) = PluginManager::open(&project_root, HOST_VERSION) else {
        return bad_request("plugin store unavailable");
    };
    match manager.list_control_center_plugins() {
        Ok(plugins) => {
            let enriched: Vec<_> = plugins.iter().map(plugin_list_entry).collect();
            json_ok(&serde_json::json!({ "plugins": enriched }))
        }
        Err(err) => bad_request(&err.to_string()),
    }
}

/// GET /v1/plugins — installed plugins with `control_center_panels` for Marketplace UI.
pub fn list_all_plugins(state: &ControlCenterState) -> HttpResponse {
    // Build marketplace-aligned entries from the on-disk plugin store.
    //
    // Parameters:
    // - `state` — Control Center runtime state (project root)
    //
    // Returns:
    // JSON `{ "plugins": [...] }` with flat fields plus inspect + panels.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = list_all_plugins(state);

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
        .filter_map(|record| {
            // Prefer full inspect reports so Marketplace can read manifest + panels.
            match manager.store().inspect(&record.name) {
                Ok(report) => Some(plugin_list_entry(&report)),
                Err(_) => Some(flat_plugin_fallback(&record)),
            }
        })
        .collect();
    json_ok(&serde_json::json!({ "plugins": plugins }))
}

/// GET /v1/plugins/search?q= — registry search (CLI parity).
fn search_plugins_handler(query: &str) -> HttpResponse {
    // Filter the bundled plugin registry by name, description, or type.
    //
    // Parameters:
    // - `query` — raw query string (`q=` preferred)
    //
    // Returns:
    // JSON `{ "plugins": [registry entries] }`.
    //
    // Options:
    // `q` — search substring (empty returns the full index).
    //
    // Example:
    // let response = search_plugins_handler("q=readiness");

    let q = crate::handlers::parse_query(query)
        .get("q")
        .cloned()
        .unwrap_or_default();
    let results = search_plugins(&q);
    json_ok(&serde_json::json!({ "plugins": results, "query": q }))
}

/// POST /v1/plugins/install — install from registry name or local path.
fn install_plugin(
    state: &ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    // Install a plugin after Provision RBAC, matching CLI install semantics.
    //
    // Parameters:
    // - `state` — Control Center state with project root
    // - `body` — JSON `{ "name"?, "path"?, "approve_dangerous"? }`
    // - `ctx` — authenticated RBAC context
    //
    // Returns:
    // Installed plugin record JSON, or 400/401.
    //
    // Options:
    // `approve_dangerous` — allow dangerous capabilities (default false).
    //
    // Example:
    // POST /v1/plugins/install {"path":"examples/plugins/control-center-panel"}

    if let Err(response) = ensure_rbac(ctx, RbacAction::Provision) {
        return response;
    }
    let Some(project_root) = state.project_root() else {
        return bad_request("project root unavailable");
    };
    let payload: serde_json::Value = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(_) => return bad_request("invalid JSON body"),
    };
    let approve_dangerous = payload
        .get("approve_dangerous")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let path = payload
        .get("path")
        .and_then(|v| v.as_str())
        .map(PathBuf::from);
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let Ok(mut manager) = PluginManager::open(&project_root, HOST_VERSION) else {
        return bad_request("plugin store unavailable");
    };

    // Prefer an explicit local path when provided.
    if let Some(source) = path {
        let resolved = if source.is_absolute() {
            source
        } else {
            project_root.join(&source)
        };
        return match manager
            .store_mut()
            .install_from_dir(&resolved, HOST_VERSION, approve_dangerous)
        {
            Ok(record) => json_ok(&serde_json::json!({
                "ok": true,
                "plugin": record,
            })),
            Err(err) => bad_request(&err.to_string()),
        };
    }

    let Some(name) = name else {
        return bad_request("missing plugin name or path");
    };

    // Resolve directory paths passed as the name field (CLI parity).
    if PathBuf::from(&name).is_dir() {
        return match manager.store_mut().install_from_dir(
            PathBuf::from(&name).as_path(),
            HOST_VERSION,
            approve_dangerous,
        ) {
            Ok(record) => json_ok(&serde_json::json!({
                "ok": true,
                "plugin": record,
            })),
            Err(err) => bad_request(&err.to_string()),
        };
    }

    // Try registry install, then fall back to examples/plugins/<slug>.
    match manager.install_from_registry(&name, None, &project_root, approve_dangerous) {
        Ok(record) => {
            return json_ok(&serde_json::json!({
                "ok": true,
                "plugin": record,
            }));
        }
        Err(_) => {
            // Fall through to local example path resolution.
        }
    }

    let slug = name
        .strip_prefix("spanda-plugin-")
        .unwrap_or(name.as_str())
        .replace('-', "_");
    let example = project_root.join("examples/plugins").join(&slug);
    if example.is_dir() {
        return match manager
            .store_mut()
            .install_from_dir(&example, HOST_VERSION, approve_dangerous)
        {
            Ok(record) => json_ok(&serde_json::json!({
                "ok": true,
                "plugin": record,
            })),
            Err(err) => bad_request(&err.to_string()),
        };
    }

    bad_request(&format!(
        "plugin not found: {name}; provide path or a registry name"
    ))
}

/// POST /v1/plugins/{name}/enable|disable — lifecycle toggles (CLI parity).
fn set_plugin_state(
    state: &ControlCenterState,
    name: &str,
    enable: bool,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    // Enable or disable an installed plugin under Operate RBAC.
    //
    // Parameters:
    // - `state` — Control Center state with project root
    // - `name` — installed plugin name
    // - `enable` — true to enable, false to disable
    // - `ctx` — authenticated RBAC context
    //
    // Returns:
    // JSON `{ "ok": true, "name", "state" }` or 400/401.
    //
    // Options:
    // None.
    //
    // Example:
    // POST /v1/plugins/spanda-plugin-control-center-panel/enable

    if let Err(response) = ensure_rbac(ctx, RbacAction::Operate) {
        return response;
    }
    let Some(project_root) = state.project_root() else {
        return bad_request("project root unavailable");
    };
    let Ok(mut manager) = PluginManager::open(&project_root, HOST_VERSION) else {
        return bad_request("plugin store unavailable");
    };
    let result = if enable {
        manager.store_mut().enable(name)
    } else {
        manager.store_mut().disable(name)
    };
    match result {
        Ok(()) => {
            let state_label = if enable {
                PluginState::Enabled
            } else {
                PluginState::Disabled
            };
            json_ok(&serde_json::json!({
                "ok": true,
                "name": name,
                "state": format!("{:?}", state_label).to_lowercase(),
            }))
        }
        Err(err) => bad_request(&err.to_string()),
    }
}

/// GET /v1/plugins/control-center/{plugin}/bundle — TypeScript panel artifact when present.
pub fn control_center_plugin_bundle(state: &ControlCenterState, plugin_name: &str) -> HttpResponse {
    // Serve the panel JS bundle; signature checks already ran at install time.
    //
    // Parameters:
    // - `state` — Control Center state with project root
    // - `plugin_name` — installed Control Center UI plugin name
    //
    // Returns:
    // JSON with `bundle` text when `index.js` exists, else `available: false`.
    //
    // Options:
    // None.
    //
    // Example:
    // let response = control_center_plugin_bundle(state, "spanda-plugin-control-center-panel");

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
    let signed = manifest.security.signed;
    let content = match fs::read_to_string(&bundle_path) {
        Ok(text) => text,
        Err(_) => {
            return json_ok(&serde_json::json!({
                "plugin": plugin_name,
                "bundle_path": bundle_path.display().to_string(),
                "available": false,
                "signed": signed,
                "signature_check": "install-time",
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
            "signed": signed,
            "signature_check": "install-time",
            "sandbox": manifest.security.sandbox,
        })
        .to_string(),
    }
}

/// Serialize an inspect report for Marketplace / list consumers.
fn plugin_list_entry(report: &PluginInspectReport) -> serde_json::Value {
    // Flatten control_center panels and keep inspect + flat identity fields.
    let panels: Vec<_> = report
        .manifest
        .control_center
        .panels
        .iter()
        .map(|panel| {
            serde_json::json!({
                "id": panel.id,
                "title": panel.title,
                "component": panel.component,
            })
        })
        .collect();
    serde_json::json!({
        "name": report.installed.name,
        "version": report.installed.version,
        "state": format!("{:?}", report.installed.state).to_lowercase(),
        "plugin_type": report.installed.plugin_type,
        "trust_tier": report.installed.trust_tier,
        "installed": report.installed,
        "manifest": report.manifest,
        "control_center_panels": panels,
    })
}

/// Minimal entry when inspect fails (corrupt install directory).
fn flat_plugin_fallback(record: &InstalledPlugin) -> serde_json::Value {
    serde_json::json!({
        "name": record.name,
        "version": record.version,
        "state": format!("{:?}", record.state).to_lowercase(),
        "plugin_type": record.plugin_type,
        "trust_tier": record.trust_tier,
        "installed": record,
        "control_center_panels": [],
    })
}
