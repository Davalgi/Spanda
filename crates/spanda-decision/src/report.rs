//! Decision audit trail report types.

use serde::{Deserialize, Serialize};

/// Evidence attached to a decision record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DecisionEvidence {
    #[serde(flatten)]
    pub fields: serde_json::Map<String, serde_json::Value>,
}

/// Single autonomous decision event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub version: u32,
    pub decision_id: String,
    pub mission: Option<String>,
    pub timestamp_ms: f64,
    pub decision: String,
    pub reason: String,
    pub evidence: DecisionEvidence,
    #[serde(default)]
    pub alternatives_considered: Vec<serde_json::Value>,
    #[serde(default)]
    pub safety_checks: Vec<serde_json::Value>,
    #[serde(default)]
    pub action: Option<serde_json::Value>,
    pub source_event: String,
}

/// Ordered decision timeline for a mission session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionTimeline {
    pub source: String,
    pub decisions: Vec<DecisionRecord>,
}

/// Linked chain from mission intent to executed action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionChain {
    pub mission: Option<String>,
    pub records: Vec<DecisionRecord>,
}

/// Full decision audit report for a trace file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionAuditReport {
    pub source: String,
    pub decision_count: usize,
    pub timeline: DecisionTimeline,
    pub chains: Vec<DecisionChain>,
}

/// Format decision audit output.
pub fn format_decision_audit(report: &DecisionAuditReport, json: bool) -> String {
    // Description:
    //     Render a decision audit report for CLI output.
    //
    // Parameters:
    // - `report` — parsed decision audit
    // - `json` — pretty JSON when true
    //
    // Returns:
    // Formatted output string.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_decision_audit(&report, false);

    if json {
        return serde_json::to_string_pretty(report).unwrap_or_default();
    }
    let mut out = String::new();
    out.push_str(&format!(
        "Decision audit: {} ({} decisions)\n",
        report.source, report.decision_count
    ));
    for record in &report.timeline.decisions {
        out.push_str(&format!(
            "\n- [{}] {} — {}\n  reason: {}\n",
            record.decision_id, record.decision, record.source_event, record.reason
        ));
    }
    out
}

/// Format human-readable decision explanations.
pub fn format_decision_explanations(report: &DecisionAuditReport) -> String {
    // Description:
    //     Render decision explanations in plain language.
    //
    // Parameters:
    // - `report` — decision audit report
    //
    // Returns:
    // Multi-line explanation text.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_decision_explanations(&report);

    let mut out = String::new();
    out.push_str(&format!("Decision explanations for {}\n", report.source));
    for record in &report.timeline.decisions {
        out.push_str(&format!(
            "\nAt T+{:.0}ms the system decided to '{}' because {} (event: {}).\n",
            record.timestamp_ms, record.decision, record.reason, record.source_event
        ));
        if !record.safety_checks.is_empty() {
            out.push_str(&format!(
                "  Safety checks: {}\n",
                serde_json::to_string(&record.safety_checks).unwrap_or_default()
            ));
        }
    }
    out
}
