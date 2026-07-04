//! Operational Governance Framework for Spanda autonomous systems.
//!
//! Provides abstractions for standards awareness, compliance validation,
//! deployment governance, certification tracking, risk assessment, and
//! human accountability — without embedding regulatory text.
//!
pub mod certification;
pub mod certification_store;
pub mod deployment_profile;
pub mod deployment_verify;
pub mod entity_governance;
pub mod human_accountability;
pub mod influence;
pub mod policy;
pub mod policy_store;
pub mod report;
pub mod types;
pub mod validate;

pub use certification::{
    CertificationEvidence, CertificationRecord, EntityCertificationSummary,
};
pub use certification_store::{
    default_certification_store_path, format_certification_report, CertificationReport,
    CertificationStore,
};
pub use deployment_profile::{
    deployment_profile_by_name, list_deployment_profiles, CommunicationConstraints,
    DecisionAuthorityRules, DeploymentProfile, EnvironmentalConstraints,
};
pub use deployment_verify::{
    format_deployment_verify, verify_deployment, DeploymentVerifyReport,
};
pub use entity_governance::{
    evaluate_entity_governance, governance_from_entity, parse_governance_config,
    stamp_entity_governance, EntityGovernance, EntityGovernanceOptions, EntityGovernanceReport,
    GovernanceFinding,
};
pub use human_accountability::{
    AccountabilityContact, ApprovalChainStep, HumanAccountability, OperatorCertification,
};
pub use influence::{
    action_requires_human_approval, blocks_live_deployment, influence_for_entity,
    GovernanceInfluence, GovernanceInfluenceFinding,
};
pub use policy::{list_standards_profiles, GovernancePolicyRef, StandardsProfileRef};
pub use policy_store::{
    default_policy_store_path, policy_ref, PolicyAssignment, PolicyAuditEntry, PolicyStore,
};
pub use report::{
    format_compliance_report, format_entity_governance_report, format_governance_report,
    format_governance_validation,
};
pub use types::{
    AutonomyLevel, CertificationStatus, DeploymentMaturity, DeploymentProfileKind,
    GovernancePolicyKind, OperationalConstraint, OperationalRisk, StandardsProfileKind,
    ValidationSeverity,
};
pub use validate::{
    governance_framework_summary, run_compliance_check, validate_governance, ComplianceCheckReport,
    ComplianceItem, ComplianceSummary, GovernanceValidationReport, ValidationOptions,
};

#[cfg(test)]
mod tests {
    use crate::deployment_profile::deployment_profile_by_name;
    use crate::types::{AutonomyLevel, DeploymentMaturity, OperationalRisk};

    #[test]
    fn autonomy_level_parsing() {
        assert_eq!(AutonomyLevel::parse("3"), AutonomyLevel::ConditionalAutonomy);
        assert!(AutonomyLevel::ConditionalAutonomy.requires_human_approval());
        assert!(!AutonomyLevel::HighAutonomy.requires_human_approval());
    }

    #[test]
    fn deployment_maturity_live_gate() {
        assert!(!DeploymentMaturity::Simulation.allows_live_deployment());
        assert!(DeploymentMaturity::Production.allows_live_deployment());
    }

    #[test]
    fn operational_risk_approval() {
        assert!(OperationalRisk::High.requires_human_approval());
        assert!(!OperationalRisk::Low.requires_human_approval());
    }

    #[test]
    fn builtin_warehouse_profile() {
        let profile = deployment_profile_by_name("warehouse").expect("warehouse profile");
        assert_eq!(profile.kind.as_str(), "warehouse");
        assert_eq!(profile.default_risk_level, OperationalRisk::Medium);
    }
}
