//! Versioned governance policy references — signed, auditable, assignable.
//!
use crate::types::GovernancePolicyKind;
use serde::{Deserialize, Serialize};

/// Reference to a versioned governance policy package.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernancePolicyRef {
    pub kind: GovernancePolicyKind,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audit_id: Option<String>,
}

impl GovernancePolicyRef {
    pub fn new(kind: GovernancePolicyKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            version: None,
            package: None,
            signature: None,
            signed_by: None,
            signed_at: None,
            audit_id: None,
        }
    }

    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }
}

/// Standards profile reference — requirements delivered via packages/plugins.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandardsProfileRef {
    pub kind: crate::types::StandardsProfileKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_checks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_evidence: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_reports: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_approvals: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_documentation: Vec<String>,
}

impl StandardsProfileRef {
    pub fn builtin(kind: crate::types::StandardsProfileKind) -> Self {
        let (checks, evidence, reports, approvals, docs) = standards_defaults(&kind);
        let slug = kind.as_str().to_string();
        Self {
            kind,
            package: Some(format!("spanda-standards-{slug}")),
            required_checks: checks,
            required_evidence: evidence,
            required_reports: reports,
            required_approvals: approvals,
            required_documentation: docs,
        }
    }
}

fn standards_defaults(
    kind: &crate::types::StandardsProfileKind,
) -> (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
) {
    match kind {
        crate::types::StandardsProfileKind::FunctionalSafety => (
            vec!["hazard_analysis".into(), "safety_case".into()],
            vec!["fmea_report".into(), "test_results".into()],
            vec!["safety_assessment".into()],
            vec!["safety_officer".into()],
            vec!["safety_manual".into()],
        ),
        crate::types::StandardsProfileKind::MedicalDevice => (
            vec!["risk_management".into(), "clinical_evaluation".into()],
            vec!["verification_report".into(), "validation_report".into()],
            vec!["technical_file".into()],
            vec!["regulatory_affairs".into(), "clinical_lead".into()],
            vec!["instructions_for_use".into()],
        ),
        crate::types::StandardsProfileKind::Automotive => (
            vec!["hara".into(), "fmea".into(), "sotif_analysis".into()],
            vec!["test_coverage".into(), "field_data".into()],
            vec!["safety_case".into()],
            vec!["functional_safety_manager".into()],
            vec!["safety_concept".into()],
        ),
        crate::types::StandardsProfileKind::Cybersecurity => (
            vec!["threat_model".into(), "vulnerability_scan".into()],
            vec!["pen_test_report".into(), "sbom".into()],
            vec!["security_assessment".into()],
            vec!["security_officer".into()],
            vec!["security_policy".into()],
        ),
        _ => (vec![], vec![], vec![], vec![], vec![]),
    }
}

/// List built-in standards profile references.
pub fn list_standards_profiles() -> Vec<StandardsProfileRef> {
    use crate::types::StandardsProfileKind;
    [
        StandardsProfileKind::FunctionalSafety,
        StandardsProfileKind::IndustrialSafety,
        StandardsProfileKind::Cybersecurity,
        StandardsProfileKind::MedicalDevice,
        StandardsProfileKind::Automotive,
        StandardsProfileKind::Aviation,
        StandardsProfileKind::Rail,
        StandardsProfileKind::Maritime,
        StandardsProfileKind::Energy,
        StandardsProfileKind::Space,
        StandardsProfileKind::Government,
    ]
    .into_iter()
    .map(StandardsProfileRef::builtin)
    .collect()
}
