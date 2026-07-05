//! Release-hardening security regressions for recovery authorization.

use spanda_config::entity::{EntityHealthStatus, EntityKind, EntityRecord, EntityRegistry};
use spanda_recovery::{decide_recovery, EntityRecoveryPolicy, RecoveryEscalationLevel};

fn robot(id: &str) -> EntityRecord {
    EntityRecord {
        id: id.into(),
        entity_type: EntityKind::Robot,
        health_status: EntityHealthStatus::Degraded,
        ..EntityRecord::default()
    }
}

#[test]
fn recovery_privilege_escalation_requires_approval_for_safety_failures() {
    // Safety failures must not auto-authorize without approval.
    let entity = robot("rover-001");
    let registry = EntityRegistry::default();
    let policies = vec![EntityRecoveryPolicy {
        entity_id: "rover-001".into(),
        requires_approval: true,
        max_escalation_level: RecoveryEscalationLevel::Level7HumanIntervention,
        ..EntityRecoveryPolicy::default()
    }];
    let decision = decide_recovery(
        &entity,
        "safety_critical_brake_failure",
        &policies,
        &registry,
        None,
    );
    assert!(
        !decision.automatic,
        "safety failure must not auto-recover: {decision:?}"
    );
    assert!(
        decision
            .explanations
            .iter()
            .any(|e| e.to_lowercase().contains("approval") || e.to_lowercase().contains("safety")),
        "expected approval/safety explanation: {:?}",
        decision.explanations
    );
}

#[test]
fn recovery_decision_does_not_leak_secrets() {
    // Recovery decisions must not embed secret material.
    let entity = robot("rover-001");
    let registry = EntityRegistry::default();
    let decision = decide_recovery(&entity, "gps", &[], &registry, None);
    let blob = format!("{decision:?}");
    assert!(!blob.to_lowercase().contains("password"));
    assert!(!blob.to_lowercase().contains("private_key"));
    assert!(!blob.contains("SPANDA_API_KEY"));
}
