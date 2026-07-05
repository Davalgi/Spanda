//! Runtime hooks — build autonomy context from entity health, trust, and telemetry hints.
//!
use std::sync::{LazyLock, Mutex};

use crate::adaptive_recovery::{
    compute_recovery_confidence, AdaptiveRecoveryPolicy, RecoveryHistory,
};
use crate::damage_risk::{evaluate_damage_risk, RiskSignal};
use crate::homeostasis::StabilityMetric;
use crate::types::AutonomySeverity;
use spanda_config::entity::{
    EntityHealthStatus, EntityReadinessStatus, EntityRecord, EntityTrustStatus,
};

use super::entity::EntityAutonomyContext;

/// Scheduler/task telemetry snapshot published by the interpreter after run/sim.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlatformTelemetrySnapshot {
    pub scheduler_ticks: u64,
    pub emergency_stops: u64,
    pub deadline_misses: u64,
    pub avg_task_duration_ms: f64,
    pub provider_failures: u64,
}

static PLATFORM_TELEMETRY: LazyLock<Mutex<Option<PlatformTelemetrySnapshot>>> =
    LazyLock::new(|| Mutex::new(None));

/// Publish the latest platform telemetry snapshot for homeostasis evaluation.
pub fn update_platform_telemetry(snapshot: PlatformTelemetrySnapshot) {
    *PLATFORM_TELEMETRY
        .lock()
        .expect("platform telemetry lock poisoned") = Some(snapshot);
}

/// Read the latest platform telemetry snapshot, if any.
pub fn platform_telemetry_snapshot() -> Option<PlatformTelemetrySnapshot> {
    PLATFORM_TELEMETRY
        .lock()
        .expect("platform telemetry lock poisoned")
        .clone()
}

impl EntityAutonomyContext {
    /// Build runtime context from entity platform signals.
    pub fn from_entity(entity: &EntityRecord) -> Self {
        let mut metrics = stability_metrics_from_entity(entity);
        if let Some(telemetry) = platform_telemetry_snapshot() {
            metrics.extend(stability_metrics_from_telemetry(&telemetry));
        }
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

fn stability_metrics_from_telemetry(snapshot: &PlatformTelemetrySnapshot) -> Vec<StabilityMetric> {
    let load_pct = (snapshot.avg_task_duration_ms / 10.0).clamp(0.0, 100.0);
    let deadline_stress = (snapshot.deadline_misses as f64 * 5.0).clamp(0.0, 100.0);
    vec![
        StabilityMetric {
            name: "scheduler_ticks".into(),
            value: snapshot.scheduler_ticks as f64,
            unit: "ticks".into(),
        },
        StabilityMetric {
            name: "runtime_load_pct".into(),
            value: load_pct,
            unit: "pct".into(),
        },
        StabilityMetric {
            name: "deadline_miss_pct".into(),
            value: deadline_stress,
            unit: "pct".into(),
        },
        StabilityMetric {
            name: "provider_failures".into(),
            value: snapshot.provider_failures as f64,
            unit: "count".into(),
        },
        StabilityMetric {
            name: "emergency_stops".into(),
            value: snapshot.emergency_stops as f64,
            unit: "count".into(),
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
    if entity.metadata.get("tamper.detected") == Some(&"true".into()) {
        signals.push(RiskSignal {
            name: "tamper_detected".into(),
            value: 1.0,
            threshold: 0.5,
            severity: AutonomySeverity::High,
        });
    }
    signals
}

/// Compute recovery confidence score for an entity from orchestrator history.
pub fn recovery_confidence_from_history(entity_id: &str, history: &[RecoveryHistory]) -> f64 {
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
