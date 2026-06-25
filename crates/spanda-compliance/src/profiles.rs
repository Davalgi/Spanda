//! Built-in industry compliance profile templates.

use serde::{Deserialize, Serialize};

/// Template requirements for an industry compliance profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceProfile {
    pub name: String,
    pub description: String,
    pub requires_kill_switch: bool,
    pub min_readiness_score: u32,
    pub required_capabilities: Vec<String>,
    pub min_health_checks: usize,
    pub requires_assurance_case: bool,
    pub max_speed_mps: Option<f64>,
    pub operation_hours: Option<String>,
    pub requires_secure_comm: bool,
    pub warn_only: bool,
}

/// List built-in compliance profile names.
pub fn list_builtin_profiles() -> Vec<&'static str> {
    vec![
        "industrial",
        "warehouse",
        "medical",
        "agriculture",
        "defense",
        "research",
    ]
}

/// Resolve a built-in compliance profile by name.
pub fn builtin_profile(name: &str) -> Option<ComplianceProfile> {
    // Look up a profile template by case-insensitive name.
    //
    // Parameters:
    // - `name` — profile identifier
    //
    // Returns:
    // Profile template when recognized.
    //
    // Options:
    // None.
    //
    // Example:
    // let profile = builtin_profile("warehouse")?;

    match name.trim().to_ascii_lowercase().as_str() {
        "industrial" => Some(industrial_profile()),
        "warehouse" => Some(warehouse_profile()),
        "medical" => Some(medical_profile()),
        "agriculture" => Some(agriculture_profile()),
        "defense" => Some(defense_profile()),
        "research" => Some(research_profile()),
        _ => None,
    }
}

fn industrial_profile() -> ComplianceProfile {
    ComplianceProfile {
        name: "industrial".into(),
        description: "Factory AMRs with fixed safety zones and baseline readiness".into(),
        requires_kill_switch: true,
        min_readiness_score: 75,
        required_capabilities: vec!["obstacle_avoidance".into()],
        min_health_checks: 1,
        requires_assurance_case: false,
        max_speed_mps: Some(1.5),
        operation_hours: None,
        requires_secure_comm: false,
        warn_only: false,
    }
}

fn warehouse_profile() -> ComplianceProfile {
    ComplianceProfile {
        name: "warehouse".into(),
        description: "Warehouse AMRs with speed caps and shift-hour discipline".into(),
        requires_kill_switch: true,
        min_readiness_score: 70,
        required_capabilities: vec![
            "gps_navigation".into(),
            "obstacle_avoidance".into(),
        ],
        min_health_checks: 1,
        requires_assurance_case: false,
        max_speed_mps: Some(2.0),
        operation_hours: Some("06:00-22:00".into()),
        requires_secure_comm: false,
        warn_only: false,
    }
}

fn medical_profile() -> ComplianceProfile {
    ComplianceProfile {
        name: "medical".into(),
        description: "Medical robotics with stricter health evidence and assurance cases".into(),
        requires_kill_switch: true,
        min_readiness_score: 85,
        required_capabilities: vec![],
        min_health_checks: 2,
        requires_assurance_case: true,
        max_speed_mps: Some(1.0),
        operation_hours: None,
        requires_secure_comm: false,
        warn_only: false,
    }
}

fn agriculture_profile() -> ComplianceProfile {
    ComplianceProfile {
        name: "agriculture".into(),
        description: "Outdoor agriculture with GPS reliance and connectivity tolerance".into(),
        requires_kill_switch: true,
        min_readiness_score: 60,
        required_capabilities: vec!["gps_navigation".into()],
        min_health_checks: 1,
        requires_assurance_case: false,
        max_speed_mps: Some(2.5),
        operation_hours: None,
        requires_secure_comm: false,
        warn_only: false,
    }
}

fn defense_profile() -> ComplianceProfile {
    ComplianceProfile {
        name: "defense".into(),
        description: "Defense robotics with secure comm and capability minimization".into(),
        requires_kill_switch: true,
        min_readiness_score: 85,
        required_capabilities: vec![],
        min_health_checks: 1,
        requires_assurance_case: true,
        max_speed_mps: Some(1.2),
        operation_hours: None,
        requires_secure_comm: true,
        warn_only: false,
    }
}

fn research_profile() -> ComplianceProfile {
    ComplianceProfile {
        name: "research".into(),
        description: "Research deployments with relaxed gates and explicit warnings".into(),
        requires_kill_switch: false,
        min_readiness_score: 50,
        required_capabilities: vec![],
        min_health_checks: 0,
        requires_assurance_case: false,
        max_speed_mps: None,
        operation_hours: None,
        requires_secure_comm: false,
        warn_only: true,
    }
}
