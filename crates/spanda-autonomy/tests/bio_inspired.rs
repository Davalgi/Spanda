//! Integration tests for bio-inspired resilient autonomy.
//!
use spanda_autonomy::types::AutonomySeverity;
use spanda_autonomy::{
    apply_habituation, apply_sensitization, attach_default_autonomy_profile, categorize_memory,
    compute_recovery_confidence, detect_signal_conflict, enrich_entity_autonomy,
    evaluate_damage_risk, evaluate_homeostasis, evaluate_quarantine_decision,
    evaluate_reflex_priority, fuse_observations, list_reflex_actions, AdaptiveRecoveryPolicy,
    ConfidencePolicy, EntityAutonomyContext, HabituationPolicy, HomeostasisPolicy, ImmunePolicy,
    MemoryCategory, RecoveryHistory, RepetitionPattern, RiskSignal, SensitizationPolicy,
    SensorConfidence, StabilityMetric,
};
use spanda_config::entity::{
    EntityHealthStatus, EntityKind, EntityReadinessStatus, EntityRecord, EntityTrustStatus,
};

fn sample_entity(id: &str, trust: EntityTrustStatus) -> EntityRecord {
    EntityRecord {
        id: id.into(),
        entity_type: EntityKind::Robot,
        health_status: EntityHealthStatus::Healthy,
        readiness_status: EntityReadinessStatus::Ready,
        trust_status: trust,
        ..Default::default()
    }
}

#[test]
fn reflex_action_priority_selects_highest() {
    let actions = list_reflex_actions();
    let selected = evaluate_reflex_priority(&actions, "emergency");
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().id, "reflex.emergency_stop");
}

#[test]
fn homeostasis_threshold_evaluation_detects_drift() {
    let entity = sample_entity("robot-1", EntityTrustStatus::Trusted);
    let metrics = vec![
        StabilityMetric {
            name: "memory_pct".into(),
            value: 95.0,
            unit: "pct".into(),
        },
        StabilityMetric {
            name: "cpu_pct".into(),
            value: 40.0,
            unit: "pct".into(),
        },
    ];
    let report = evaluate_homeostasis(&entity, &metrics, &HomeostasisPolicy::platform_defaults());
    assert!(!report.stable);
    assert!(report
        .drift_signals
        .iter()
        .any(|d| d.metric == "memory_pct"));
    assert!(report
        .corrections
        .iter()
        .any(|c| c.action == "restart_low_risk_provider"));
}

#[test]
fn immunity_quarantine_decision_for_untrusted() {
    let entity = sample_entity("device-1", EntityTrustStatus::Untrusted);
    let decision = evaluate_quarantine_decision(&entity, &ImmunePolicy::platform_defaults());
    assert!(decision.quarantine);
    assert!(!decision.reasons.is_empty());
}

#[test]
fn confidence_conflict_detection() {
    let readings = vec![
        SensorConfidence {
            source: "gps".into(),
            value: 10.0,
            confidence: 0.9,
            timestamp: None,
        },
        SensorConfidence {
            source: "imu".into(),
            value: 50.0,
            confidence: 0.8,
            timestamp: None,
        },
    ];
    let conflicts = detect_signal_conflict(&readings, 0.25);
    assert!(!conflicts.is_empty());
    let fused = fuse_observations("position", &readings, &ConfidencePolicy::default());
    assert!(!fused.confidence.meets_policy);
}

#[test]
fn alert_habituation_and_sensitization() {
    let patterns = vec![
        RepetitionPattern {
            label: "routine_telemetry".into(),
            count: 100,
            trend: "stable".into(),
        },
        RepetitionPattern {
            label: "network_glitch".into(),
            count: 8,
            trend: "worsening".into(),
        },
    ];
    let suppressions = apply_habituation(&patterns, &HabituationPolicy::default());
    assert!(suppressions.iter().any(|s| s.suppressed));
    let escalations = apply_sensitization(&patterns, &SensitizationPolicy::default());
    assert!(escalations.iter().any(|e| e.escalated));
}

#[test]
fn recovery_confidence_calculation() {
    let history = vec![
        RecoveryHistory {
            entity_id: "robot-1".into(),
            strategy: "reconnect_camera".into(),
            success: true,
            duration_ms: 500,
        },
        RecoveryHistory {
            entity_id: "robot-1".into(),
            strategy: "reconnect_camera".into(),
            success: true,
            duration_ms: 600,
        },
        RecoveryHistory {
            entity_id: "robot-1".into(),
            strategy: "reconnect_camera".into(),
            success: true,
            duration_ms: 550,
        },
        RecoveryHistory {
            entity_id: "robot-1".into(),
            strategy: "restart_provider".into(),
            success: false,
            duration_ms: 2000,
        },
    ];
    let rc = compute_recovery_confidence(
        "robot-1",
        &history,
        &AdaptiveRecoveryPolicy::platform_defaults(),
    );
    assert!(rc.score > 0.5);
    assert_eq!(
        rc.preferred.as_ref().map(|p| p.strategy.as_str()),
        Some("reconnect_camera")
    );
}

#[test]
fn memory_category_mapping() {
    assert_eq!(categorize_memory("trace"), MemoryCategory::Episodic);
    assert_eq!(categorize_memory("playbook"), MemoryCategory::Procedural);
    assert_eq!(categorize_memory("entity_graph"), MemoryCategory::Semantic);
    assert_eq!(categorize_memory("safety_reflex"), MemoryCategory::Reflex);
}

#[test]
fn damage_risk_severity() {
    let signals = vec![RiskSignal {
        name: "motor_overheating".into(),
        value: 95.0,
        threshold: 80.0,
        severity: AutonomySeverity::Critical,
    }];
    let risk = evaluate_damage_risk("robot-1", &signals);
    assert!(risk.index > 0.0);
    assert!(!risk.protective_actions.is_empty());
}

#[test]
fn entity_integration_autonomy_profile() {
    let mut entity = sample_entity("robot-1", EntityTrustStatus::Trusted);
    attach_default_autonomy_profile(&mut entity);
    assert!(entity.autonomy.is_some());
    assert!(!entity.autonomy.as_ref().unwrap().reflexes.is_empty());

    let ctx = EntityAutonomyContext {
        metrics: vec![StabilityMetric {
            name: "battery_pct".into(),
            value: 10.0,
            unit: "pct".into(),
        }],
        risk_signals: vec![RiskSignal {
            name: "battery_swelling".into(),
            value: 1.0,
            threshold: 0.5,
            severity: AutonomySeverity::High,
        }],
        ..Default::default()
    };
    enrich_entity_autonomy(&mut entity, &ctx);
    let profile = entity.autonomy.as_ref().unwrap();
    assert!(profile.homeostasis.as_ref().is_some_and(|h| !h.stable));
    assert!(profile.damage_risk.as_ref().is_some_and(|d| d.index > 0.0));
}

#[test]
fn registry_autonomy_profiles_applied() {
    let mut registry = spanda_config::EntityRegistry::default();
    registry.entities.insert(
        "robot-1".into(),
        EntityRecord {
            id: "robot-1".into(),
            entity_type: EntityKind::Robot,
            health_status: EntityHealthStatus::Degraded,
            readiness_status: EntityReadinessStatus::Ready,
            trust_status: EntityTrustStatus::Trusted,
            ..Default::default()
        },
    );
    spanda_autonomy::apply_registry_autonomy_profiles(&mut registry);
    let entity = registry.get("robot-1").unwrap();
    assert!(entity.autonomy.is_some());
    assert!(!entity.autonomy.as_ref().unwrap().reflexes.is_empty());
    assert!(entity.autonomy.as_ref().unwrap().homeostasis.is_some());
}

#[test]
fn runtime_reflex_trace_buffer_records_hint() {
    spanda_autonomy::record_runtime_reflex(
        "robot-trace-test",
        "obstacle",
        "lidar.nearest_distance",
        "brake_and_hold",
    );
    let traces = spanda_autonomy::list_recorded_reflex_traces();
    assert!(
        traces
            .iter()
            .any(|t| t.entity_id == "robot-trace-test" && t.reflex_id.contains("obstacle"))
    );
}
