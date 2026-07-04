//! Runtime influence of governance attributes on platform domains.
//!
//! Readiness, trust, decisions, recovery, and deployment gates call these
//! helpers so autonomy, risk, maturity, and certification affect operations
//! without duplicating governance logic in each domain crate.
//!
use crate::entity_governance::governance_from_entity;
use crate::types::{AutonomyLevel, CertificationStatus, DeploymentMaturity, OperationalRisk};
use serde::{Deserialize, Serialize};
use spanda_config::entity::{EntityRecord, EntityTrustStatus};

/// Governance influence applied to a single entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GovernanceInfluence {
    pub requires_human_approval: bool,
    pub blocks_live_deployment: bool,
    pub requires_simulation: bool,
    pub recovery_escalation_required: bool,
    pub decision_authority_capped: bool,
    pub minimum_trust_tier: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readiness_blockers: Vec<GovernanceInfluenceFinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trust_blockers: Vec<GovernanceInfluenceFinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decision_blockers: Vec<GovernanceInfluenceFinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recovery_notes: Vec<GovernanceInfluenceFinding>,
}

/// Single influence finding for domain engines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceInfluenceFinding {
    pub factor: String,
    pub severity: String,
    pub message: String,
}

/// Compute governance influence for an entity from its governance metadata.
pub fn influence_for_entity(entity: &EntityRecord) -> GovernanceInfluence {
    // Derive readiness, trust, decision, and recovery constraints from governance.
    //
    // Parameters:
    // - `entity` — entity record with optional governance metadata
    //
    // Returns:
    // Influence flags and findings for domain engines.
    //
    // Options:
    // None.
    //
    // Example:
    // let influence = influence_for_entity(&entity);

    let gov = governance_from_entity(entity);
    let mut influence = GovernanceInfluence::default();

    let autonomy = gov.autonomy_level.unwrap_or(AutonomyLevel::Manual);
    let risk = gov.risk_level.unwrap_or(OperationalRisk::Negligible);
    let maturity = gov.operational_maturity.unwrap_or(DeploymentMaturity::Concept);
    let cert_status = gov
        .certification
        .as_ref()
        .map(|c| c.status)
        .unwrap_or(CertificationStatus::Draft);

    influence.minimum_trust_tier = autonomy.minimum_trust_tier().to_string();
    influence.requires_human_approval =
        autonomy.requires_human_approval() || risk.requires_human_approval();
    influence.requires_simulation = risk.requires_simulation();
    influence.recovery_escalation_required = risk >= OperationalRisk::High
        || matches!(
            risk,
            OperationalRisk::Critical
                | OperationalRisk::LifeCritical
                | OperationalRisk::MissionCritical
        );
    influence.decision_authority_capped = autonomy.level_number() >= 3;

    if maturity.allows_live_deployment() && !cert_status.is_operational() {
        influence.blocks_live_deployment = true;
        influence.readiness_blockers.push(GovernanceInfluenceFinding {
            factor: "governance.certification".into(),
            severity: "high".into(),
            message: format!(
                "Live maturity '{}' requires validated/certified status (current: {})",
                maturity.as_str(),
                cert_status.as_str()
            ),
        });
    }

    if maturity.allows_live_deployment() {
        let accountability_ok = gov
            .accountability
            .as_ref()
            .map(|a| a.is_complete_for_production())
            .unwrap_or(false);
        if !accountability_ok {
            influence.blocks_live_deployment = true;
            influence.readiness_blockers.push(GovernanceInfluenceFinding {
                factor: "governance.accountability".into(),
                severity: "high".into(),
                message: "Live deployment requires responsible person, deployment owner, and emergency contact"
                    .into(),
            });
        }
    }

    if risk.requires_human_approval() {
        let has_chain = gov
            .accountability
            .as_ref()
            .map(|a| !a.approval_chain.is_empty())
            .unwrap_or(false);
        if !has_chain {
            influence.decision_blockers.push(GovernanceInfluenceFinding {
                factor: "governance.approval_chain".into(),
                severity: "high".into(),
                message: format!(
                    "Risk '{}' requires an approval chain before autonomous decisions",
                    risk.as_str()
                ),
            });
        }
    }

    if autonomy.level_number() >= 3 {
        let trust_ok = matches!(
            entity.trust_status,
            EntityTrustStatus::Trusted | EntityTrustStatus::Verified
        );
        if !trust_ok {
            influence.trust_blockers.push(GovernanceInfluenceFinding {
                factor: "governance.autonomy_trust".into(),
                severity: "high".into(),
                message: format!(
                    "Autonomy '{}' requires trusted/verified posture (current: {})",
                    autonomy.as_str(),
                    entity.trust_status.as_str()
                ),
            });
            influence.readiness_blockers.push(GovernanceInfluenceFinding {
                factor: "governance.autonomy_trust".into(),
                severity: "high".into(),
                message: format!(
                    "Autonomy '{}' blocked until trust posture is trusted or verified",
                    autonomy.as_str()
                ),
            });
        }
    }

    if risk.requires_simulation() && maturity < DeploymentMaturity::Simulation {
        influence.readiness_blockers.push(GovernanceInfluenceFinding {
            factor: "governance.simulation".into(),
            severity: "medium".into(),
            message: "Medium+ risk requires at least simulation maturity".into(),
        });
    }

    if influence.recovery_escalation_required {
        influence.recovery_notes.push(GovernanceInfluenceFinding {
            factor: "governance.risk".into(),
            severity: "high".into(),
            message: format!(
                "Risk '{}' requires escalation contacts during recovery",
                risk.as_str()
            ),
        });
    }

    if influence.requires_human_approval {
        influence.decision_blockers.push(GovernanceInfluenceFinding {
            factor: "governance.human_approval".into(),
            severity: "medium".into(),
            message: format!(
                "Autonomy '{}' / risk '{}' requires human approval for autonomous actions",
                autonomy.as_str(),
                risk.as_str()
            ),
        });
    }

    influence
}

/// Whether an action requires human approval under governance rules.
///
/// Reflex/safety actions are never blocked. High/life/mission-critical risk
/// and manual/assisted autonomy require approval for other actions.
pub fn action_requires_human_approval(entity: &EntityRecord, action: &str) -> bool {
    let action_key = action.to_ascii_lowercase().replace(' ', "_");
    if is_reflex_or_safety_action(&action_key) {
        return false;
    }
    let gov = governance_from_entity(entity);
    let autonomy = gov.autonomy_level.unwrap_or(AutonomyLevel::Manual);
    let risk = gov.risk_level.unwrap_or(OperationalRisk::Negligible);

    if risk.requires_human_approval() {
        return true;
    }
    // Manual and assisted modes require human approval for non-reflex actions.
    matches!(autonomy, AutonomyLevel::Manual | AutonomyLevel::Assisted)
}

fn is_reflex_or_safety_action(action: &str) -> bool {
    const REFLEX: &[&str] = &[
        "emergency_stop",
        "stop_motor",
        "kill_switch",
        "estop",
        "e_stop",
        "cut_power",
        "safe_stop",
        "halt",
    ];
    REFLEX.iter().any(|name| action.contains(name))
}

/// Whether live deployment is blocked by governance for this entity.
pub fn blocks_live_deployment(entity: &EntityRecord) -> bool {
    influence_for_entity(entity).blocks_live_deployment
}
