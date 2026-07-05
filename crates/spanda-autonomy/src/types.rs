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
    "Sensory fusion validators are rule-based; no live sensor fusion pipeline yet.",
    "Attention scoring uses static policies; Control Center UI is placeholder-only.",
    "Adaptive recovery learning is statistics-based, not ML.",
    "Maintenance/sleep mode scheduling is declarative; OTA integration is partial.",
    "Habituation/sensitization applies to CLI-reported alert streams, not all telemetry backends.",
];
