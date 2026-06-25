//! Explainability report types.

use serde::{Deserialize, Serialize};

/// One explainability section (readiness, verify, safety, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplainSection {
    pub topic: String,
    pub summary: String,
    pub details: Vec<String>,
}

/// Full explainability report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplainReport {
    pub program: String,
    pub sections: Vec<ExplainSection>,
}

/// Format explainability output.
pub fn format_explain_report(report: &ExplainReport, json: bool) -> String {
    // Description:
    //     Render explainability report for CLI output.
    //
    // Parameters:
    // - `report` — explainability report
    // - `json` — JSON when true
    //
    // Returns:
    // Formatted string.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_explain_report(&report, false);

    if json {
        return serde_json::to_string_pretty(report).unwrap_or_default();
    }
    let mut out = String::new();
    out.push_str(&format!("Explain: {}\n", report.program));
    for section in &report.sections {
        out.push_str(&format!("\n## {}\n{}\n", section.topic, section.summary));
        for detail in &section.details {
            out.push_str(&format!("- {detail}\n"));
        }
    }
    out
}
