//! Habituation and sensitization — alert fatigue management.
//!
use serde::{Deserialize, Serialize};

/// Policy for suppressing repeated harmless alerts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HabituationPolicy {
    pub max_repeats: u32,
    pub harmless_labels: Vec<String>,
}

impl Default for HabituationPolicy {
    fn default() -> Self {
        Self {
            max_repeats: 10,
            harmless_labels: vec![
                "routine_telemetry".into(),
                "network_glitch".into(),
                "low_battery_warning".into(),
            ],
        }
    }
}

/// Policy for escalating worsening or repeated issues.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensitizationPolicy {
    pub escalation_threshold: u32,
    pub worsening_factor: f64,
}

impl Default for SensitizationPolicy {
    fn default() -> Self {
        Self {
            escalation_threshold: 5,
            worsening_factor: 1.5,
        }
    }
}

/// Suppression record for habituated alerts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertSuppression {
    pub label: String,
    pub count: u32,
    pub suppressed: bool,
}

/// Escalation record for sensitized alerts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertEscalation {
    pub label: String,
    pub count: u32,
    pub escalated: bool,
    pub recommendation: String,
}

/// Repetition pattern detected in alert stream.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepetitionPattern {
    pub label: String,
    pub count: u32,
    pub trend: String,
}

/// Alert fatigue metric summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlertFatigueMetric {
    pub total_alerts: u32,
    pub suppressed_count: u32,
    pub escalated_count: u32,
    pub fatigue_index: f64,
}

/// Apply habituation — suppress repeated harmless alerts.
pub fn apply_habituation(
    patterns: &[RepetitionPattern],
    policy: &HabituationPolicy,
) -> Vec<AlertSuppression> {
    patterns
        .iter()
        .map(|p| {
            let harmless = policy
                .harmless_labels
                .iter()
                .any(|l| p.label.contains(l.as_str()));
            let suppressed = harmless && p.count >= policy.max_repeats;
            AlertSuppression {
                label: p.label.clone(),
                count: p.count,
                suppressed,
            }
        })
        .collect()
}

/// Apply sensitization — escalate worsening or repeated issues.
pub fn apply_sensitization(
    patterns: &[RepetitionPattern],
    policy: &SensitizationPolicy,
) -> Vec<AlertEscalation> {
    patterns
        .iter()
        .map(|p| {
            let escalated =
                p.count >= policy.escalation_threshold || (p.trend == "worsening" && p.count >= 2);
            let recommendation = if p.label.contains("recovery") && escalated {
                "create_incident".into()
            } else if p.label.contains("battery") && escalated {
                "maintenance_recommendation".into()
            } else if escalated {
                "escalate_to_operator".into()
            } else {
                "monitor".into()
            };
            AlertEscalation {
                label: p.label.clone(),
                count: p.count,
                escalated,
                recommendation,
            }
        })
        .collect()
}

/// Analyze alert fatigue from patterns.
pub fn analyze_alert_fatigue(
    suppressions: &[AlertSuppression],
    escalations: &[AlertEscalation],
) -> AlertFatigueMetric {
    let total = suppressions.len() as u32 + escalations.len() as u32;
    let suppressed_count = suppressions.iter().filter(|s| s.suppressed).count() as u32;
    let escalated_count = escalations.iter().filter(|e| e.escalated).count() as u32;
    let fatigue_index = if total == 0 {
        0.0
    } else {
        (suppressed_count as f64 + escalated_count as f64 * 2.0) / total as f64
    };
    AlertFatigueMetric {
        total_alerts: total,
        suppressed_count,
        escalated_count,
        fatigue_index,
    }
}
