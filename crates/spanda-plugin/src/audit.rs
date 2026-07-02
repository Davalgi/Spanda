//! Plugin audit logging for install, enable, and hook events.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// One audit log entry for plugin lifecycle or hook activity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginAuditEntry {
    pub timestamp_ms: u64,
    pub plugin: String,
    pub action: String,
    pub detail: String,
}

/// In-memory audit log for plugin operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginAuditLog {
    entries: Vec<PluginAuditEntry>,
}

impl PluginAuditLog {
    pub fn record(&mut self, plugin: &str, action: &str, detail: impl Into<String>) {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        self.entries.push(PluginAuditEntry {
            timestamp_ms,
            plugin: plugin.to_string(),
            action: action.to_string(),
            detail: detail.into(),
        });
    }

    pub fn entries(&self) -> &[PluginAuditEntry] {
        &self.entries
    }
}
