//! Recovery path coverage reporting for known failure modes.

use crate::{
    extract_continuity_policies, extract_recovery_policies, RecoveryContext, RecoveryLevel,
    RecoveryPlanner,
};
use serde::{Deserialize, Serialize};
use spanda_ast::nodes::Program;

/// Recovery coverage status for a failure mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryCoverageStatus {
    Covered,
    PartiallyCovered,
    Uncovered,
}

/// Recovery plan mapping for a failure mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanSummary {
    pub failure: String,
    pub policy: Option<String>,
    pub actions: Vec<String>,
}

/// Gap where recovery is missing or incomplete.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryGap {
    pub failure: String,
    pub recommendation: String,
}

/// Full recovery coverage report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryCoverageReport {
    pub program: String,
    pub coverage_pct: u32,
    pub known_failures: usize,
    pub covered: usize,
    pub partially_covered: usize,
    pub uncovered: usize,
    pub recovery_plans: Vec<RecoveryPlanSummary>,
    pub missing_paths: Vec<RecoveryGap>,
}

const KNOWN_FAILURES: &[&str] = &[
    "gps_loss",
    "battery_critical",
    "connectivity_loss",
    "sensor_failure",
    "actuator_failure",
    "provider_timeout",
    "fleet_peer_loss",
    "swarm_member_loss",
    "package_unavailable",
    "human_approval_timeout",
    "robot_failed",
];

fn failure_keywords(failure: &str) -> Vec<&'static str> {
    // Description:
    //     Map catalog failure ids to policy condition keywords.
    //
    // Parameters:
    // - `failure` — failure mode id
    //
    // Returns:
    // Keyword list used for policy matching.
    //
    // Options:
    // None.
    //
    // Example:
    // let keys = failure_keywords("gps_loss");

    match failure {
        "gps_loss" => vec!["gps", "navigation"],
        "battery_critical" => vec!["battery", "power"],
        "connectivity_loss" => vec!["connect", "network", "wifi"],
        "sensor_failure" => vec!["sensor", "lidar", "camera"],
        "actuator_failure" => vec!["actuator", "drive", "motor"],
        "provider_timeout" => vec!["provider", "timeout"],
        "fleet_peer_loss" => vec!["fleet", "peer"],
        "swarm_member_loss" => vec!["swarm", "member"],
        "package_unavailable" => vec!["package", "provider"],
        "human_approval_timeout" => vec!["approval", "operator"],
        "robot_failed" => vec!["robot", "failed"],
        _ => vec![],
    }
}

fn policy_matches_failure(policy_text: &str, failure: &str) -> bool {
    // Description:
    //     Check whether a recovery policy branch matches a failure mode.
    //
    // Parameters:
    // - `policy_text` — serialized policy branch
    // - `failure` — failure id
    //
    // Returns:
    // True when keywords overlap.
    //
    // Options:
    // None.
    //
    // Example:
    // let ok = policy_matches_failure("on robot.failed", "robot_failed");

    let lower = policy_text.to_ascii_lowercase();
    failure_keywords(failure)
        .iter()
        .any(|keyword| lower.contains(keyword))
}

/// Evaluate recovery coverage for known failure modes.
pub fn evaluate_recovery_coverage(program: &Program, source_label: &str) -> RecoveryCoverageReport {
    // Description:
    //     Score recovery path coverage from recovery and continuity policies.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    //
    // Returns:
    // Recovery coverage report.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = evaluate_recovery_coverage(&program, "rover.sd");

    let policies = extract_recovery_policies(program);
    let continuity = extract_continuity_policies(program);
    let policy_blob = format!("{policies:?}");
    let continuity_blob = format!("{continuity:?}");

    let mut recovery_plans = Vec::new();
    let mut missing_paths = Vec::new();
    let mut covered = 0usize;
    let mut partially_covered = 0usize;
    let mut uncovered = 0usize;

    for failure in KNOWN_FAILURES {
        let recovery_match = policy_matches_failure(&policy_blob, failure);
        let continuity_match = policy_matches_failure(&continuity_blob, failure);
        let status = if recovery_match {
            RecoveryCoverageStatus::Covered
        } else if continuity_match {
            RecoveryCoverageStatus::PartiallyCovered
        } else {
            RecoveryCoverageStatus::Uncovered
        };

        match status {
            RecoveryCoverageStatus::Covered => covered += 1,
            RecoveryCoverageStatus::PartiallyCovered => partially_covered += 1,
            RecoveryCoverageStatus::Uncovered => uncovered += 1,
        }

        if status == RecoveryCoverageStatus::Covered {
            let plan = RecoveryPlanner::plan(
                program,
                &RecoveryContext {
                    issue: (*failure).into(),
                    diagnosis: None,
                    classification: None,
                    level: RecoveryLevel::Level2AutomaticLowRisk,
                },
            );
            recovery_plans.push(RecoveryPlanSummary {
                failure: (*failure).into(),
                policy: Some(plan.name),
                actions: plan.actions.iter().map(|a| a.description.clone()).collect(),
            });
        } else if status == RecoveryCoverageStatus::Uncovered {
            missing_paths.push(RecoveryGap {
                failure: (*failure).into(),
                recommendation: format!("Add recovery_policy or continuity_policy for {failure}"),
            });
        }
    }

    let known_failures = KNOWN_FAILURES.len();
    let points = covered * 100 + partially_covered * 50;
    let coverage_pct = if known_failures == 0 {
        0
    } else {
        (points / known_failures) as u32
    };

    RecoveryCoverageReport {
        program: source_label.into(),
        coverage_pct,
        known_failures,
        covered,
        partially_covered,
        uncovered,
        recovery_plans,
        missing_paths,
    }
}

/// Format recovery coverage for CLI output.
pub fn format_recovery_coverage(
    report: &RecoveryCoverageReport,
    json: bool,
    markdown: bool,
) -> String {
    // Description:
    //     Render recovery coverage as JSON, markdown, or text.
    //
    // Parameters:
    // - `report` — recovery coverage report
    // - `json` — JSON when true
    // - `markdown` — markdown when true
    //
    // Returns:
    // Formatted output.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_recovery_coverage(&report, false, false);

    if json {
        return serde_json::to_string_pretty(report).unwrap_or_default();
    }
    let mut out = String::new();
    out.push_str(&format!(
        "Recovery coverage: {}% — covered {}/{} failures ({} partial, {} uncovered)\n",
        report.coverage_pct,
        report.covered,
        report.known_failures,
        report.partially_covered,
        report.uncovered
    ));
    if !report.recovery_plans.is_empty() {
        out.push_str("\nRecovery plans:\n");
        for plan in &report.recovery_plans {
            if markdown {
                out.push_str(&format!(
                    "- **{}** via {:?} — actions {:?}\n",
                    plan.failure, plan.policy, plan.actions
                ));
            } else {
                out.push_str(&format!(
                    "  {} policy={:?} actions={:?}\n",
                    plan.failure, plan.policy, plan.actions
                ));
            }
        }
    }
    if !report.missing_paths.is_empty() {
        out.push_str("\nMissing paths:\n");
        for gap in &report.missing_paths {
            out.push_str(&format!("  * {} — {}\n", gap.failure, gap.recommendation));
        }
    }
    out
}
