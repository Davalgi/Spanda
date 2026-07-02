//! Plugin capability registry and enforcement.

use crate::error::{PluginError, PluginResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Well-known plugin host capabilities.
pub const KNOWN_PLUGIN_CAPABILITIES: &[&str] = &[
    "entity.read",
    "entity.write",
    "device.read",
    "device.write",
    "readiness.read",
    "readiness.write",
    "assurance.read",
    "assurance.write",
    "diagnosis.read",
    "diagnosis.write",
    "recovery.read",
    "recovery.write",
    "health.read",
    "health.write",
    "trust.read",
    "trust.write",
    "telemetry.read",
    "telemetry.write",
    "report.generate",
    "network.outbound",
    "filesystem.read",
    "filesystem.write",
];

/// One declared plugin capability.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PluginCapability {
    pub name: String,
}

/// Set of granted capabilities for a loaded plugin instance.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CapabilitySet {
    capabilities: HashSet<String>,
}

impl CapabilitySet {
    /// Build a capability set from manifest capability names.
    pub fn from_names(names: &[String]) -> Self {
        // Insert each declared capability into the set.
        let mut capabilities = HashSet::new();
        for name in names {
            capabilities.insert(name.clone());
        }
        Self { capabilities }
    }

    /// Return true when the capability was declared in the manifest.
    pub fn has(&self, capability: &str) -> bool {
        self.capabilities.contains(capability)
    }

    /// Enumerate granted capabilities in stable order.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        let mut names: Vec<_> = self.capabilities.iter().collect();
        names.sort();
        names.into_iter()
    }

    /// Grant an additional capability at runtime (admin override).
    pub fn grant(&mut self, capability: impl Into<String>) {
        self.capabilities.insert(capability.into());
    }
}

/// Validate that every requested capability is known to the host.
pub fn validate_capability_list(requires: &[String]) -> PluginResult<()> {
    for cap in requires {
        if !KNOWN_PLUGIN_CAPABILITIES.contains(&cap.as_str()) {
            return Err(PluginError::Capability(format!(
                "unknown capability: {cap}"
            )));
        }
    }
    Ok(())
}

/// Enforce capability access at runtime; deny undeclared access.
pub fn enforce_capability(set: &CapabilitySet, capability: &str) -> PluginResult<()> {
    if set.has(capability) {
        Ok(())
    } else {
        Err(PluginError::Capability(format!(
            "access denied: capability '{capability}' not declared in manifest"
        )))
    }
}

/// Return capabilities considered dangerous without explicit approval.
pub fn dangerous_capabilities() -> &'static [&'static str] {
    &[
        "entity.write",
        "device.write",
        "filesystem.write",
        "network.outbound",
        "readiness.write",
        "recovery.write",
    ]
}

/// Check whether a capability set includes any dangerous capability.
pub fn has_dangerous_capabilities(set: &CapabilitySet) -> bool {
    dangerous_capabilities().iter().any(|cap| set.has(cap))
}
