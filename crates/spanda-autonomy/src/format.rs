//! Report formatting for autonomy CLI and API.
//!
use crate::types::AutonomyReportFormat;
use serde::Serialize;

/// Format any serializable report.
pub fn format_report<T: Serialize>(value: &T, format: AutonomyReportFormat) -> String {
    match format {
        AutonomyReportFormat::Json => {
            serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into())
        }
        AutonomyReportFormat::Markdown => format_markdown_value(value),
        AutonomyReportFormat::Text => format_text_value(value),
    }
}

fn format_text_value<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).unwrap_or_default()
}

fn format_markdown_value<T: Serialize>(value: &T) -> String {
    let json = serde_json::to_value(value).unwrap_or(serde_json::json!({}));
    let mut out = String::from("# Autonomy Report\n\n");
    if let Some(obj) = json.as_object() {
        for (key, val) in obj {
            out.push_str(&format!("## {}\n\n", key));
            out.push_str(&format!(
                "```json\n{}\n```\n\n",
                serde_json::to_string_pretty(val).unwrap_or_default()
            ));
        }
    } else {
        out.push_str(&format!(
            "```json\n{}\n```\n",
            serde_json::to_string_pretty(&json).unwrap_or_default()
        ));
    }
    out
}
