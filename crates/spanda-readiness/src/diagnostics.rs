//! Readiness diagnostics for CLI, LSP, and CI integration.

use crate::engine::evaluate_readiness_with_runtime;
use crate::runtime::build_runtime_context;
use crate::spans::line_column_for_issue;
use crate::types::{ReadinessOptions, ReadinessSeverity};
use spanda_ast::nodes::Program;
use spanda_capability::VerificationDiagnostic;

/// Collect span-aware readiness diagnostics for IDE and `spanda check --readiness-json`.
pub fn collect_readiness_diagnostics(
    program: &Program,
    options: &ReadinessOptions,
) -> Vec<VerificationDiagnostic> {
    // Description:
    //     Collect readiness diagnostics.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     options: &ReadinessOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Vec<VerificationDiagnostic>
    //         Return value from `collect_readiness_diagnostics`.
    //
    // Example:

    //     let result = spanda_readiness::diagnostics::collect_readiness_diagnostics(progra, options);

    let runtime = if options.include_runtime {
        Some(build_runtime_context(program, options.inject_health_faults))
    } else {
        None
    };
    let report = evaluate_readiness_with_runtime(program, options, runtime.as_ref());
    report
        .issues
        .iter()
        .map(|issue| {
            let (line, column) = line_column_for_issue(program, issue);
            VerificationDiagnostic {
                message: issue.message.clone(),
                line,
                column,
                severity: severity_label(issue.severity).into(),
                category: format!("readiness:{}", issue.factor.to_ascii_lowercase()),
                suggested_fix: issue.suggested_action.clone(),
            }
        })
        .collect()
}

fn severity_label(severity: ReadinessSeverity) -> &'static str {
    // Description:
    //     Severity label.
    //
    // Inputs:
    //     severity: ReadinessSeverity
    //         Caller-supplied severity.
    //
    // Outputs:
    //     result: &'static str
    //         Return value from `severity_label`.
    //
    // Example:

    //     let result = spanda_readiness::diagnostics::severity_label(severity);

    match severity {
        ReadinessSeverity::Critical => "error",
        ReadinessSeverity::High => "error",
        ReadinessSeverity::Medium => "warning",
        ReadinessSeverity::Low => "info",
        ReadinessSeverity::Info => "info",
    }
}
