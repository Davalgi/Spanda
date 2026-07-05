//! Runtime hooks — build autonomy context from entity health, trust, and telemetry hints.
//!
use crate::adaptive_recovery::{RecoveryHistory, AdaptiveRecoveryPolicy, compute_recovery_confidence};
use crate::damage_risk::{evaluate_damage_risk, RiskSignal};
use crate::homeostasis::StabilityMetric;
use crate::types::AutonomySeverity;
use spanda_config::entity::{
    EntityHealthStatus, EntityReadinessStatus, EntityRecord, EntityTrustStatus,
};

use super::entity::EntityAutonomyContext;

impl EntityAutonomyContext {
    /// Build runtime context from entity platform signals.
    pub fn from_entity(entity: &EntityRecord) -> Self {
        let mut metrics = stability_metrics_from_entity(entity);
        if metrics.is_empty() {
            metrics = default_platform_metrics();
        }
        let risk_signals = damage_risk_signals_from_entity(entity);
        Self {
            metrics,
            sensor_readings: Vec::new(),
            risk_signals,
            recovery_history: Vec::new(),
            fleet_id: entity.metadata.get("fleet_id").cloned(),
            region_id: entity.metadata.get("region_id").cloned(),
        }
    }

    /// Attach recovery history for adaptive confidence scoring.
    pub fn with_recovery_history(mut self, history: Vec<RecoveryHistory>) -> Self {
        self.recovery_history = history;
        self
    }
}

fn default_platform_metrics() -> Vec<StabilityMetric> {
    vec![
        StabilityMetric {
            name: "cpu_pct".into(),
            value: 45.0,
            unit: "pct".into(),
        },
        StabilityMetric {
            name: "memory_pct".into(),
            value: 55.0,
            unit: "pct".into(),
        },
    ]
}

fn stability_metrics_from_entity(entity: &EntityRecord) -> Vec<StabilityMetric> {
    let mut metrics = Vec::new();
    let cpu = match entity.health_status {
        EntityHealthStatus::Healthy => 35.0,
        EntityHealthStatus::Warning => 65.0,
        EntityHealthStatus::Degraded => 78.0,
        EntityHealthStatus::Critical => 92.0,
        EntityHealthStatus::Offline => 10.0,
        EntityHealthStatus::Unknown => 50.0,
    };
    metrics.push(StabilityMetric {
        name: "cpu_pct".into(),
        value: cpu,
        unit: "pct".into(),
    });
    let memory = match entity.readiness_status {
        EntityReadinessStatus::Ready => 50.0,
        EntityReadinessStatus::Partial => 72.0,
        EntityReadinessStatus::NotReady => 88.0,
        EntityReadinessStatus::Unknown => 60.0,
    };
    metrics.push(StabilityMetric {
        name: "memory_pct".into(),
        value: memory,
        unit: "pct".into(),
    });
    if let Some(battery) = entity.metadata.get("battery_pct").and_then(|v| v.parse().ok()) {
        metrics.push(StabilityMetric {
            name: "battery_pct".into(),
            value: battery,
            unit: "pct".into(),
        });
    }
    metrics
}

fn damage_risk_signals_from_entity(entity: &EntityRecord) -> Vec<RiskSignal> {
    let mut signals = Vec::new();
    if entity.health_status == EntityHealthStatus::Critical {
        signals.push(RiskSignal {
            name: "health_critical".into(),
            value: 1.0,
            threshold: 0.5,
            severity: AutonomySeverity::Critical,
        });
    }
    if entity.trust_status == EntityTrustStatus::Compromised {
        signals.push(RiskSignal {
            name: "trust_compromised".into(),
            value: 1.0,
            threshold: 0.5,
            severity: AutonomySeverity::High,
        });
    }
    if entity.metadata.get("motor_overheating") == Some(&"true".into()) {
        signals.push(RiskSignal {
            name: "motor_overheating".into(),
            value: 1.0,
            threshold: 0.8,
            severity: AutonomySeverity::Critical,
        });
    }
    signals
}

/// Compute recovery confidence score for an entity from orchestrator history.
pub fn recovery_confidence_from_history(
    entity_id: &str,
    history: &[RecoveryHistory],
) -> f64 {
    compute_recovery_confidence(
        entity_id,
        history,
        &AdaptiveRecoveryPolicy::platform_defaults(),
    )
    .score
}

/// Evaluate composite damage risk index for an entity record.
pub fn entity_damage_risk_index(entity: &EntityRecord) -> f64 {
    let signals = damage_risk_signals_from_entity(entity);
    if signals.is_empty() {
        return 0.0;
    }
    evaluate_damage_risk(&entity.id, &signals).index
}
