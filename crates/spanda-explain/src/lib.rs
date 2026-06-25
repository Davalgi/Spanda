//! Explainability reports for Spanda programs, verification, and traces.

mod explain;
mod report;

pub use explain::{
    explain_program, explain_readiness, explain_safety, explain_trace, explain_verify,
};
pub use report::{format_explain_report, ExplainReport, ExplainSection};
