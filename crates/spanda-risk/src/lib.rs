//! Mission deployment risk scoring — composes coverage, readiness, trust, and contracts.
//!
mod mission_risk;

pub use mission_risk::{
    evaluate_mission_risk, format_mission_risk, MissionRiskAssessment, MissionRiskFactor,
    MissionRiskFormat, MissionRiskScore,
};
