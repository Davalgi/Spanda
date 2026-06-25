//! Contract verification report types and formatters.

use serde::{Deserialize, Serialize};

/// Single contract clause check result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractCheck {
    pub name: String,
    pub category: String,
    pub passed: bool,
    pub detail: String,
}

/// Per-mission contract summary derived from program declarations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionContractReport {
    pub name: String,
    pub kind: String,
    pub objectives: Vec<String>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<String>,
    pub invariants: Vec<String>,
    pub guarantees: Vec<String>,
    pub continuity_aligned: bool,
    pub recovery_aligned: bool,
    pub safety_aligned: bool,
}

/// Full contract verification report for a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractVerificationReport {
    pub program: String,
    pub passed: bool,
    pub contracts: Vec<MissionContractReport>,
    pub checks: Vec<ContractCheck>,
    pub issues: Vec<String>,
}

/// Format a contract report for CLI output.
pub fn format_contract_report(report: &ContractVerificationReport, json: bool) -> String {
    // Description:
    //     Format contract verification output as JSON or human-readable text.
    //
    // Parameters:
    // - `report` — verification result to render
    //
    // Returns:
    // Formatted string for stdout.
    //
    // Options:
    // - `json` — emit pretty JSON when true
    //
    // Example:
    // let text = format_contract_report(&report, false);

    if json {
        return serde_json::to_string_pretty(report).unwrap_or_default();
    }

    let mut out = String::new();
    out.push_str(&format!(
        "Mission contract verification: {}\n",
        if report.passed { "PASSED" } else { "FAILED" }
    ));
    out.push_str(&format!("Contracts: {}\n", report.contracts.len()));
    for contract in &report.contracts {
        out.push_str(&format!(
            "\n## {} ({}) — safety:{} continuity:{} recovery:{}\n",
            contract.name,
            contract.kind,
            if contract.safety_aligned { "ok" } else { "gap" },
            if contract.continuity_aligned {
                "ok"
            } else {
                "gap"
            },
            if contract.recovery_aligned {
                "ok"
            } else {
                "gap"
            },
        ));
        if !contract.objectives.is_empty() {
            out.push_str("Objectives:\n");
            for item in &contract.objectives {
                out.push_str(&format!("  - {item}\n"));
            }
        }
        if !contract.constraints.is_empty() {
            out.push_str("Constraints:\n");
            for item in &contract.constraints {
                out.push_str(&format!("  - {item}\n"));
            }
        }
    }
    if !report.checks.is_empty() {
        out.push_str("\nChecks:\n");
        for check in &report.checks {
            let mark = if check.passed { "ok" } else { "FAIL" };
            out.push_str(&format!(
                "  [{mark}] {} ({}) — {}\n",
                check.name, check.category, check.detail
            ));
        }
    }
    if !report.issues.is_empty() {
        out.push_str("\nIssues:\n");
        for issue in &report.issues {
            out.push_str(&format!("  * {issue}\n"));
        }
    }
    out
}
