//! Report formatting for readiness and safety outputs.

use crate::auditor::SafetyAuditReport;
use crate::failure::FailureAnalysisReport;
use crate::mission::MissionVerificationReport;
use crate::root_cause::RootCauseReport;
use crate::safety_report::SafetyCaseReport;
use crate::types::FleetReadinessReport;
use crate::types::{ReadinessReport, ReportFormat, TwinReadinessStatus};

/// Format a readiness report for CLI output.
pub fn format_readiness(report: &ReadinessReport, format: ReportFormat) -> String {
    // Description:
    //     Format readiness.
    //
    // Inputs:
    //     repor: &ReadinessReport
    //         Caller-supplied repor.
    //     forma: ReportFormat
    //         Caller-supplied forma.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_readiness`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_readiness(repor, forma);

    match format {
        ReportFormat::Json => serde_json::to_string_pretty(report).unwrap_or_default(),
        ReportFormat::Markdown => format_readiness_markdown(report),
        ReportFormat::Html => format_readiness_html(report),
        ReportFormat::Text => format_readiness_text(report),
    }
}

fn format_readiness_text(report: &ReadinessReport) -> String {
    // Description:
    //     Format readiness text.
    //
    // Inputs:
    //     repor: &ReadinessReport
    //         Caller-supplied repor.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_readiness_text`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_readiness_text(repor);

    let mut out = String::new();
    out.push_str(&format!(
        "Mission Ready: {}\n",
        if report.mission_ready { "YES" } else { "NO" }
    ));
    out.push_str(&format!(
        "Score: {}/{}\n",
        report.score.total, report.score.maximum
    ));
    out.push_str(&format!("Status: {:?}\n", report.status));
    if !report.issues.is_empty() {
        out.push_str("\nIssues:\n");
        for issue in &report.issues {
            out.push_str(&format!("* {}\n", issue.message));
        }
    }
    out
}

fn format_readiness_markdown(report: &ReadinessReport) -> String {
    // Description:
    //     Format readiness markdown.
    //
    // Inputs:
    //     repor: &ReadinessReport
    //         Caller-supplied repor.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_readiness_markdown`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_readiness_markdown(repor);

    let mut out = String::new();
    out.push_str("# Readiness Report\n\n");
    out.push_str(&format!(
        "**Mission Ready:** {}\n\n",
        if report.mission_ready { "YES" } else { "NO" }
    ));
    out.push_str(&format!(
        "**Score:** {}/{}\n\n",
        report.score.total, report.score.maximum
    ));
    if !report.issues.is_empty() {
        out.push_str("## Issues\n\n");
        for issue in &report.issues {
            out.push_str(&format!("- {}\n", issue.message));
        }
    }
    out
}

fn format_readiness_html(report: &ReadinessReport) -> String {
    // Description:
    //     Format readiness html.
    //
    // Inputs:
    //     repor: &ReadinessReport
    //         Caller-supplied repor.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_readiness_html`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_readiness_html(repor);

    let issues: String = report
        .issues
        .iter()
        .map(|i| format!("<li>{}</li>", html_escape(&i.message)))
        .collect();
    format!(
        "<!DOCTYPE html><html><head><title>Readiness Report</title></head><body>\
         <h1>Readiness Report</h1>\
         <p><strong>Mission Ready:</strong> {}</p>\
         <p><strong>Score:</strong> {}/{}</p>\
         <h2>Issues</h2><ul>{issues}</ul>\
         </body></html>",
        if report.mission_ready { "YES" } else { "NO" },
        report.score.total,
        report.score.maximum
    )
}

/// Format safety case report.
pub fn format_safety_report(report: &SafetyCaseReport, format: ReportFormat) -> String {
    // Description:
    //     Format safety report.
    //
    // Inputs:
    //     repor: &SafetyCaseReport
    //         Caller-supplied repor.
    //     forma: ReportFormat
    //         Caller-supplied forma.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_safety_report`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_safety_report(repor, forma);

    match format {
        ReportFormat::Json => serde_json::to_string_pretty(report).unwrap_or_default(),
        ReportFormat::Markdown => {
            let mut out = String::from("# Safety Case Report\n\n");
            out.push_str(&format!(
                "**Deployable:** {}\n\n",
                if report.deployable { "YES" } else { "NO" }
            ));
            if !report.known_risks.is_empty() {
                out.push_str("## Known Risks\n\n");
                for risk in &report.known_risks {
                    out.push_str(&format!("- {risk}\n"));
                }
            }
            out
        }
        ReportFormat::Html => format!(
            "<!DOCTYPE html><html><head><title>Safety Report</title></head><body>\
             <h1>Safety Case Report</h1><p>Deployable: {}</p></body></html>",
            if report.deployable { "YES" } else { "NO" }
        ),
        ReportFormat::Text => format!(
            "Deployable: {}\nKnown risks: {}\n",
            if report.deployable { "YES" } else { "NO" },
            report.known_risks.len()
        ),
    }
}

/// Format failure analysis report.
pub fn format_failure_analysis(report: &FailureAnalysisReport) -> String {
    // Description:
    //     Format failure analysis.
    //
    // Inputs:
    //     repor: &FailureAnalysisReport
    //         Caller-supplied repor.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_failure_analysis`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_failure_analysis(repor);

    let mut out = String::new();
    for impact in &report.impacts {
        out.push_str(&format!("If {} fails:\n", impact.component));
        out.push_str(&format!("  {}\n", impact.mitigation));
        out.push('\n');
    }
    out
}

/// Format fleet readiness report.
pub fn format_fleet_readiness(report: &FleetReadinessReport) -> String {
    // Description:
    //     Format fleet readiness.
    //
    // Inputs:
    //     repor: &FleetReadinessReport
    //         Caller-supplied repor.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_fleet_readiness`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_fleet_readiness(repor);

    format!(
        "Fleet Score: {}/100\nHealthy Robots:\n{}\nDegraded Robots:\n{}\nMission Capacity:\n{}%\n",
        report.fleet_score,
        report.healthy_robots,
        report.degraded_robots,
        report.mission_capacity_percent
    )
}

/// Format mission verification reports.
pub fn format_mission_verification(reports: &[MissionVerificationReport]) -> String {
    // Description:
    //     Format mission verification.
    //
    // Inputs:
    //     reports: &[MissionVerificationReport]
    //         Caller-supplied reports.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_mission_verification`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_mission_verification(reports);

    let mut out = String::new();
    for r in reports {
        out.push_str(&format!(
            "Mission {:?} on {:?}: {}\n",
            r.mission_name,
            r.robot,
            if r.achievable {
                "ACHIEVABLE"
            } else {
                "BLOCKED"
            }
        ));
        for issue in &r.issues {
            out.push_str(&format!("  - {issue}\n"));
        }
    }
    out
}

/// Format root cause report.
pub fn format_root_cause(report: &RootCauseReport) -> String {
    // Description:
    //     Format root cause.
    //
    // Inputs:
    //     repor: &RootCauseReport
    //         Caller-supplied repor.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_root_cause`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_root_cause(repor);

    let mut out = format!(
        "Root Cause\n{}\n\nContributing Factors\n",
        report.root_cause
    );
    for f in &report.contributing_factors {
        out.push_str(&format!("* {f}\n"));
    }
    out.push_str("\nTimeline\n");
    for e in report.timeline.iter().take(20) {
        out.push_str(&format!(
            "  T+{:.0}ms {} — {}\n",
            e.sim_time_ms, e.event, e.detail
        ));
    }
    out.push_str("\nRecommended Actions\n");
    for a in &report.recommended_actions {
        out.push_str(&format!("* {a}\n"));
    }
    out
}

/// Format safety audit report.
pub fn format_audit(report: &SafetyAuditReport) -> String {
    // Description:
    //     Format audit.
    //
    // Inputs:
    //     repor: &SafetyAuditReport
    //         Caller-supplied repor.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_audit`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_audit(repor);

    let mut out = String::new();
    out.push_str(&format!(
        "Critical: {}  High: {}  Medium: {}  Low: {}\n",
        report.critical_count, report.high_count, report.medium_count, report.low_count
    ));
    for f in &report.findings {
        out.push_str(&format!(
            "[{:?}] {} — {}\n",
            f.severity, f.category, f.message
        ));
    }
    out
}

/// Format twin readiness status.
pub fn format_twin_readiness(status: &TwinReadinessStatus) -> String {
    // Description:
    //     Format twin readiness.
    //
    // Inputs:
    //     status: &TwinReadinessStatus
    //         Caller-supplied status.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_twin_readiness`.
    //
    // Example:

    //     let result = spanda_readiness::report::format_twin_readiness(status);

    format!(
        "Physical Ready: {}\nTwin Ready: {}\nConfiguration Drift: {}\nCapability Drift: {}\nHealth Drift: {}\n",
        status.physical_ready,
        status.twin_ready,
        status.configuration_drift.len(),
        status.capability_drift.len(),
        status.health_drift.len()
    )
}

fn html_escape(s: &str) -> String {
    // Description:
    //     Html escape.
    //
    // Inputs:
    //     s: &str
    //         Caller-supplied s.
    //
    // Outputs:
    //     result: String
    //         Return value from `html_escape`.
    //
    // Example:

    //     let result = spanda_readiness::report::html_escape(s);

    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
