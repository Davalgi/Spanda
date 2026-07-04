//! Deployment verification — profile requirements, maturity, certification, accountability.
//!
use crate::deployment_profile::deployment_profile_by_name;
use crate::entity_governance::{governance_from_entity, parse_governance_config};
use crate::influence::influence_for_entity;
use crate::types::ValidationSeverity;
use crate::validate::{ComplianceItem, ValidationOptions};
use serde::{Deserialize, Serialize};
use spanda_config::entity::EntityRegistry;
use spanda_config::ResolvedSystemConfig;

/// Deployment verification report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentVerifyReport {
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

/// Verify deployment readiness against governance profile and entity posture.
pub fn verify_deployment(
    registry: &EntityRegistry,
    resolved: &ResolvedSystemConfig,
    opts: &ValidationOptions,
) -> DeploymentVerifyReport {
    // Check profile requirements, live-deployment gates, and entity capabilities.
    //
    // Parameters:
    // - `registry` — entity graph
    // - `resolved` — system configuration including governance fragment
    // - `opts` — optional entity filter and strict mode
    //
    // Returns:
    // Deployment verification report.
    //
    // Options:
    // `strict` treats medium findings as failures.
    //
    // Example:
    // let report = verify_deployment(&registry, &resolved, &opts);

    let mut findings = Vec::new();
    let mut actions = Vec::new();
    let system_gov = resolved
        .section("governance")
        .map(parse_governance_config)
        .unwrap_or_default();

    let profile_name = system_gov
        .deployment_profile
        .as_ref()
        .map(|k| k.as_str().to_string());
    let maturity = system_gov
        .operational_maturity
        .map(|m| m.as_str().to_string());

    if let Some(ref name) = profile_name {
        if let Some(profile) = deployment_profile_by_name(name) {
            let entities: Vec<_> = if let Some(id) = opts.entity_id.as_ref() {
                registry.get(id).into_iter().collect()
            } else {
                registry
                    .entities
                    .values()
                    .filter(|e| e.governance.is_some())
                    .collect()
            };
            for entity in entities {
                let influence = influence_for_entity(entity);
                for blocker in &influence.readiness_blockers {
                    findings.push(ComplianceItem {
                        severity: if blocker.severity == "high" {
                            ValidationSeverity::Missing
                        } else {
                            ValidationSeverity::Warning
                        },
                        code: "DEPLOY_GOV".into(),
                        message: blocker.message.clone(),
                        entity_id: Some(entity.id.clone()),
                    });
                }
                let capability_applicable = matches!(
                    entity.entity_type,
                    spanda_config::entity::EntityKind::Robot
                        | spanda_config::entity::EntityKind::Drone
                        | spanda_config::entity::EntityKind::Vehicle
                );
                if capability_applicable {
                    for cap in &profile.required_capabilities {
                        if !entity.capabilities.iter().any(|c| c == cap) {
                            findings.push(ComplianceItem {
                                severity: ValidationSeverity::Missing,
                                code: "DEPLOY_CAPABILITY".into(),
                                message: format!(
                                    "Profile '{}' requires capability '{}'",
                                    name, cap
                                ),
                                entity_id: Some(entity.id.clone()),
                            });
                            actions.push(format!("Add capability '{cap}' to {}", entity.id));
                        }
                    }
                }
                let gov = governance_from_entity(entity);
                if let Some(autonomy) = gov.autonomy_level {
                    if autonomy > profile.decision_authority.max_autonomy_level {
                        findings.push(ComplianceItem {
                            severity: ValidationSeverity::Warning,
                            code: "DEPLOY_AUTONOMY".into(),
                            message: format!(
                                "Autonomy {} exceeds profile max {}",
                                autonomy.as_str(),
                                profile.decision_authority.max_autonomy_level.as_str()
                            ),
                            entity_id: Some(entity.id.clone()),
                        });
                    }
                }
            }
        } else {
            findings.push(ComplianceItem {
                severity: ValidationSeverity::Missing,
                code: "DEPLOY_UNKNOWN_PROFILE".into(),
                message: format!("Unknown deployment profile '{name}'"),
                entity_id: None,
            });
        }
    } else {
        findings.push(ComplianceItem {
            severity: ValidationSeverity::Warning,
            code: "DEPLOY_NO_PROFILE".into(),
            message: "No deployment profile configured".into(),
            entity_id: None,
        });
        actions.push("Set deployment_profile in spanda.governance.toml".into());
    }

    let has_blocking = findings.iter().any(|f| {
        matches!(
            f.severity,
            ValidationSeverity::Missing | ValidationSeverity::Action
        )
    });
    let has_warnings = findings
        .iter()
        .any(|f| f.severity == ValidationSeverity::Warning);
    let passed = !has_blocking && (!opts.strict || !has_warnings);

    DeploymentVerifyReport {
        passed,
        deployment_profile: profile_name,
        operational_maturity: maturity,
        findings,
        recommended_actions: actions,
    }
}

pub fn format_deployment_verify(report: &DeploymentVerifyReport, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".into());
    }
    let mut lines = vec![
        "Deployment Verification Report".into(),
        "==============================".into(),
        format!(
            "Result: {}",
            if report.passed { "PASS" } else { "FAIL" }
        ),
    ];
    if let Some(profile) = report.deployment_profile.as_ref() {
        lines.push(format!("Deployment Profile: {profile}"));
    }
    if let Some(maturity) = report.operational_maturity.as_ref() {
        lines.push(format!("Operational Maturity: {maturity}"));
    }
    if !report.findings.is_empty() {
        lines.push(String::new());
        lines.push("Findings:".into());
        for item in &report.findings {
            lines.push(format!(
                "  [{:?}] {} — {}",
                item.severity, item.code, item.message
            ));
        }
    }
    if !report.recommended_actions.is_empty() {
        lines.push(String::new());
        lines.push("Recommended Actions:".into());
        for action in &report.recommended_actions {
            lines.push(format!("  - {action}"));
        }
    }
    lines.join("\n")
}
