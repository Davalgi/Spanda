//! Build Recovery Orchestrator plugin registries from installed plugins.
//!
use crate::persistence::persist_runtime_state;
use crate::state::ControlCenterState;
use spanda_plugin::manifest::PluginManifest;
use spanda_plugin::runtime::PluginManager;
use spanda_recovery::{PluginRecoveryExtension, RecoveryOrchestrator, RecoveryPluginRegistry};

const HOST_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Load plugin-contributed recovery extensions for the current project.
pub fn build_recovery_plugin_registry(state: &ControlCenterState) -> RecoveryPluginRegistry {
    // Walk enabled plugins and register declared recovery extensions.
    let mut registry = RecoveryPluginRegistry::new();
    let Some(project_root) = state.project_root() else {
        return registry;
    };
    let Ok(manager) = PluginManager::open(&project_root, HOST_VERSION) else {
        return registry;
    };
    for name in manager.store().enabled_plugin_names() {
        let Some(record) = manager.store().get(&name) else {
            continue;
        };
        let Ok(manifest) = PluginManifest::load_from_dir(&record.install_path) else {
            continue;
        };
        for decl in &manifest.recovery.extensions {
            let description = decl
                .description
                .clone()
                .or_else(|| decl.trigger.clone())
                .unwrap_or_default();
            registry.register(PluginRecoveryExtension {
                plugin_id: manifest.plugin.name.clone(),
                extension_kind: decl.kind.clone(),
                name: decl.name.clone(),
                description,
                trigger: decl.trigger.clone(),
                strategy: decl.strategy.clone(),
            });
        }
    }
    registry
}

/// Create an orchestrator wired with persisted history and plugin extensions.
pub fn orchestrator_for_state(state: &ControlCenterState) -> RecoveryOrchestrator {
    let mut orchestrator = RecoveryOrchestrator::new().with_history(state.recovery_history.clone());
    let registry = build_recovery_plugin_registry(state);
    if !registry.all().is_empty() {
        orchestrator = orchestrator.with_plugins(registry);
    }
    orchestrator
}

/// Persist orchestrator evidence history back to Control Center state.
pub fn save_orchestrator_history(
    state: &mut ControlCenterState,
    orchestrator: RecoveryOrchestrator,
) {
    state.recovery_history = orchestrator.into_history();
    let _ = persist_runtime_state(state);
}

/// Dispatch recovery lifecycle hooks to enabled plugins.
pub fn dispatch_recovery_completed_hook(state: &ControlCenterState, payload: serde_json::Value) {
    let Some(project_root) = state.project_root() else {
        return;
    };
    if let Ok(mut manager) = PluginManager::open(&project_root, HOST_VERSION) {
        let _ = manager.dispatch_hook_to_enabled(
            spanda_plugin::hooks::PluginHook::OnRecoveryCompleted,
            payload,
        );
    }
}
