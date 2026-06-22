//! Real-time reliability validation helpers for tasks, pipelines, and watchdogs.
//!
use spanda_ast::foundations::{PipelineDecl, WatchdogDecl};
use spanda_ast::nodes::Stmt;
use spanda_typecheck::Diagnostic;

pub use spanda_typecheck::{
    validate_resource_budget, validate_task_priority, validate_task_timing,
};

pub fn validate_pipeline(pipeline: &PipelineDecl) -> Vec<Diagnostic> {
    // Validate pipeline latency budget configuration.
    //
    // Parameters:
    // - `pipeline` — pipeline declaration
    //
    // Returns:
    // Diagnostics for invalid pipeline budgets.
    //
    // Options:
    // None.
    //
    // Example:
    // let diags = validate_pipeline(&pipeline);

    let PipelineDecl::PipelineDecl {
        name,
        budget_ms,
        span,
        ..
    } = pipeline;
    let mut diags = Vec::new();

    if *budget_ms <= 0.0 {
        diags.push(Diagnostic {
            message: format!("Pipeline '{name}' budget must be positive (got {budget_ms}ms)."),
            line: span.start.line,
            column: span.start.column,
        });
    }
    diags
}

pub fn validate_watchdog(watchdog: &WatchdogDecl, task_names: &[String]) -> Vec<Diagnostic> {
    // Validate watchdog timeout and monitored task target.
    //
    // Parameters:
    // - `watchdog` — watchdog declaration
    // - `task_names` — known task names in the robot block
    //
    // Returns:
    // Diagnostics for invalid watchdog configuration.
    //
    // Options:
    // None.
    //
    // Example:
    // let diags = validate_watchdog(&watchdog, &task_names);

    let WatchdogDecl::WatchdogDecl {
        name,
        target,
        timeout_ms,
        span,
        ..
    } = watchdog;
    let mut diags = Vec::new();

    if *timeout_ms <= 0.0 {
        diags.push(Diagnostic {
            message: format!("Watchdog '{name}' timeout must be positive."),
            line: span.start.line,
            column: span.start.column,
        });
    }

    if let Some(task) = target {
        if !task_names.iter().any(|n| n == task) {
            diags.push(Diagnostic {
                message: format!(
                    "Watchdog '{name}' target task '{task}' not found. Suggestion: declare the task before the watchdog or fix the task name."
                ),
                line: span.start.line,
                column: span.start.column,
            });
        }
    }
    diags
}

pub fn validate_recover(recover: &spanda_ast::foundations::RecoverDecl) -> Vec<Diagnostic> {
    // Validate recovery handler safety requirements.
    //
    // Parameters:
    // - `recover` — recovery handler declaration
    //
    // Returns:
    // Diagnostics when recovery blocks omit safety actions for runtime errors.
    //
    // Options:
    // None.
    //
    // Example:
    // let diags = validate_recover(&recover);

    let spanda_ast::foundations::RecoverDecl::RecoverDecl {
        error_name,
        body,
        span,
        ..
    } = recover;
    let mut diags = Vec::new();

    if error_name == "RuntimeError" {
        let has_safe_action = body.iter().any(|stmt| {
            matches!(
                stmt,
                Stmt::StopAllActuatorsStmt { .. } | Stmt::EnterModeStmt { .. }
            )
        });
        if !has_safe_action {
            diags.push(Diagnostic {
                message: "Recovery from RuntimeError should stop actuators or enter degraded mode. Suggestion: add stop_all_actuators() or enter degraded_mode;".into(),
                line: span.start.line,
                column: span.start.column,
            });
        }
    }
    diags
}
