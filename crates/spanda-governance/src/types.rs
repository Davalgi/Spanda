//! Core operational governance enumerations and shared types.
//!
use serde::{Deserialize, Serialize};

/// SAE-style autonomy level for entity decision authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AutonomyLevel {
    /// Level 0 — full human control, no automation.
    #[default]
    Manual,
    /// Level 1 — driver/operator assistance.
    Assisted,
    /// Level 2 — partial automation with human monitoring.
    PartialAutomation,
    /// Level 3 — conditional autonomy in defined ODD.
    ConditionalAutonomy,
    /// Level 4 — high autonomy within operational design domain.
    HighAutonomy,
    /// Level 5 — full autonomy without human intervention.
    FullAutonomy,
}

impl AutonomyLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Assisted => "assisted",
            Self::PartialAutomation => "partial_automation",
            Self::ConditionalAutonomy => "conditional_autonomy",
            Self::HighAutonomy => "high_autonomy",
            Self::FullAutonomy => "full_autonomy",
        }
    }

    pub fn level_number(&self) -> u8 {
        match self {
            Self::Manual => 0,
            Self::Assisted => 1,
            Self::PartialAutomation => 2,
            Self::ConditionalAutonomy => 3,
            Self::HighAutonomy => 4,
            Self::FullAutonomy => 5,
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().replace(['-', ' '], "_").as_str() {
            "manual" | "level_0" | "l0" | "0" => Self::Manual,
            "assisted" | "level_1" | "l1" | "1" => Self::Assisted,
            "partial_automation" | "partial" | "level_2" | "l2" | "2" => Self::PartialAutomation,
            "conditional_autonomy" | "conditional" | "level_3" | "l3" | "3" => {
                Self::ConditionalAutonomy
            }
            "high_autonomy" | "high" | "level_4" | "l4" | "4" => Self::HighAutonomy,
            "full_autonomy" | "full" | "level_5" | "l5" | "5" => Self::FullAutonomy,
            _ => Self::Manual,
        }
    }

    /// Whether human approval is required before autonomous action at this level.
    pub fn requires_human_approval(&self) -> bool {
        self.level_number() < 4
    }

    /// Minimum trust posture required for this autonomy level.
    pub fn minimum_trust_tier(&self) -> &'static str {
        match self {
            Self::Manual | Self::Assisted => "unverified",
            Self::PartialAutomation => "verified",
            Self::ConditionalAutonomy => "trusted",
            Self::HighAutonomy | Self::FullAutonomy => "trusted",
        }
    }
}

/// Deployment maturity stage for operational lifecycle governance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentMaturity {
    #[default]
    Concept,
    Prototype,
    Simulation,
    Laboratory,
    Pilot,
    PreProduction,
    Production,
    MissionCritical,
    Certified,
    Retired,
}

impl DeploymentMaturity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Concept => "concept",
            Self::Prototype => "prototype",
            Self::Simulation => "simulation",
            Self::Laboratory => "laboratory",
            Self::Pilot => "pilot",
            Self::PreProduction => "pre_production",
            Self::Production => "production",
            Self::MissionCritical => "mission_critical",
            Self::Certified => "certified",
            Self::Retired => "retired",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().replace(['-', ' '], "_").as_str() {
            "concept" => Self::Concept,
            "prototype" => Self::Prototype,
            "simulation" | "sim" => Self::Simulation,
            "laboratory" | "lab" => Self::Laboratory,
            "pilot" => Self::Pilot,
            "pre_production" | "preproduction" | "staging" => Self::PreProduction,
            "production" | "prod" => Self::Production,
            "mission_critical" | "missioncritical" => Self::MissionCritical,
            "certified" => Self::Certified,
            "retired" | "decommissioned" => Self::Retired,
            _ => Self::Concept,
        }
    }

    /// Whether deployment to live environments is permitted at this maturity.
    pub fn allows_live_deployment(&self) -> bool {
        matches!(
            self,
            Self::Pilot
                | Self::PreProduction
                | Self::Production
                | Self::MissionCritical
                | Self::Certified
        )
    }
}

/// Certification lifecycle state — independent from health posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CertificationStatus {
    #[default]
    Draft,
    UnderReview,
    Testing,
    Validated,
    Certified,
    Expired,
    Revoked,
    Suspended,
    Deprecated,
    Archived,
}

impl CertificationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::UnderReview => "under_review",
            Self::Testing => "testing",
            Self::Validated => "validated",
            Self::Certified => "certified",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Suspended => "suspended",
            Self::Deprecated => "deprecated",
            Self::Archived => "archived",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().replace(['-', ' '], "_").as_str() {
            "draft" => Self::Draft,
            "under_review" | "review" => Self::UnderReview,
            "testing" | "test" => Self::Testing,
            "validated" => Self::Validated,
            "certified" => Self::Certified,
            "expired" => Self::Expired,
            "revoked" => Self::Revoked,
            "suspended" => Self::Suspended,
            "deprecated" => Self::Deprecated,
            "archived" => Self::Archived,
            _ => Self::Draft,
        }
    }

    pub fn is_operational(&self) -> bool {
        matches!(self, Self::Validated | Self::Certified)
    }
}

/// Operational risk tier influencing decision authority and recovery posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OperationalRisk {
    #[default]
    Negligible,
    Low,
    Medium,
    High,
    Critical,
    LifeCritical,
    MissionCritical,
}

impl OperationalRisk {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Negligible => "negligible",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
            Self::LifeCritical => "life_critical",
            Self::MissionCritical => "mission_critical",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().replace(['-', ' '], "_").as_str() {
            "negligible" | "minimal" => Self::Negligible,
            "low" => Self::Low,
            "medium" | "moderate" => Self::Medium,
            "high" => Self::High,
            "critical" => Self::Critical,
            "life_critical" | "lifecritical" | "safety_critical" => Self::LifeCritical,
            "mission_critical" | "missioncritical" => Self::MissionCritical,
            _ => Self::Negligible,
        }
    }

    pub fn requires_human_approval(&self) -> bool {
        matches!(
            self,
            Self::High | Self::Critical | Self::LifeCritical | Self::MissionCritical
        )
    }

    pub fn requires_simulation(&self) -> bool {
        matches!(
            self,
            Self::Medium | Self::High | Self::Critical | Self::LifeCritical | Self::MissionCritical
        )
    }
}

/// Environmental and operational constraint tags for entity deployment context.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalConstraint {
    Connectivity,
    Power,
    Latency,
    Temperature,
    Humidity,
    HazardousArea,
    CleanRoom,
    GpsAvailability,
    Indoor,
    Outdoor,
    Underground,
    Remote,
    LimitedBandwidth,
    BatteryOnly,
    Offline,
    HighEmi,
    Radiation,
    Waterproof,
    ExplosionRisk,
    Custom(String),
}

impl OperationalConstraint {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Connectivity => "connectivity",
            Self::Power => "power",
            Self::Latency => "latency",
            Self::Temperature => "temperature",
            Self::Humidity => "humidity",
            Self::HazardousArea => "hazardous_area",
            Self::CleanRoom => "clean_room",
            Self::GpsAvailability => "gps_availability",
            Self::Indoor => "indoor",
            Self::Outdoor => "outdoor",
            Self::Underground => "underground",
            Self::Remote => "remote",
            Self::LimitedBandwidth => "limited_bandwidth",
            Self::BatteryOnly => "battery_only",
            Self::Offline => "offline",
            Self::HighEmi => "high_emi",
            Self::Radiation => "radiation",
            Self::Waterproof => "waterproof",
            Self::ExplosionRisk => "explosion_risk",
            Self::Custom(name) => name.as_str(),
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().replace(['-', ' '], "_").as_str() {
            "connectivity" => Self::Connectivity,
            "power" => Self::Power,
            "latency" => Self::Latency,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "hazardous_area" | "hazardous" => Self::HazardousArea,
            "clean_room" | "cleanroom" => Self::CleanRoom,
            "gps_availability" | "gps" => Self::GpsAvailability,
            "indoor" => Self::Indoor,
            "outdoor" => Self::Outdoor,
            "underground" => Self::Underground,
            "remote" => Self::Remote,
            "limited_bandwidth" | "bandwidth" => Self::LimitedBandwidth,
            "battery_only" | "battery" => Self::BatteryOnly,
            "offline" => Self::Offline,
            "high_emi" | "emi" => Self::HighEmi,
            "radiation" => Self::Radiation,
            "waterproof" => Self::Waterproof,
            "explosion_risk" | "atex" => Self::ExplosionRisk,
            other => Self::Custom(other.to_string()),
        }
    }
}

/// Governance policy category for versioned operational policy assignment.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernancePolicyKind {
    Safety,
    Security,
    Recovery,
    Trust,
    Maintenance,
    Update,
    Decision,
    Mission,
    Deployment,
    Compliance,
    Custom(String),
}

impl GovernancePolicyKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Safety => "safety",
            Self::Security => "security",
            Self::Recovery => "recovery",
            Self::Trust => "trust",
            Self::Maintenance => "maintenance",
            Self::Update => "update",
            Self::Decision => "decision",
            Self::Mission => "mission",
            Self::Deployment => "deployment",
            Self::Compliance => "compliance",
            Self::Custom(name) => name.as_str(),
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "safety" => Self::Safety,
            "security" => Self::Security,
            "recovery" => Self::Recovery,
            "trust" => Self::Trust,
            "maintenance" => Self::Maintenance,
            "update" => Self::Update,
            "decision" => Self::Decision,
            "mission" => Self::Mission,
            "deployment" => Self::Deployment,
            "compliance" => Self::Compliance,
            other => Self::Custom(other.to_string()),
        }
    }
}

/// Optional standards profile reference — implemented via packages/plugins, not embedded text.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StandardsProfileKind {
    FunctionalSafety,
    IndustrialSafety,
    Cybersecurity,
    MedicalDevice,
    Automotive,
    Aviation,
    Rail,
    Maritime,
    Energy,
    Space,
    Government,
    Custom(String),
}

impl StandardsProfileKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::FunctionalSafety => "functional_safety",
            Self::IndustrialSafety => "industrial_safety",
            Self::Cybersecurity => "cybersecurity",
            Self::MedicalDevice => "medical_device",
            Self::Automotive => "automotive",
            Self::Aviation => "aviation",
            Self::Rail => "rail",
            Self::Maritime => "maritime",
            Self::Energy => "energy",
            Self::Space => "space",
            Self::Government => "government",
            Self::Custom(name) => name.as_str(),
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().replace(['-', ' '], "_").as_str() {
            "functional_safety" | "func_safety" => Self::FunctionalSafety,
            "industrial_safety" | "industrial" => Self::IndustrialSafety,
            "cybersecurity" | "cyber" => Self::Cybersecurity,
            "medical_device" | "medical" => Self::MedicalDevice,
            "automotive" | "auto" => Self::Automotive,
            "aviation" | "aerospace" => Self::Aviation,
            "rail" => Self::Rail,
            "maritime" => Self::Maritime,
            "energy" => Self::Energy,
            "space" => Self::Space,
            "government" | "gov" => Self::Government,
            other => Self::Custom(other.to_string()),
        }
    }
}

/// Deployment profile identifier for industry/operational context.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileKind {
    Warehouse,
    Factory,
    Hospital,
    OperatingRoom,
    SearchRescue,
    Agriculture,
    Mining,
    Construction,
    Maritime,
    Aviation,
    Space,
    RoadVehicle,
    Campus,
    SmartBuilding,
    Home,
    Retail,
    Office,
    Defense,
    Research,
    Custom(String),
}

impl DeploymentProfileKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Warehouse => "warehouse",
            Self::Factory => "factory",
            Self::Hospital => "hospital",
            Self::OperatingRoom => "operating_room",
            Self::SearchRescue => "search_rescue",
            Self::Agriculture => "agriculture",
            Self::Mining => "mining",
            Self::Construction => "construction",
            Self::Maritime => "maritime",
            Self::Aviation => "aviation",
            Self::Space => "space",
            Self::RoadVehicle => "road_vehicle",
            Self::Campus => "campus",
            Self::SmartBuilding => "smart_building",
            Self::Home => "home",
            Self::Retail => "retail",
            Self::Office => "office",
            Self::Defense => "defense",
            Self::Research => "research",
            Self::Custom(name) => name.as_str(),
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().replace(['-', ' '], "_").as_str() {
            "warehouse" => Self::Warehouse,
            "factory" | "industrial" => Self::Factory,
            "hospital" | "healthcare" => Self::Hospital,
            "operating_room" | "or" => Self::OperatingRoom,
            "search_rescue" | "sar" => Self::SearchRescue,
            "agriculture" | "agri" => Self::Agriculture,
            "mining" => Self::Mining,
            "construction" => Self::Construction,
            "maritime" => Self::Maritime,
            "aviation" => Self::Aviation,
            "space" => Self::Space,
            "road_vehicle" | "adas" | "automotive" => Self::RoadVehicle,
            "campus" => Self::Campus,
            "smart_building" | "building" => Self::SmartBuilding,
            "home" => Self::Home,
            "retail" => Self::Retail,
            "office" => Self::Office,
            "defense" | "military" => Self::Defense,
            "research" | "lab" => Self::Research,
            other => Self::Custom(other.to_string()),
        }
    }

    pub fn all_builtin() -> Vec<Self> {
        vec![
            Self::Warehouse,
            Self::Factory,
            Self::Hospital,
            Self::OperatingRoom,
            Self::SearchRescue,
            Self::Agriculture,
            Self::Mining,
            Self::Construction,
            Self::Maritime,
            Self::Aviation,
            Self::Space,
            Self::RoadVehicle,
            Self::Campus,
            Self::SmartBuilding,
            Self::Home,
            Self::Retail,
            Self::Office,
            Self::Defense,
            Self::Research,
        ]
    }
}

/// Validation outcome severity for compliance and governance checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Pass,
    Warning,
    Missing,
    Action,
}
