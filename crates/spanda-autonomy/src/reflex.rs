//! Reflex arc — immediate local safety response.
//!
use crate::types::AutonomySeverity;
use serde::{Deserialize, Serialize};
use spanda_decision::DecisionLayer;

/// A registered reflex action with priority and safety category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReflexAction {
    pub id: String,
    pub name: String,
    pub trigger: String,
    pub action: String,
    pub priority: u8,
    pub severity: AutonomySeverity,
    pub enabled: bool,
}

/// Reflex controller binding sensor/detector to immediate safe action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReflexController {
    pub id: String,
    pub entity_id: String,
    pub sensor: String,
    pub actions: Vec<ReflexAction>,
}

/// Complete reflex arc: sensor → controller → action → audit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReflexArc {
    pub controller: ReflexController,
    pub layer: DecisionLayer,
    pub audit_required: bool,
    pub notify_control_center: bool,
}

/// Trace entry for reflex simulation and replay.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReflexTrace {
    pub reflex_id: String,
    pub entity_id: String,
    pub trigger: String,
    pub action_taken: String,
    pub timestamp: String,
    pub priority: u8,
}

/// Default reflex actions mapped from distributed decision layer 0 patterns.
pub fn list_reflex_actions() -> Vec<ReflexAction> {
    vec![
        ReflexAction {
            id: "reflex.emergency_stop".into(),
            name: "Emergency stop".into(),
            trigger: "kill_switch | collision_imminent".into(),
            action: "halt_actuators".into(),
            priority: 255,
            severity: AutonomySeverity::Critical,
            enabled: true,
        },
        ReflexAction {
            id: "reflex.obstacle".into(),
            name: "Obstacle imminent".into(),
            trigger: "lidar.nearest_distance < safety.min_distance".into(),
            action: "brake_and_hold".into(),
            priority: 200,
            severity: AutonomySeverity::Critical,
            enabled: true,
        },
        ReflexAction {
            id: "reflex.overcurrent".into(),
            name: "Actuator overcurrent".into(),
            trigger: "actuator.current > actuator.max_current".into(),
            action: "disable_actuator".into(),
            priority: 190,
            severity: AutonomySeverity::High,
            enabled: true,
        },
        ReflexAction {
            id: "reflex.thermal".into(),
            name: "Thermal runaway".into(),
            trigger: "temperature > thermal.limit".into(),
            action: "reduce_load_and_cool".into(),
            priority: 180,
            severity: AutonomySeverity::High,
            enabled: true,
        },
        ReflexAction {
            id: "reflex.untrusted_cmd".into(),
            name: "Untrusted command rejected".into(),
            trigger: "command.trust != trusted".into(),
            action: "reject_command".into(),
            priority: 170,
            severity: AutonomySeverity::High,
            enabled: true,
        },
        ReflexAction {
            id: "reflex.unsafe_actuator".into(),
            name: "Unsafe actuator request blocked".into(),
            trigger: "safety.validate(proposal) == fail".into(),
            action: "block_actuator".into(),
            priority: 160,
            severity: AutonomySeverity::High,
            enabled: true,
        },
    ]
}

/// Select highest-priority enabled reflex for a trigger pattern match.
pub fn evaluate_reflex_priority<'a>(
    actions: &'a [ReflexAction],
    trigger_hint: &str,
) -> Option<&'a ReflexAction> {
    let hint = trigger_hint.to_lowercase();
    actions
        .iter()
        .filter(|a| a.enabled)
        .filter(|a| {
            a.trigger.to_lowercase().contains(&hint)
                || a.name.to_lowercase().contains(&hint)
                || hint.is_empty()
        })
        .max_by_key(|a| a.priority)
}
