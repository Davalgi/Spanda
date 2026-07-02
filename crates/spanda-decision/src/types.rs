//! Core types for the distributed decision architecture (brain / spinal cord / reflex model).

use serde::{Deserialize, Serialize};

/// Decision layer in the hierarchical autonomy model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DecisionLayer {
    /// Layer 0 — immediate safety reflexes (milliseconds, no cloud).
    #[default]
    Reflex,
    /// Layer 1 — local entity autonomy (milliseconds to seconds).
    LocalEntity,
    /// Layer 2 — fleet / swarm / site coordination (seconds).
    GroupFleet,
    /// Layer 3 — control center / cloud strategy (seconds to minutes).
    ControlCenter,
}

impl DecisionLayer {
    /// Human-readable label for reports.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Reflex => "reflex",
            Self::LocalEntity => "local_entity",
            Self::GroupFleet => "group_fleet",
            Self::ControlCenter => "control_center",
        }
    }
}

/// Category of autonomous decision.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionType {
    Safety,
    Recovery,
    Readiness,
    Trust,
    Mission,
    Takeover,
    Delegation,
    Health,
    Security,
    Policy,
}

/// Scope within which a decision authority applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionScope {
    pub entity_id: Option<String>,
    pub fleet_id: Option<String>,
    pub site_id: Option<String>,
    pub layer: DecisionLayer,
}

/// Hard boundary that local decisions must not cross.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionBoundary {
    pub action: String,
    pub max_layer: DecisionLayer,
    pub requires_approval: bool,
    pub reason: String,
}

/// Delegation of decision authority from one entity to another.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionDelegation {
    pub from_entity: String,
    pub to_entity: String,
    pub actions: Vec<String>,
    pub expires_at_ms: Option<f64>,
    pub policy_version: String,
}

/// Escalation step when local resolution is insufficient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionEscalation {
    pub from_layer: DecisionLayer,
    pub to_layer: DecisionLayer,
    pub reason: String,
    pub entity_id: String,
    pub pending_approval: bool,
    pub escalation_id: String,
}

/// Local decision authority declared on an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionAuthority {
    pub entity_id: String,
    pub local_actions: Vec<String>,
    pub requires_central_approval: Vec<String>,
    pub layer: DecisionLayer,
}

/// Policy governing distributed decisions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionPolicy {
    pub name: String,
    pub version: String,
    pub layer: DecisionLayer,
    pub allowed_actions: Vec<String>,
    pub forbidden_actions: Vec<String>,
    pub signature: Option<String>,
    pub expires_at_ms: Option<f64>,
}

/// Security envelope attached to every local decision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionSecurityEnvelope {
    pub entity_id: String,
    pub authority_scope: String,
    pub policy_version: String,
    pub decision_tree_hash: Option<String>,
    pub timestamp_ms: f64,
    pub nonce: String,
    pub signature: Option<String>,
    pub safety_validation_passed: bool,
    pub trust_validation_passed: bool,
    pub audit_record_id: Option<String>,
}

/// Full distributed decision trace record (extends audit trail).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributedDecisionRecord {
    pub decision_id: String,
    pub layer: DecisionLayer,
    pub decision_type: DecisionType,
    pub entity_id: String,
    pub mission: Option<String>,
    pub timestamp_ms: f64,
    pub inputs: serde_json::Value,
    pub policy_version: String,
    pub local_context: serde_json::Value,
    pub selected_action: String,
    pub rejected_alternatives: Vec<String>,
    pub safety_validation: serde_json::Value,
    pub trust_validation: serde_json::Value,
    pub escalation_path: Vec<DecisionEscalation>,
    pub outcome: Option<String>,
    pub security: DecisionSecurityEnvelope,
}

/// Conflict resolution precedence (lower index = higher priority).
pub const CONFLICT_PRECEDENCE: &[&str] = &[
    "safety_kill_switch",
    "local_immediate_safety",
    "trust_security_block",
    "human_emergency_override",
    "control_center_policy",
    "fleet_coordination",
    "local_optimization",
];
