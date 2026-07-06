//! Compliance and governance validation orchestration.
//!
use crate::deployment_profile::{deployment_profile_by_name, list_deployment_profiles};
use crate::entity_governance::{
    evaluate_entity_governance, governance_from_entity, EntityGovernanceOptions,
    EntityGovernanceReport,
};
use crate::policy::list_standards_profiles;
use crate::types::ValidationSeverity;
use serde::{Deserialize, Serialize};
use spanda_config::entity::{EntityRecord, EntityRegistry};
use spanda_config::ResolvedSystemConfig;

/// System-wide compliance check report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceCheckReport {
    pub passed: bool,
    pub summary: ComplianceSummary,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_reports: Vec<EntityGovernanceReport>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<ComplianceItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing_requirements: Vec<ComplianceItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommended_actions: Vec<String>,
}

/// Compliance summary counters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceSummary {
    pub entities_checked: usize,
    pub entities_passed: usize,
    pub entities_failed: usize,
    pub warnings: usize,
    pub missing: usize,
}

/// Individual compliance item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplianceItem {
    pub severity: ValidationSeverity,
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

/// Governance validation report for system configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceValidationReport {
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operational_maturity: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<ComplianceItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommended_actions: Vec<String>,
}

/// Options for compliance and governance validation.
#[derive(Debug, Clone, Default)]
pub struct ValidationOptions {
    pub strict: bool,
    pub entity_id: Option<String>,
}

/// Run system-wide compliance check against entity registry.
pub fn run_compliance_check(
    registry: &EntityRegistry,
    resolved: &ResolvedSystemConfig,
    opts: &ValidationOptions,
) -> ComplianceCheckReport {
    // Evaluate governance posture for all governed entities in the registry.
    let system_governance = resolved
        .section("governance")
        .map(crate::entity_governance::parse_governance_config)
        .unwrap_or_default();

    let eval_opts = EntityGovernanceOptions {
        strict: opts.strict,
    };

    let entities: Vec<&EntityRecord> = if let Some(id) = opts.entity_id.as_ref() {
        registry.get(id).into_iter().collect()
    } else {
        registry.entities.values().collect()
    };

    let mut entity_reports = Vec::new();
    let mut warnings = Vec::new();
    let mut missing = Vec::new();
    let mut actions = Vec::new();
    let mut passed_count = 0usize;

    for entity in entities {
        // Only evaluate entities that carry governance metadata (stamped fleet/robots).
        if entity.governance.is_none() {
            continue;
        }
        let mut governance = governance_from_entity(entity);
        if governance.autonomy_level.is_none() {
            governance.autonomy_level = system_governance.autonomy_level;
        }
        if governance.deployment_profile.is_none() {
            governance.deployment_profile = system_governance.deployment_profile.clone();
        }
        if governance.operational_maturity.is_none() {
            governance.operational_maturity = system_governance.operational_maturity;
        }
        if governance.risk_level.is_none() {
            governance.risk_level = system_governance.risk_level;
        }
        if governance.certification.is_none() {
            governance.certification = system_governance.certification.clone();
        }
        if governance.accountability.is_none() {
            governance.accountability = system_governance.accountability.clone();
        } else if let (Some(entity_acct), Some(system_acct)) = (
            governance.accountability.as_mut(),
            system_governance.accountability.as_ref(),
        ) {
            if entity_acct.responsible_person.is_none() {
                entity_acct.responsible_person = system_acct.responsible_person.clone();
            }
            if entity_acct.deployment_owner.is_none() {
                entity_acct.deployment_owner = system_acct.deployment_owner.clone();
            }
            if entity_acct.emergency_contact.is_none() {
                entity_acct.emergency_contact = system_acct.emergency_contact.clone();
            }
            if entity_acct.approval_chain.is_empty() {
                entity_acct.approval_chain = system_acct.approval_chain.clone();
            }
        }
        if governance.standards_profiles.is_empty() {
            governance.standards_profiles = system_governance.standards_profiles.clone();
        }

        let report = evaluate_entity_governance(&entity.id, registry, &governance, &eval_opts);
        if report.passed {
            passed_count += 1;
        }
        for finding in &report.findings {
            let item = ComplianceItem {
                severity: finding.severity,
                code: finding.code.clone(),
                message: finding.message.clone(),
                entity_id: Some(entity.id.clone()),
            };
            match finding.severity {
                ValidationSeverity::Warning => warnings.push(item),
                ValidationSeverity::Missing | ValidationSeverity::Action => missing.push(item),
                ValidationSeverity::Pass => {}
            }
        }
        actions.extend(report.recommended_actions.clone());
        entity_reports.push(report);
    }

    actions.sort();
    actions.dedup();

    let entities_checked = entity_reports.len();
    let entities_failed = entities_checked.saturating_sub(passed_count);
    let passed = if entities_failed == 0 && warnings.is_empty() {
        true
    } else {
        !opts.strict
    };

    ComplianceCheckReport {
        passed,
        summary: ComplianceSummary {
            entities_checked,
            entities_passed: passed_count,
            entities_failed,
            warnings: warnings.len(),
            missing: missing.len(),
        },
        entity_reports,
        warnings,
        missing_requirements: missing,
        recommended_actions: actions,
    }
}

/// Validate governance configuration for deployment readiness.
pub fn validate_governance(
    registry: &EntityRegistry,
    resolved: &ResolvedSystemConfig,
    opts: &ValidationOptions,
) -> GovernanceValidationReport {
    let mut findings = Vec::new();
    let mut actions = Vec::new();

    let system_gov = resolved
        .section("governance")
        .map(crate::entity_governance::parse_governance_config)
        .unwrap_or_default();

    let profile_name = system_gov
        .deployment_profile
        .as_ref()
        .map(|k| k.as_str().to_string())
        .or_else(|| {
            resolved
                .section("governance")
                .and_then(|v| v.get("deployment_profile"))
                .and_then(|v| v.as_str())
                .map(String::from)
        });

    let maturity = system_gov
        .operational_maturity
        .map(|m| m.as_str().to_string());

    if system_gov.deployment_profile.is_none() {
        findings.push(ComplianceItem {
            severity: ValidationSeverity::Warning,
            code: "GOV_NO_PROFILE".into(),
            message: "No deployment profile configured — using entity-level defaults".into(),
            entity_id: None,
        });
    }

    if let Some(ref name) = profile_name {
        if deployment_profile_by_name(name).is_none() {
            findings.push(ComplianceItem {
                severity: ValidationSeverity::Missing,
                code: "GOV_UNKNOWN_PROFILE".into(),
                message: format!("Unknown deployment profile '{name}'"),
                entity_id: None,
            });
            actions.push(format!(
                "Select a valid deployment profile from: {}",
                list_deployment_profiles()
                    .iter()
                    .map(|p| p.kind.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    if system_gov.operational_maturity.is_none() {
        findings.push(ComplianceItem {
            severity: ValidationSeverity::Warning,
            code: "GOV_NO_MATURITY".into(),
            message: "Operational maturity not set — defaulting to concept stage".into(),
            entity_id: None,
        });
    }

    if !system_gov
        .accountability
        .as_ref()
        .map(|a| a.is_complete_for_production())
        .unwrap_or(false)
        && system_gov
            .operational_maturity
            .map(|m| m.allows_live_deployment())
            .unwrap_or(false)
    {
        findings.push(ComplianceItem {
            severity: ValidationSeverity::Missing,
            code: "GOV_ACCOUNTABILITY_SYSTEM".into(),
            message: "System-level human accountability incomplete for live deployment".into(),
            entity_id: None,
        });
        actions.push("Configure responsible_person, deployment_owner, and emergency_contact in spanda.governance.toml".into());
    }

    let compliance = run_compliance_check(registry, resolved, opts);
    findings.extend(compliance.warnings.clone());
    findings.extend(compliance.missing_requirements.clone());
    actions.extend(compliance.recommended_actions.clone());

    let has_blocking = findings.iter().any(|f| {
        matches!(
            f.severity,
            ValidationSeverity::Missing | ValidationSeverity::Action
        )
    });
    let passed = !has_blocking && compliance.passed;

    GovernanceValidationReport {
        passed,
        deployment_profile: profile_name,
        operational_maturity: maturity,
        findings,
        recommended_actions: actions,
    }
}

/// Build governance framework summary for API listing.
pub fn governance_framework_summary() -> serde_json::Value {
    serde_json::json!({
        "version": "v1",
        "framework": "operational_governance",
        "capabilities": [
            "standards_awareness",
            "compliance_validation",
            "deployment_governance",
            "operational_policies",
            "certification_tracking",
            "risk_assessment",
            "audit_support",
            "human_accountability"
        ],
        "deployment_profiles": list_deployment_profiles().iter().map(|p| serde_json::json!({
            "kind": p.kind.as_str(),
            "display_name": p.display_name,
            "default_risk_level": p.default_risk_level.as_str(),
            "max_autonomy_level": p.decision_authority.max_autonomy_level.as_str(),
        })).collect::<Vec<_>>(),
        "standards_profiles": list_standards_profiles().iter().map(|p| serde_json::json!({
            "kind": p.kind.as_str(),
            "package": p.package,
            "required_checks": p.required_checks.len(),
        })).collect::<Vec<_>>(),
        "autonomy_levels": (0..=5u8).map(|n| AutonomyLevel::parse(&n.to_string()).as_str()).collect::<Vec<_>>(),
        "disclaimer": "Spanda provides governance abstractions and validation mechanisms — not legal or regulatory advice."
    })
}

use crate::types::AutonomyLevel;
