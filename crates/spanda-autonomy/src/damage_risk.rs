//! Damage-risk model — harm potential, not just errors.
//!
use crate::types::AutonomySeverity;
use serde::{Deserialize, Serialize};

/// Risk signal indicating potential physical or operational harm.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskSignal {
    pub name: String,
    pub value: f64,
    pub threshold: f64,
    pub severity: AutonomySeverity,
}

/// Harm potential assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmPotential {
    pub description: String,
    pub severity: AutonomySeverity,
}

/// Protective action to reduce damage risk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProtectiveAction {
    pub action: String,
    pub rationale: String,
}

/// Composite damage risk for an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageRisk {
    pub entity_id: String,
    pub index: f64,
    pub signals: Vec<RiskSignal>,
    pub harm: Vec<HarmPotential>,
    pub protective_actions: Vec<ProtectiveAction>,
}

/// Safety pain index — normalized harm-risk score (0.0–1.0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyPainIndex {
    pub entity_id: String,
    pub index: f64,
    pub level: AutonomySeverity,
}

/// Evaluate damage risk from risk signals.
pub fn evaluate_damage_risk(entity_id: &str, signals: &[RiskSignal]) -> DamageRisk {
    let mut index = 0.0_f64;
    let mut harm = Vec::new();
    let mut protective_actions = Vec::new();

    for signal in signals {
        if signal.value >= signal.threshold {
            let overshoot = (signal.value - signal.threshold) / signal.threshold.max(1.0);
            let weight = match signal.severity {
                AutonomySeverity::Critical => 1.0,
                AutonomySeverity::High => 0.75,
                AutonomySeverity::Medium => 0.5,
                AutonomySeverity::Low => 0.25,
                AutonomySeverity::Info => 0.1,
            };
            index = index.max((overshoot * weight).min(1.0));
            harm.push(HarmPotential {
                description: format!("{} exceeded threshold", signal.name),
                severity: signal.severity,
            });
            if let Some(action) = suggest_protective_action(&signal.name) {
                protective_actions.push(ProtectiveAction {
                    action: action.0.into(),
                    rationale: action.1.into(),
                });
            }
        }
    }

    DamageRisk {
        entity_id: entity_id.into(),
        index,
        signals: signals.to_vec(),
        harm,
        protective_actions,
    }
}

fn suggest_protective_action(signal: &str) -> Option<(&'static str, &'static str)> {
    match signal {
        "motor_temperature" | "motor_overheating" => {
            Some(("degraded_mode", "Reduce motor load to prevent overheating"))
        }
        "vibration" | "excessive_vibration" => {
            Some(("halt_motion", "Stop motion until inspection"))
        }
        "battery_swelling" => Some(("mission_abort", "Battery swelling risk requires abort")),
        "operator_fatigue" => Some(("operator_alert", "Alert operator to rest")),
        "brake_degradation" => Some(("preventive_maintenance", "Schedule brake service")),
        _ => Some(("investigate", "Review signal and apply playbook")),
    }
}
