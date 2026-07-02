//! Spanda and plugin API version compatibility checks.

use crate::error::{PluginError, PluginResult};
use crate::manifest::PluginManifest;
use crate::types::{CURRENT_API_VERSION, DEFAULT_SPANDA_VERSION_REQ};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

/// Result of compatibility validation for one plugin manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompatibilityReport {
    pub spanda_version_ok: bool,
    pub api_version_ok: bool,
    pub spanda_version_required: String,
    pub api_version_required: String,
    pub spanda_version_current: String,
    pub api_version_current: String,
    pub compatible: bool,
    pub detail: String,
}

/// Validate manifest Spanda version constraint against the running host.
pub fn validate_spanda_version(
    manifest: &PluginManifest,
    host_version: &str,
) -> PluginResult<CompatibilityReport> {
    let required_raw = manifest.compatibility.spanda_version.trim();
    let required = if required_raw.is_empty() {
        DEFAULT_SPANDA_VERSION_REQ.to_string()
    } else {
        required_raw.to_string()
    };

    let req = VersionReq::parse(&required)?;
    let current = Version::parse(host_version)?;
    let spanda_ok = req.matches(&current);
    let api_ok = validate_api_version(&manifest.compatibility.api_version);
    let compatible = spanda_ok && api_ok;
    let detail = if compatible {
        "plugin is compatible with this Spanda host".to_string()
    } else if !spanda_ok {
        format!("requires Spanda {required}, host is {host_version}")
    } else {
        format!(
            "requires plugin API {}, host supports {CURRENT_API_VERSION}",
            manifest.compatibility.api_version
        )
    };

    Ok(CompatibilityReport {
        spanda_version_ok: spanda_ok,
        api_version_ok: api_ok,
        spanda_version_required: required,
        api_version_required: manifest.compatibility.api_version.clone(),
        spanda_version_current: host_version.to_string(),
        api_version_current: CURRENT_API_VERSION.to_string(),
        compatible,
        detail,
    })
}

/// Validate plugin API version against host-supported API.
pub fn validate_api_version(api_version: &str) -> bool {
    api_version.trim() == CURRENT_API_VERSION
}

/// Fail when compatibility report indicates incompatibility.
pub fn require_compatible(report: &CompatibilityReport) -> PluginResult<()> {
    if report.compatible {
        Ok(())
    } else {
        Err(PluginError::Compatibility(report.detail.clone()))
    }
}
