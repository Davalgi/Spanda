//! Report formatting for CLI and API output.
//!
use crate::validate::{ComplianceCheckReport, GovernanceValidationReport};
use crate::entity_governance::EntityGovernanceReport;
use serde_json;

pub fn format_compliance_report(report: &ComplianceCheckReport, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".into());
    }
    let mut lines = vec![
        "Compliance Check Report".into(),
        "=======================".into(),
        format!(
            "Result: {}",
            if report.passed { "PASS" } else { "FAIL" }
        ),
        format!(
            "Entities: {} checked, {} passed, {} failed",
            report.summary.entities_checked,
            report.summary.entities_passed,
            report.summary.entities_failed
        ),
        format!("Warnings: {}", report.summary.warnings),
        format!("Missing: {}", report.summary.missing),
        String::new(),
    ];
    if !report.warnings.is_empty() {
        lines.push("Warnings:".into());
        for item in &report.warnings {
            lines.push(format!("  [{}] {} — {}", item.code, item.entity_id.as_deref().unwrap_or("-"), item.message));
        }
        lines.push(String::new());
    }
    if !report.missing_requirements.is_empty() {
        lines.push("Missing Requirements:".into());
        for item in &report.missing_requirements {
            lines.push(format!("  [{}] {} — {}", item.code, item.entity_id.as_deref().unwrap_or("-"), item.message));
        }
        lines.push(String::new());
    }
    if !report.recommended_actions.is_empty() {
        lines.push("Recommended Actions:".into());
        for action in &report.recommended_actions {
            lines.push(format!("  - {action}"));
        }
    }
    lines.join("\n")
}

pub fn format_governance_validation(report: &GovernanceValidationReport, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".into());
    }
    let mut lines = vec![
        "Governance Validation Report".into(),
        "============================".into(),
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
    lines.push(String::new());
    if !report.findings.is_empty() {
        lines.push("Findings:".into());
        for item in &report.findings {
            lines.push(format!(
                "  [{:?}] {} — {}",
                item.severity,
                item.code,
                item.message
            ));
        }
        lines.push(String::new());
    }
    if !report.recommended_actions.is_empty() {
        lines.push("Recommended Actions:".into());
        for action in &report.recommended_actions {
            lines.push(format!("  - {action}"));
        }
    }
    lines.join("\n")
}

pub fn format_entity_governance_report(report: &EntityGovernanceReport, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".into());
    }
    format!(
        "Entity {} governance: {}\nFindings: {}",
        report.entity_id,
        if report.passed { "PASS" } else { "FAIL" },
        report.findings.len()
    )
}

pub fn format_governance_report(
    compliance: &ComplianceCheckReport,
    validation: &GovernanceValidationReport,
    json: bool,
) -> String {
    if json {
        return serde_json::to_string_pretty(&serde_json::json!({
            "compliance": compliance,
            "governance": validation,
        }))
        .unwrap_or_else(|_| "{}".into());
    }
    format!(
        "{}\n\n{}",
        format_compliance_report(compliance, false),
        format_governance_validation(validation, false)
    )
}
