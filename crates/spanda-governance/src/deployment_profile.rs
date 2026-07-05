//! Deployment profile model — operational context constraints and requirements.
//!
use crate::types::{
    AutonomyLevel, DeploymentProfileKind, OperationalConstraint, OperationalRisk,
    StandardsProfileKind,
};
use serde::{Deserialize, Serialize};

/// Communication constraints for a deployment profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationConstraints {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_protocols: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_latency_ms: Option<u64>,
    #[serde(default)]
    pub offline_capable: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bandwidth_tiers: Vec<String>,
}

/// Environmental constraints for a deployment profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentalConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_temperature_c: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_temperature_c: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_humidity_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hazard_classes: Vec<String>,
}

/// Decision authority rules for a deployment profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DecisionAuthorityRules {
    #[serde(default)]
    pub max_autonomy_level: AutonomyLevel,
    #[serde(default)]
    pub requires_human_approval_above_risk: OperationalRisk,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub escalation_contacts: Vec<String>,
}

/// Full deployment profile definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentProfile {
    pub kind: DeploymentProfileKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety_policies: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recovery_policies: Vec<String>,
    #[serde(default)]
    pub default_risk_level: OperationalRisk,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_hardware: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_certifications: Vec<String>,
    #[serde(default)]
    pub decision_authority: DecisionAuthorityRules,
    #[serde(default)]
    pub communication: CommunicationConstraints,
    #[serde(default)]
    pub environmental: EnvironmentalConstraints,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operational_constraints: Vec<OperationalConstraint>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub standards_profiles: Vec<StandardsProfileKind>,
}

impl DeploymentProfile {
    pub fn builtin(kind: DeploymentProfileKind) -> Self {
        // Return built-in profile defaults for the requested kind.
        //
        // Parameters:
        // - `kind` — deployment profile identifier
        //
        // Returns:
        // A profile with industry-appropriate defaults.
        //
        // Options:
        // None.
        //
        // Example:
        // let profile = DeploymentProfile::builtin(DeploymentProfileKind::Warehouse);

        let (display, risk, autonomy, caps, hardware, certs, constraints, standards) =
            profile_defaults(&kind);
        Self {
            kind: kind.clone(),
            display_name: Some(display.clone()),
            description: Some(format!("Built-in {display} deployment profile")),
            safety_policies: vec!["safety.default".into()],
            recovery_policies: vec!["recovery.default".into()],
            default_risk_level: risk,
            required_capabilities: caps,
            required_hardware: hardware,
            required_certifications: certs,
            decision_authority: DecisionAuthorityRules {
                max_autonomy_level: autonomy,
                requires_human_approval_above_risk: OperationalRisk::High,
                escalation_contacts: vec![],
            },
            communication: CommunicationConstraints::default(),
            environmental: EnvironmentalConstraints::default(),
            operational_constraints: constraints,
            standards_profiles: standards,
        }
    }
}

fn profile_defaults(
    kind: &DeploymentProfileKind,
) -> (
    String,
    OperationalRisk,
    AutonomyLevel,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<OperationalConstraint>,
    Vec<StandardsProfileKind>,
) {
    match kind {
        DeploymentProfileKind::Warehouse => (
            "Warehouse".to_string(),
            OperationalRisk::Medium,
            AutonomyLevel::PartialAutomation,
            vec!["navigation".into(), "obstacle_avoidance".into()],
            vec!["lidar".into(), "imu".into()],
            vec!["iso3691-4".into()],
            vec![
                OperationalConstraint::Indoor,
                OperationalConstraint::Connectivity,
            ],
            vec![StandardsProfileKind::IndustrialSafety],
        ),
        DeploymentProfileKind::Hospital => (
            "Hospital".to_string(),
            OperationalRisk::LifeCritical,
            AutonomyLevel::Assisted,
            vec!["navigation".into(), "human_proximity".into()],
            vec!["lidar".into(), "camera".into()],
            vec!["iec62304".into(), "iso13485".into()],
            vec![
                OperationalConstraint::Indoor,
                OperationalConstraint::CleanRoom,
            ],
            vec![
                StandardsProfileKind::MedicalDevice,
                StandardsProfileKind::FunctionalSafety,
            ],
        ),
        DeploymentProfileKind::SearchRescue => (
            "Search & Rescue".to_string(),
            OperationalRisk::MissionCritical,
            AutonomyLevel::ConditionalAutonomy,
            vec![
                "navigation".into(),
                "gps_fallback".into(),
                "thermal_imaging".into(),
            ],
            vec!["gps".into(), "lidar".into(), "thermal_camera".into()],
            vec!["iso26262".into()],
            vec![
                OperationalConstraint::Outdoor,
                OperationalConstraint::Remote,
                OperationalConstraint::LimitedBandwidth,
            ],
            vec![StandardsProfileKind::Aviation],
        ),
        DeploymentProfileKind::Factory => (
            "Factory".to_string(),
            OperationalRisk::High,
            AutonomyLevel::PartialAutomation,
            vec!["manipulation".into(), "safety_zones".into()],
            vec!["force_torque".into(), "lidar".into()],
            vec!["iso10218".into(), "iso13849".into()],
            vec![
                OperationalConstraint::Indoor,
                OperationalConstraint::HazardousArea,
            ],
            vec![
                StandardsProfileKind::IndustrialSafety,
                StandardsProfileKind::FunctionalSafety,
            ],
        ),
        DeploymentProfileKind::RoadVehicle => (
            "Road Vehicle".to_string(),
            OperationalRisk::LifeCritical,
            AutonomyLevel::ConditionalAutonomy,
            vec!["perception".into(), "lane_keeping".into(), "aeb".into()],
            vec!["camera".into(), "radar".into(), "lidar".into()],
            vec!["iso26262".into(), "iso21448".into()],
            vec![
                OperationalConstraint::Outdoor,
                OperationalConstraint::GpsAvailability,
            ],
            vec![
                StandardsProfileKind::Automotive,
                StandardsProfileKind::FunctionalSafety,
            ],
        ),
        DeploymentProfileKind::SmartBuilding => (
            "Smart Building".to_string(),
            OperationalRisk::Low,
            AutonomyLevel::PartialAutomation,
            vec!["hvac_control".into(), "access_control".into()],
            vec!["iot_gateway".into()],
            vec![],
            vec![
                OperationalConstraint::Indoor,
                OperationalConstraint::Connectivity,
            ],
            vec![StandardsProfileKind::Cybersecurity],
        ),
        DeploymentProfileKind::OperatingRoom => (
            "Operating Room".to_string(),
            OperationalRisk::LifeCritical,
            AutonomyLevel::Assisted,
            vec!["precision_control".into(), "sterile_operation".into()],
            vec!["force_torque".into(), "camera".into()],
            vec!["iec62304".into()],
            vec![
                OperationalConstraint::CleanRoom,
                OperationalConstraint::HighEmi,
            ],
            vec![StandardsProfileKind::MedicalDevice],
        ),
        _ => (
            kind.as_str().to_string(),
            OperationalRisk::Medium,
            AutonomyLevel::PartialAutomation,
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ),
    }
}

/// List all built-in deployment profiles.
pub fn list_deployment_profiles() -> Vec<DeploymentProfile> {
    DeploymentProfileKind::all_builtin()
        .into_iter()
        .map(DeploymentProfile::builtin)
        .collect()
}

/// Resolve a deployment profile by name.
pub fn deployment_profile_by_name(name: &str) -> Option<DeploymentProfile> {
    let kind = DeploymentProfileKind::parse(name);
    if matches!(kind, DeploymentProfileKind::Custom(ref n) if n == name && !DeploymentProfileKind::all_builtin().contains(&kind))
    {
        // Custom profiles not in builtin list still resolve if explicitly named.
        return Some(DeploymentProfile::builtin(kind));
    }
    Some(DeploymentProfile::builtin(kind))
}
