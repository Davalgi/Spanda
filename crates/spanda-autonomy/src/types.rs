//! Shared types for bio-inspired resilient autonomy.
//!
use serde::{Deserialize, Serialize};

/// Report output format for autonomy CLI and API consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AutonomyReportFormat {
    #[default]
    Text,
    Json,
    Markdown,
}

/// Severity tier for autonomy signals and events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutonomySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Known limitations for bio-inspired autonomy features.
pub const KNOWN_LIMITATIONS: &[&str] = &[
    "Live multi-sensor fusion requires SPANDA_LIVE_FUSION_SENSORS=1 and a registered supplier; default path is entity-derived rule-based fusion.",
    "Attention scoring uses static policies; Control Center UI is placeholder-only.",
    "Adaptive recovery learning is statistics-based, not ML.",
    "Maintenance/sleep mode scheduling is declarative; OTA integration is partial.",
    "Habituation/sensitization applies to CLI-reported alert streams, not all telemetry backends.",
];
