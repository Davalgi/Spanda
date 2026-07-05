//! Entity integration for bio-inspired autonomy profiles.
//!
use crate::adaptive_recovery::{
    compute_recovery_confidence, AdaptiveRecoveryPolicy, RecoveryHistory,
};
use crate::damage_risk::{evaluate_damage_risk, RiskSignal};
use crate::fusion::{fuse_observations, ConfidencePolicy, SensorConfidence};
use crate::homeostasis::{evaluate_homeostasis, HomeostasisPolicy, StabilityMetric};
use crate::immunity::{evaluate_quarantine_decision, ImmunePolicy};
use crate::reflex::{list_reflex_actions, ReflexAction};
use spanda_config::entity::EntityRecord;
use spanda_config::entity_autonomy::{
    EntityAutonomyProfile, EntityConfidenceSnapshot, EntityDamageRisk, EntityHomeostasisSnapshot,
    EntityImmunityStatus, EntityMemoryRefs, EntityRecoveryConfidence, EntityReflexSummary,
};

/// Context for enriching entity autonomy from platform state.
#[derive(Debug, Clone, Default)]
pub struct EntityAutonomyContext {
    pub metrics: Vec<StabilityMetric>,
    pub sensor_readings: Vec<SensorConfidence>,
    pub risk_signals: Vec<RiskSignal>,
    pub recovery_history: Vec<RecoveryHistory>,
    pub fleet_id: Option<String>,
    pub region_id: Option<String>,
}

/// Attach default autonomy profile to an entity record.
pub fn attach_default_autonomy_profile(entity: &mut EntityRecord) {
    if entity.autonomy.is_none() {
        entity.autonomy = Some(default_profile_for_entity(entity));
    }
}

/// Enrich entity autonomy profile from runtime context.
pub fn enrich_entity_autonomy(entity: &mut EntityRecord, ctx: &EntityAutonomyContext) {
    attach_default_autonomy_profile(entity);

    let entity_id = entity.id.clone();
    let entity_snapshot = entity.clone();

    let homeostasis = evaluate_homeostasis(
        &entity_snapshot,
        &ctx.metrics,
        &HomeostasisPolicy::platform_defaults(),
    );
    let immunity =
        evaluate_quarantine_decision(&entity_snapshot, &ImmunePolicy::platform_defaults());

    let confidence = if ctx.sensor_readings.is_empty() {
        None
    } else {
        let fused = fuse_observations(
            "entity_state",
            &ctx.sensor_readings,
            &ConfidencePolicy::default(),
        );
        Some(EntityConfidenceSnapshot {
            score: fused.confidence.score,
            conflicts: fused
                .conflicts
                .iter()
                .map(|c| c.description.clone())
                .collect(),
            sources: ctx
                .sensor_readings
                .iter()
                .map(|r| r.source.clone())
                .collect(),
        })
    };

    let damage_risk = if ctx.risk_signals.is_empty() {
        None
    } else {
        let risk = evaluate_damage_risk(&entity_id, &ctx.risk_signals);
        Some(EntityDamageRisk {
            index: risk.index,
            risk_signals: risk.signals.iter().map(|s| s.name.clone()).collect(),
            protective_action: risk.protective_actions.first().map(|a| a.action.clone()),
        })
    };

    let recovery_confidence = if ctx.recovery_history.is_empty() {
        None
    } else {
        let rc = compute_recovery_confidence(
            &entity_id,
            &ctx.recovery_history,
            &AdaptiveRecoveryPolicy::platform_defaults(),
        );
        Some(EntityRecoveryConfidence {
            score: rc.score,
            preferred_strategy: rc.preferred.as_ref().map(|p| p.strategy.clone()),
            attempts: rc.rates.iter().map(|r| r.attempts).sum(),
        })
    };

    if let Some(profile) = entity.autonomy.as_mut() {
        profile.homeostasis = Some(EntityHomeostasisSnapshot {
            stable: homeostasis.stable,
            drift_signals: homeostasis
                .drift_signals
                .iter()
                .map(|d| format!("{}:{}", d.metric, d.direction))
                .collect(),
            last_report_at: None,
        });
        profile.confidence = confidence;
        profile.immunity_status = Some(EntityImmunityStatus {
            quarantined: immunity.quarantine,
            threat_level: if immunity.quarantine {
                Some("elevated".into())
            } else {
                None
            },
            violations: immunity.reasons,
        });
        profile.damage_risk = damage_risk;
        profile.recovery_confidence = recovery_confidence;
    }
}

fn default_profile_for_entity(entity: &EntityRecord) -> EntityAutonomyProfile {
    let reflexes: Vec<EntityReflexSummary> = list_reflex_actions()
        .into_iter()
        .map(|r| reflex_to_summary(&r))
        .collect();
    EntityAutonomyProfile {
        reflexes,
        confidence: None,
        homeostasis: None,
        immunity_status: None,
        memory_refs: Some(EntityMemoryRefs {
            semantic: vec![format!("entity:{}", entity.id)],
            ..Default::default()
        }),
        damage_risk: None,
        recovery_confidence: None,
        metadata: Default::default(),
    }
}

fn reflex_to_summary(action: &ReflexAction) -> EntityReflexSummary {
    EntityReflexSummary {
        id: action.id.clone(),
        name: action.name.clone(),
        priority: action.priority,
        enabled: action.enabled,
        last_triggered_at: None,
    }
}
