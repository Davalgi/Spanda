//! Cross-domain integration tests for the Cognitive & Resilience Architecture.
//!
use spanda_autonomy::attention::compute_attention_score;
use spanda_autonomy::types::AutonomySeverity;
use spanda_autonomy::{
    apply_habituation, attach_default_autonomy_profile, categorize_memory,
    compute_recovery_confidence, detect_signal_conflict, enrich_entity_autonomy,
    evaluate_damage_risk, evaluate_homeostasis, evaluate_quarantine_decision,
    evaluate_reflex_priority, fuse_observations, list_reflex_actions, rank_events,
    register_live_sensor_supplier, AdaptiveRecoveryPolicy, AttentionPolicy, ConfidencePolicy,
    EntityAutonomyContext, EventPriority, HabituationPolicy, HomeostasisPolicy, ImmunePolicy,
    RecoveryHistory, RepetitionPattern, RiskSignal, SensorConfidence, StabilityMetric,
};
use spanda_config::entity::{
    EntityHealthStatus, EntityKind, EntityReadinessStatus, EntityRecord, EntityTrustStatus,
};

fn sample_robot(id: &str, health: EntityHealthStatus, trust: EntityTrustStatus) -> EntityRecord {
    EntityRecord {
        id: id.into(),
        entity_type: EntityKind::Robot,
        health_status: health,
        readiness_status: EntityReadinessStatus::Ready,
        trust_status: trust,
        ..Default::default()
    }
}

/// Reflex + Homeostasis: unstable homeostasis should correlate with reflex-eligible entity state.
#[test]
fn reflex_and_homeostasis_interaction() {
    let entity = sample_robot(
        "robot-h",
        EntityHealthStatus::Degraded,
        EntityTrustStatus::Trusted,
    );
    let metrics = vec![StabilityMetric {
        name: "memory_pct".into(),
        value: 96.0,
        unit: "pct".into(),
    }];
    let homeo = evaluate_homeostasis(&entity, &metrics, &HomeostasisPolicy::platform_defaults());
    assert!(!homeo.stable);

    let reflexes = list_reflex_actions();
    let selected = evaluate_reflex_priority(&reflexes, "thermal");
    assert!(selected.is_some());
    assert!(selected.unwrap().priority >= 50);
}

/// Fusion + Readiness: conflicting sensors should fail confidence policy (readiness input).
#[test]
fn fusion_and_readiness_interaction() {
    let readings = vec![
        SensorConfidence {
            source: "gps".into(),
            value: 1.0,
            confidence: 0.95,
            timestamp: None,
        },
        SensorConfidence {
            source: "imu".into(),
            value: 99.0,
            confidence: 0.9,
            timestamp: None,
        },
    ];
    assert!(!detect_signal_conflict(&readings, 0.25).is_empty());
    let fused = fuse_observations("position", &readings, &ConfidencePolicy::default());
    assert!(!fused.confidence.meets_policy);

    let mut entity = sample_robot(
        "robot-f",
        EntityHealthStatus::Healthy,
        EntityTrustStatus::Trusted,
    );
    entity.readiness_status = EntityReadinessStatus::Partial;
    let ctx = EntityAutonomyContext {
        sensor_readings: readings,
        ..Default::default()
    };
    enrich_entity_autonomy(&mut entity, &ctx);
    let profile = entity.autonomy.as_ref().unwrap();
    assert!(profile
        .confidence
        .as_ref()
        .is_some_and(|c| !c.conflicts.is_empty()));
}

/// Immunity + Trust: untrusted entity triggers quarantine aligned with trust status.
#[test]
fn immunity_and_trust_interaction() {
    let entity = sample_robot(
        "robot-i",
        EntityHealthStatus::Healthy,
        EntityTrustStatus::Untrusted,
    );
    let decision = evaluate_quarantine_decision(&entity, &ImmunePolicy::platform_defaults());
    assert!(decision.quarantine);

    let mut registry = spanda_config::EntityRegistry::default();
    registry.entities.insert(
        "robot-tamper".into(),
        EntityRecord {
            id: "robot-tamper".into(),
            entity_type: EntityKind::Robot,
            trust_status: EntityTrustStatus::Trusted,
            metadata: [("tamper.detected".into(), "true".into())]
                .into_iter()
                .collect(),
            ..Default::default()
        },
    );
    spanda_autonomy::apply_registry_autonomy_profiles(&mut registry);
    let updated = registry.get("robot-tamper").unwrap();
    assert_eq!(updated.trust_status, EntityTrustStatus::Compromised);
    assert!(updated
        .autonomy
        .as_ref()
        .and_then(|a| a.immunity_status.as_ref())
        .is_some_and(|i| i.quarantined));
}

/// Attention + Recovery: recovery-related events rank above routine telemetry.
#[test]
fn attention_and_recovery_interaction() {
    let routine = compute_attention_score(
        "telemetry-1",
        "routine_telemetry",
        EventPriority::Routine,
        AutonomySeverity::Info,
    );
    let recovery_event = compute_attention_score(
        "recovery-1",
        "recovery_failed:robot-1",
        EventPriority::Urgent,
        AutonomySeverity::High,
    );
    let window = rank_events(vec![routine, recovery_event], &AttentionPolicy::default());
    assert_eq!(
        window.items.first().map(|i| i.event_id.as_str()),
        Some("recovery-1")
    );
}

/// Operational Memory + Replay: trace artifacts map to episodic memory category.
#[test]
fn fusion_entity_derived_sensor_readings() {
    let entity = sample_robot(
        "robot-fusion",
        EntityHealthStatus::Degraded,
        EntityTrustStatus::Trusted,
    );
    let readings = spanda_autonomy::sensor_readings_from_entity(&entity);
    assert!(readings.len() >= 3);
    let fused = fuse_observations("health_bundle", &readings, &ConfidencePolicy::default());
    assert!(fused.confidence.score > 0.0);
}

#[test]
fn operational_memory_and_replay_interaction() {
    assert_eq!(
        categorize_memory("trace"),
        spanda_autonomy::MemoryCategory::Episodic
    );
    assert_eq!(
        categorize_memory("replay"),
        spanda_autonomy::MemoryCategory::Episodic
    );

    let mut entity = sample_robot(
        "robot-m",
        EntityHealthStatus::Healthy,
        EntityTrustStatus::Trusted,
    );
    attach_default_autonomy_profile(&mut entity);
    let refs = entity
        .autonomy
        .as_ref()
        .and_then(|a| a.memory_refs.as_ref())
        .expect("memory refs");
    assert!(!refs.semantic.is_empty());
    assert!(!refs.procedural.is_empty());
    assert!(!refs.episodic.is_empty());
}

#[test]
fn registry_memory_refs_include_categories() {
    let mut registry = spanda_config::EntityRegistry::default();
    registry.entities.insert(
        "robot-mem".into(),
        EntityRecord {
            id: "robot-mem".into(),
            entity_type: EntityKind::Robot,
            ..Default::default()
        },
    );
    spanda_autonomy::apply_registry_autonomy_profiles(&mut registry);
    let refs = registry
        .get("robot-mem")
        .unwrap()
        .autonomy
        .as_ref()
        .and_then(|a| a.memory_refs.as_ref())
        .expect("memory refs");
    assert!(!refs.reflex.is_empty());
    assert!(refs.procedural.iter().any(|p| p.contains("playbook")));
}

/// Damage Risk + Mission Planning: elevated risk index implies protective action.
#[test]
fn damage_risk_and_mission_planning_interaction() {
    let signals = vec![RiskSignal {
        name: "motor_overheating".into(),
        value: 95.0,
        threshold: 80.0,
        severity: AutonomySeverity::Critical,
    }];
    let risk = evaluate_damage_risk("robot-d", &signals);
    assert!(risk.index > 0.0);
    assert!(!risk.protective_actions.is_empty());

    let mut entity = sample_robot(
        "robot-d",
        EntityHealthStatus::Critical,
        EntityTrustStatus::Trusted,
    );
    let ctx = EntityAutonomyContext {
        risk_signals: signals,
        ..Default::default()
    };
    enrich_entity_autonomy(&mut entity, &ctx);
    assert!(entity
        .autonomy
        .as_ref()
        .and_then(|a| a.damage_risk.as_ref())
        .is_some_and(|d| d.index > 0.0));
    assert!(entity
        .autonomy
        .as_ref()
        .and_then(|a| a.attention.as_ref())
        .is_some_and(|a| a.top_priority.is_some()));
}

/// Adaptive Recovery + Recovery Orchestrator: history drives confidence and strategy preference.
#[test]
fn adaptive_recovery_and_orchestrator_interaction() {
    let history = vec![
        RecoveryHistory {
            entity_id: "robot-r".into(),
            strategy: "reconnect_sensor".into(),
            success: true,
            duration_ms: 400,
        },
        RecoveryHistory {
            entity_id: "robot-r".into(),
            strategy: "reconnect_sensor".into(),
            success: true,
            duration_ms: 420,
        },
        RecoveryHistory {
            entity_id: "robot-r".into(),
            strategy: "reconnect_sensor".into(),
            success: true,
            duration_ms: 410,
        },
        RecoveryHistory {
            entity_id: "robot-r".into(),
            strategy: "restart_provider".into(),
            success: false,
            duration_ms: 3000,
        },
    ];
    let rc = compute_recovery_confidence(
        "robot-r",
        &history,
        &AdaptiveRecoveryPolicy::platform_defaults(),
    );
    assert!(rc.score > 0.5);
    assert_eq!(
        rc.preferred.as_ref().map(|p| p.strategy.as_str()),
        Some("reconnect_sensor")
    );

    let patterns = vec![RepetitionPattern {
        label: "recovery_failed".into(),
        count: 6,
        trend: "worsening".into(),
    }];
    let suppressions = apply_habituation(&patterns, &HabituationPolicy::default());
    assert!(suppressions
        .iter()
        .any(|s| !s.suppressed || s.label == "recovery_failed"));
}

/// Live fusion supplier merges automotive proxy readings when env-gated.
#[test]
fn live_fusion_sensor_supplier_merges_readings() {
    fn supplier(_entity_id: &str) -> Vec<(String, f64, f64)> {
        vec![("gps_proxy_radar".into(), 42.0, 0.9)]
    }
    register_live_sensor_supplier(supplier);
    std::env::set_var("SPANDA_LIVE_FUSION_SENSORS", "1");
    let entity = sample_robot(
        "robot-live",
        EntityHealthStatus::Healthy,
        EntityTrustStatus::Trusted,
    );
    let ctx = EntityAutonomyContext::from_entity(&entity);
    assert!(ctx
        .sensor_readings
        .iter()
        .any(|r| r.source == "gps_proxy_radar" && (r.value - 42.0).abs() < f64::EPSILON));
    std::env::remove_var("SPANDA_LIVE_FUSION_SENSORS");
}
