//! Industry compliance profile verification for Spanda programs.
//!
pub mod evaluate;
pub mod profiles;

pub use evaluate::{
    evaluate_compliance_profile, format_compliance_report, list_compliance_profiles,
    ComplianceEvaluationReport, ComplianceSeverity, ComplianceViolation,
};
pub use profiles::ComplianceProfile;
