//! Plugin registry trust tiers and index lookup.

use crate::error::{PluginError, PluginResult};
use crate::types::PluginType;
use serde::{Deserialize, Serialize};
use spanda_package::registry_sign::{
    registry_trust_key, verify_registry_signature, RegistryVersionSignature,
};
use std::collections::BTreeMap;

/// Registry trust tier for a published plugin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginTrustTier {
    Official,
    Verified,
    Community,
    Experimental,
    Deprecated,
    Blocked,
}

impl PluginTrustTier {
    pub fn parse_str(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "official" => Some(Self::Official),
            "verified" => Some(Self::Verified),
            "community" => Some(Self::Community),
            "experimental" => Some(Self::Experimental),
            "deprecated" => Some(Self::Deprecated),
            "blocked" => Some(Self::Blocked),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Official => "official",
            Self::Verified => "verified",
            Self::Community => "community",
            Self::Experimental => "experimental",
            Self::Deprecated => "deprecated",
            Self::Blocked => "blocked",
        }
    }

    pub fn install_allowed(self) -> bool {
        !matches!(self, Self::Blocked)
    }
}

impl std::fmt::Display for PluginTrustTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One plugin entry in the registry index.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginRegistryEntry {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub plugin_type: Option<String>,
    #[serde(default)]
    pub publisher: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub tier: Option<String>,
    pub versions: Vec<String>,
    #[serde(default)]
    pub version_checksums: BTreeMap<String, String>,
    #[serde(default)]
    pub version_signatures: BTreeMap<String, RegistryVersionSignature>,
}

impl PluginRegistryEntry {
    pub fn trust_tier(&self) -> PluginTrustTier {
        self.tier
            .as_deref()
            .and_then(PluginTrustTier::parse_str)
            .unwrap_or(PluginTrustTier::Community)
    }

    pub fn latest_version(&self) -> Option<&str> {
        self.versions.last().map(String::as_str)
    }

    pub fn version_sha256(&self, version: &str) -> Option<&str> {
        self.version_checksums.get(version).map(String::as_str)
    }

    pub fn version_signature(&self, version: &str) -> Option<&RegistryVersionSignature> {
        self.version_signatures.get(version)
    }
}

/// Full plugin registry index.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginRegistryIndex {
    pub plugins: Vec<PluginRegistryEntry>,
}

const BUNDLED_PLUGIN_REGISTRY: &str = include_str!("../plugin-registry/index.json");

pub fn plugin_registry_index() -> PluginRegistryIndex {
    serde_json::from_str(BUNDLED_PLUGIN_REGISTRY).unwrap_or_default()
}

pub fn lookup_plugin_entry(name: &str) -> Option<PluginRegistryEntry> {
    plugin_registry_index()
        .plugins
        .into_iter()
        .find(|entry| entry.name == name)
}

pub fn search_plugins(query: &str) -> Vec<PluginRegistryEntry> {
    let q = query.to_ascii_lowercase();
    if q.is_empty() {
        return plugin_registry_index().plugins;
    }
    plugin_registry_index()
        .plugins
        .into_iter()
        .filter(|entry| {
            entry.name.to_ascii_lowercase().contains(&q)
                || entry
                    .description
                    .as_deref()
                    .is_some_and(|d| d.to_ascii_lowercase().contains(&q))
                || entry
                    .plugin_type
                    .as_deref()
                    .is_some_and(|t| t.to_ascii_lowercase().contains(&q))
        })
        .collect()
}

pub fn verify_plugin_registry_signature(
    name: &str,
    version: &str,
    sha256: &str,
    signature: &RegistryVersionSignature,
) -> bool {
    let trust_key = registry_trust_key().unwrap_or_else(|| signature.public_key.clone());
    verify_registry_signature(name, version, sha256, signature, &trust_key)
}

pub fn entry_plugin_type(entry: &PluginRegistryEntry) -> Option<PluginType> {
    entry.plugin_type.as_deref().and_then(PluginType::parse_str)
}

pub fn ensure_installable(entry: &PluginRegistryEntry) -> PluginResult<()> {
    let tier = entry.trust_tier();
    if tier.install_allowed() {
        Ok(())
    } else {
        Err(PluginError::Registry(format!(
            "plugin '{}' is blocked and cannot be installed",
            entry.name
        )))
    }
}
