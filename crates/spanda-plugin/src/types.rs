//! Plugin type taxonomy and shared identifiers.

use serde::{Deserialize, Serialize};

/// Supported plugin categories in the Spanda platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginType {
    Provider,
    ControlCenterUi,
    Cli,
    Readiness,
    Assurance,
    Diagnosis,
    Recovery,
    Trust,
    Health,
    Telemetry,
    DeviceDiscovery,
    ReportGenerator,
    SolutionBlueprint,
}

impl PluginType {
    /// Parse a plugin type from manifest text.
    pub fn parse_str(value: &str) -> Option<Self> {
        // Map manifest type strings to enum variants.
        //
        // Parameters:
        // - `value` — raw `[plugin].type` field
        //
        // Returns:
        // Parsed variant, or `None` when unknown.
        //
        // Options:
        // None.
        //
        // Example:
        // let t = PluginType::parse_str("readiness");

        match value {
            "provider" => Some(Self::Provider),
            "control-center-ui" | "control_center_ui" => Some(Self::ControlCenterUi),
            "cli" => Some(Self::Cli),
            "readiness" => Some(Self::Readiness),
            "assurance" => Some(Self::Assurance),
            "diagnosis" => Some(Self::Diagnosis),
            "recovery" => Some(Self::Recovery),
            "trust" => Some(Self::Trust),
            "health" => Some(Self::Health),
            "telemetry" => Some(Self::Telemetry),
            "device-discovery" | "device_discovery" => Some(Self::DeviceDiscovery),
            "report-generator" | "report_generator" => Some(Self::ReportGenerator),
            "solution-blueprint" | "solution_blueprint" => Some(Self::SolutionBlueprint),
            _ => None,
        }
    }

    /// Canonical kebab-case identifier for CLI and registry output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Provider => "provider",
            Self::ControlCenterUi => "control-center-ui",
            Self::Cli => "cli",
            Self::Readiness => "readiness",
            Self::Assurance => "assurance",
            Self::Diagnosis => "diagnosis",
            Self::Recovery => "recovery",
            Self::Trust => "trust",
            Self::Health => "health",
            Self::Telemetry => "telemetry",
            Self::DeviceDiscovery => "device-discovery",
            Self::ReportGenerator => "report-generator",
            Self::SolutionBlueprint => "solution-blueprint",
        }
    }
}

impl std::fmt::Display for PluginType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Current stable plugin host API version.
pub const CURRENT_API_VERSION: &str = "v1";

/// Default Spanda version requirement when manifest omits compatibility.
pub const DEFAULT_SPANDA_VERSION_REQ: &str = ">=0.4.0";
