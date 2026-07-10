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

    /// Build a policy from `.sd` `@policy(kind: "homeostasis")` / `homeostasis_policy` metric names.
    ///
    /// Parameters:
    /// - `metric_names` — declared metric identifiers from the program AST
    ///
    /// Returns:
    /// Ranges filtered from `platform_defaults` for known names; unknown names get a
    /// conservative 0..100 range with 80% warn line. Empty input yields `platform_defaults()`.
    ///
    /// Options:
    /// None.
    ///
    /// Example:
    /// let policy = HomeostasisPolicy::from_declared_metrics(&["cpu_pct".into()]);
    pub fn from_declared_metrics(metric_names: &[String]) -> Self {
        // Fall back to full platform defaults when the program declares no metrics.
        if metric_names.is_empty() {
            return Self::platform_defaults();
        }
        let defaults = Self::platform_defaults();
        let mut ranges = Vec::new();

        // Map each declared metric to a known range or a conservative placeholder.
        for name in metric_names {
            if let Some(range) = defaults.ranges.iter().find(|r| &r.metric == name) {
                ranges.push(range.clone());
            } else {
                ranges.push(StabilityRange {
                    metric: name.clone(),
                    min: 0.0,
                    max: 100.0,
                    warn_pct: 0.8,
                });
            }
        }
        Self { ranges }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_declared_metrics_filters_defaults() {
        // Description:
        //     From declared metrics filters defaults.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        let policy = HomeostasisPolicy::from_declared_metrics(&[
            "cpu_pct".into(),
            "custom_metric".into(),
        ]);
        assert_eq!(policy.ranges.len(), 2);
        assert_eq!(policy.ranges[0].metric, "cpu_pct");
        assert_eq!(policy.ranges[0].max, 85.0);
        assert_eq!(policy.ranges[1].metric, "custom_metric");
        assert_eq!(policy.ranges[1].max, 100.0);
    }

    #[test]
    fn from_declared_metrics_empty_uses_platform_defaults() {
        // Description:
        //     From declared metrics empty uses platform defaults.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        let policy = HomeostasisPolicy::from_declared_metrics(&[]);
        assert_eq!(policy.ranges.len(), HomeostasisPolicy::platform_defaults().ranges.len());
    }
}
