//! Maintenance and sleep mode — low-risk operational windows.
//!
use serde::{Deserialize, Serialize};

/// Scheduled maintenance window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    pub id: String,
    pub start: String,
    pub end: String,
    pub activities: Vec<String>,
}

/// Sleep mode — minimal activity state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SleepMode {
    Active,
    LowActivity,
    Sleep,
    Maintenance,
}

/// Low-activity mode descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowActivityMode {
    pub entity_id: String,
    pub mode: SleepMode,
    pub allowed_tasks: Vec<String>,
}

/// Scheduled recovery during maintenance window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduledRecovery {
    pub entity_id: String,
    pub window_id: String,
    pub playbook_id: String,
}

/// Calibration window for sensors and actuators.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationWindow {
    pub entity_id: String,
    pub sensors: Vec<String>,
    pub scheduled_at: String,
}

/// Firmware or model update window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWindow {
    pub entity_id: String,
    pub update_kind: String,
    pub scheduled_at: String,
}
