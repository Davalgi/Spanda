//! Homeostasis — maintain safe operating range before failures.
//!
use serde::{Deserialize, Serialize};
use spanda_config::entity::{EntityHealthStatus, EntityReadinessStatus, EntityRecord};

/// Metric monitored for stability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StabilityMetric {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

/// Acceptable range for a stability metric.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StabilityRange {
    pub metric: String,
    pub min: f64,
    pub max: f64,
    pub warn_pct: f64,
}

/// Drift signal when a metric trends toward boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriftSignal {
    pub metric: String,
    pub direction: String,
    pub severity: String,
}

/// Corrective action suggested by homeostasis evaluation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrectionAction {
    pub metric: String,
    pub action: String,
    pub rationale: String,
}

/// Homeostasis policy with stability ranges.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HomeostasisPolicy {
    pub ranges: Vec<StabilityRange>,
}

impl HomeostasisPolicy {
    pub fn platform_defaults() -> Self {
        Self {
            ranges: vec![
                StabilityRange {
                    metric: "cpu_pct".into(),
                    min: 0.0,
                    max: 85.0,
                    warn_pct: 0.75,
                },
                StabilityRange {
                    metric: "memory_pct".into(),
                    min: 0.0,
                    max: 90.0,
                    warn_pct: 0.8,
                },
                StabilityRange {
                    metric: "battery_pct".into(),
                    min: 15.0,
                    max: 100.0,
                    warn_pct: 0.2,
                },
                StabilityRange {
                    metric: "temperature_c".into(),
                    min: -10.0,
                    max: 75.0,
                    warn_pct: 0.85,
                },
                StabilityRange {
                    metric: "latency_ms".into(),
                    min: 0.0,
                    max: 500.0,
                    warn_pct: 0.8,
                },
            ],
        }
    }
}

/// Homeostasis evaluation report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StabilityReport {
    pub entity_id: String,
    pub stable: bool,
    pub metrics: Vec<StabilityMetric>,
    pub drift_signals: Vec<DriftSignal>,
    pub corrections: Vec<CorrectionAction>,
}

/// Evaluate homeostasis for an entity given current metrics.
pub fn evaluate_homeostasis(
    entity: &EntityRecord,
    metrics: &[StabilityMetric],
    policy: &HomeostasisPolicy,
) -> StabilityReport {
    let mut drift_signals = Vec::new();
    let mut corrections = Vec::new();

    for metric in metrics {
        if let Some(range) = policy.ranges.iter().find(|r| r.metric == metric.name) {
            if metric.value > range.max {
                drift_signals.push(DriftSignal {
                    metric: metric.name.clone(),
                    direction: "above_max".into(),
                    severity: "high".into(),
                });
                corrections.push(CorrectionAction {
                    metric: metric.name.clone(),
                    action: suggest_correction(&metric.name, "high"),
                    rationale: format!(
                        "{} = {} exceeds max {}",
                        metric.name, metric.value, range.max
                    ),
                });
            } else if metric.value < range.min {
                drift_signals.push(DriftSignal {
                    metric: metric.name.clone(),
                    direction: "below_min".into(),
                    severity: "medium".into(),
                });
                corrections.push(CorrectionAction {
                    metric: metric.name.clone(),
                    action: suggest_correction(&metric.name, "low"),
                    rationale: format!(
                        "{} = {} below min {}",
                        metric.name, metric.value, range.min
                    ),
                });
            } else {
                let span = range.max - range.min;
                let warn_line = range.min + span * range.warn_pct;
                if metric.value >= warn_line && metric.name != "battery_pct" {
                    drift_signals.push(DriftSignal {
                        metric: metric.name.clone(),
                        direction: "approaching_limit".into(),
                        severity: "low".into(),
                    });
                }
            }
        }
    }

    let health_stable = !matches!(
        entity.health_status,
        EntityHealthStatus::Critical | EntityHealthStatus::Offline
    );
    let readiness_stable = !matches!(entity.readiness_status, EntityReadinessStatus::NotReady);
    let stable = drift_signals.is_empty() && health_stable && readiness_stable;

    StabilityReport {
        entity_id: entity.id.clone(),
        stable,
        metrics: metrics.to_vec(),
        drift_signals,
        corrections,
    }
}

fn suggest_correction(metric: &str, level: &str) -> String {
    match (metric, level) {
        ("memory_pct", _) => "restart_low_risk_provider".into(),
        ("temperature_c", "high") => "reduce_workload".into(),
        ("battery_pct", "low") => "replan_mission".into(),
        ("latency_ms", _) => "switch_transport".into(),
        ("cpu_pct", _) => "reduce_workload".into(),
        _ => "investigate".into(),
    }
}
