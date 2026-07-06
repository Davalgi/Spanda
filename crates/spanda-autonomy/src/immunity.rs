//! Platform immunity — quarantine and threat response.
//!
use serde::{Deserialize, Serialize};
use spanda_config::entity::{EntityRecord, EntityTrustStatus};

/// Immunity policy thresholds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ImmunePolicy {
    pub quarantine_untrusted: bool,
    pub block_unsigned_plugins: bool,
    pub reject_spoofed_sensors: bool,
}

impl ImmunePolicy {
    pub fn platform_defaults() -> Self {
        Self {
            quarantine_untrusted: true,
            block_unsigned_plugins: true,
            reject_spoofed_sensors: true,
        }
    }
}

/// Quarantine action for a compromised entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineAction {
    Monitor,
    Isolate,
    RemoveFromMission,
    Block,
}

/// Threat response tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThreatResponse {
    Log,
    Alert,
    Quarantine,
    Isolate,
    Block,
}

/// Trust boundary violation record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustBoundaryViolation {
    pub entity_id: String,
    pub boundary: String,
    pub detail: String,
}

/// Immune event emitted by scan or runtime detection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImmuneEvent {
    pub entity_id: String,
    pub kind: String,
    pub response: ThreatResponse,
    pub detail: String,
}

/// Isolation decision outcome.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsolationDecision {
    pub entity_id: String,
    pub quarantine: bool,
    pub action: QuarantineAction,
    pub reasons: Vec<String>,
}

/// Evaluate immunity events from entity trust and metadata hints.
pub fn evaluate_immunity(entity: &EntityRecord, policy: &ImmunePolicy) -> Vec<ImmuneEvent> {
    let mut events = Vec::new();
    if policy.quarantine_untrusted && entity.trust_status == EntityTrustStatus::Untrusted {
        events.push(ImmuneEvent {
            entity_id: entity.id.clone(),
            kind: "untrusted_entity".into(),
            response: ThreatResponse::Quarantine,
            detail: "Entity trust status is untrusted".into(),
        });
    }
    if policy.quarantine_untrusted && entity.trust_status == EntityTrustStatus::Compromised {
        events.push(ImmuneEvent {
            entity_id: entity.id.clone(),
            kind: "compromised_trust".into(),
            response: ThreatResponse::Isolate,
            detail: "Entity trust is compromised".into(),
        });
    }
    if policy.block_unsigned_plugins
        && entity.metadata.get("plugin.unsigned") == Some(&"true".into())
    {
        events.push(ImmuneEvent {
            entity_id: entity.id.clone(),
            kind: "unsigned_plugin".into(),
            response: ThreatResponse::Block,
            detail: "Unsigned plugin detected".into(),
        });
    }
    if policy.reject_spoofed_sensors
        && entity.metadata.get("sensor.spoofing_detected") == Some(&"true".into())
    {
        events.push(ImmuneEvent {
            entity_id: entity.id.clone(),
            kind: "sensor_spoofing".into(),
            response: ThreatResponse::Quarantine,
            detail: "Sensor spoofing signal rejected".into(),
        });
    }
    events
}

/// Decide whether to quarantine an entity based on immunity events.
pub fn evaluate_quarantine_decision(
    entity: &EntityRecord,
    policy: &ImmunePolicy,
) -> IsolationDecision {
    let events = evaluate_immunity(entity, policy);
    let reasons: Vec<String> = events.iter().map(|e| e.detail.clone()).collect();
    let quarantine = !events.is_empty();
    let action = if events
        .iter()
        .any(|e| matches!(e.response, ThreatResponse::Isolate))
    {
        QuarantineAction::RemoveFromMission
    } else if events
        .iter()
        .any(|e| matches!(e.response, ThreatResponse::Block))
    {
        QuarantineAction::Block
    } else if quarantine {
        QuarantineAction::Isolate
    } else {
        QuarantineAction::Monitor
    };
    IsolationDecision {
        entity_id: entity.id.clone(),
        quarantine,
        action,
        reasons,
    }
}
