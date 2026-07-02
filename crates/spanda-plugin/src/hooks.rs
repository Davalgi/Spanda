//! Plugin lifecycle hooks and execution context.

use crate::error::{PluginError, PluginResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Supported plugin lifecycle and event hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginHook {
    OnInstall,
    OnEnable,
    OnDisable,
    OnUninstall,
    OnEntityEvent,
    OnHealthChanged,
    OnReadinessCompleted,
    OnDiagnosisCompleted,
    OnRecoveryCompleted,
    OnReportRequested,
}

impl PluginHook {
    pub fn parse_str(value: &str) -> Option<Self> {
        match value {
            "on_install" => Some(Self::OnInstall),
            "on_enable" => Some(Self::OnEnable),
            "on_disable" => Some(Self::OnDisable),
            "on_uninstall" => Some(Self::OnUninstall),
            "on_entity_event" => Some(Self::OnEntityEvent),
            "on_health_changed" => Some(Self::OnHealthChanged),
            "on_readiness_completed" => Some(Self::OnReadinessCompleted),
            "on_diagnosis_completed" => Some(Self::OnDiagnosisCompleted),
            "on_recovery_completed" => Some(Self::OnRecoveryCompleted),
            "on_report_requested" => Some(Self::OnReportRequested),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::OnInstall => "on_install",
            Self::OnEnable => "on_enable",
            Self::OnDisable => "on_disable",
            Self::OnUninstall => "on_uninstall",
            Self::OnEntityEvent => "on_entity_event",
            Self::OnHealthChanged => "on_health_changed",
            Self::OnReadinessCompleted => "on_readiness_completed",
            Self::OnDiagnosisCompleted => "on_diagnosis_completed",
            Self::OnRecoveryCompleted => "on_recovery_completed",
            Self::OnReportRequested => "on_report_requested",
        }
    }
}

/// Context passed to a hook handler.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookContext {
    pub plugin_name: String,
    pub hook: String,
    #[serde(default)]
    pub payload: Value,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Result of executing one plugin hook.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HookExecutionResult {
    pub hook: String,
    pub success: bool,
    #[serde(default)]
    pub output: Option<Value>,
    #[serde(default)]
    pub message: Option<String>,
}

pub fn parse_enabled_hooks(names: &[String]) -> PluginResult<Vec<PluginHook>> {
    let mut hooks = Vec::new();
    for name in names {
        match PluginHook::parse_str(name) {
            Some(hook) => hooks.push(hook),
            None => return Err(PluginError::Hook(format!("unknown hook: {name}"))),
        }
    }
    Ok(hooks)
}

pub fn hook_context(plugin_name: &str, hook: PluginHook, payload: Value) -> HookContext {
    HookContext {
        plugin_name: plugin_name.to_string(),
        hook: hook.as_str().to_string(),
        payload,
        metadata: HashMap::new(),
    }
}
