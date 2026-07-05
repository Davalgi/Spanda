//! Entity-attached bio-inspired resilient autonomy profile fields.
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Operational memory category references attached to an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityMemoryRefs {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reflex: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub working: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub episodic: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedural: Vec<String>,
}

/// Reflex registration summary on an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityReflexSummary {
    pub id: String,
    pub name: String,
    pub priority: u8,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_triggered_at: Option<String>,
}

/// Confidence snapshot for fused observations on an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityConfidenceSnapshot {
    #[serde(default)]
    pub score: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicts: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,
}

/// Homeostasis stability snapshot for an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityHomeostasisSnapshot {
    #[serde(default)]
    pub stable: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drift_signals: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_report_at: Option<String>,
}

/// Immunity / quarantine status for an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityImmunityStatus {
    #[serde(default)]
    pub quarantined: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threat_level: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub violations: Vec<String>,
}

/// Damage-risk index for an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityDamageRisk {
    #[serde(default)]
    pub index: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub risk_signals: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protective_action: Option<String>,
}

/// Adaptive recovery confidence for an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityRecoveryConfidence {
    #[serde(default)]
    pub score: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_strategy: Option<String>,
    #[serde(default)]
    pub attempts: u32,
}

/// Bio-inspired resilient autonomy profile attached to every entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityAutonomyProfile {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reflexes: Vec<EntityReflexSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<EntityConfidenceSnapshot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homeostasis: Option<EntityHomeostasisSnapshot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immunity_status: Option<EntityImmunityStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_refs: Option<EntityMemoryRefs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage_risk: Option<EntityDamageRisk>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_confidence: Option<EntityRecoveryConfidence>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}
