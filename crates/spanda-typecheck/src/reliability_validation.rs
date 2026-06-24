//! Compile-time reliability validation for tasks and resource budgets.
//!
use crate::Diagnostic;
use spanda_ast::foundations::{ResourceBudgetDecl, TaskDecl, TaskPriority};
use spanda_ast::nodes::Span;

pub fn validate_task_timing(task: &TaskDecl) -> Vec<Diagnostic> {
    // Description:
    //     Validate task timing.
    //
    // Inputs:
    //     ask: &TaskDecl
    //         Caller-supplied ask.
    //
    // Outputs:
    //     result: Vec<Diagnostic>
    //         Return value from `validate_task_timing`.
    //
    // Example:

    //     let result = spanda_typecheck::reliability_validation::validate_task_timing(ask);

    let TaskDecl::TaskDecl {
        name,
        interval_ms,
        deadline_ms,
        jitter_ms_max,
        span,
        ..
    } = task;
    let mut diags = Vec::new();

    if *interval_ms <= 0.0 {
        diags.push(Diagnostic {
            message: format!(
                "Task '{name}' period must be positive (got {interval_ms}ms). Suggestion: use `every 10ms` or larger."
            ),
            line: span.start.line,
            column: span.start.column,
        });
    }

    if let Some(deadline) = deadline_ms {
        if *deadline <= 0.0 {
            diags.push(Diagnostic {
                message: format!("Task '{name}' deadline must be positive (got {deadline}ms)."),
                line: span.start.line,
                column: span.start.column,
            });
        } else if *deadline > *interval_ms {
            diags.push(Diagnostic {
                message: format!(
                    "Task '{name}' deadline ({deadline}ms) must be <= period ({interval_ms}ms). Suggestion: increase period or reduce deadline."
                ),
                line: span.start.line,
                column: span.start.column,
            });
        }
    }

    if let Some(jitter) = jitter_ms_max {
        if *jitter < 0.0 {
            diags.push(Diagnostic {
                message: format!("Task '{name}' jitter must be non-negative."),
                line: span.start.line,
                column: span.start.column,
            });
        }
        let slack = deadline_ms.unwrap_or(*interval_ms);
        if *jitter > slack {
            diags.push(Diagnostic {
                message: format!(
                    "Task '{name}' jitter ({jitter}ms) exceeds allowable slack ({slack}ms). Suggestion: reduce jitter or increase deadline/period."
                ),
                line: span.start.line,
                column: span.start.column,
            });
        }
    }
    diags
}

pub fn validate_task_priority(task: &TaskDecl) -> Vec<Diagnostic> {
    // Description:
    //     Validate task priority.
    //
    // Inputs:
    //     ask: &TaskDecl
    //         Caller-supplied ask.
    //
    // Outputs:
    //     result: Vec<Diagnostic>
    //         Return value from `validate_task_priority`.
    //
    // Example:

    //     let result = spanda_typecheck::reliability_validation::validate_task_priority(ask);

    let TaskDecl::TaskDecl {
        name,
        priority,
        isolated,
        span,
        ..
    } = task;
    let mut diags = Vec::new();

    if *isolated && !matches!(priority, TaskPriority::Critical | TaskPriority::High) {
        diags.push(Diagnostic {
            message: format!(
                "Task '{name}' is marked isolated but priority is {priority:?}. Suggestion: use `critical isolated` or `high isolated`."
            ),
            line: span.start.line,
            column: span.start.column,
        });
    }
    diags
}

pub fn validate_resource_budget(budget: &ResourceBudgetDecl, span: Span) -> Vec<Diagnostic> {
    // Description:
    //     Validate resource budget.
    //
    // Inputs:
    //     budge: &ResourceBudgetDecl
    //         Caller-supplied budge.
    //     span: Span
    //         Caller-supplied span.
    //
    // Outputs:
    //     result: Vec<Diagnostic>
    //         Return value from `validate_resource_budget`.
    //
    // Example:

    //     let result = spanda_typecheck::reliability_validation::validate_resource_budget(budge, span);

    let ResourceBudgetDecl::ResourceBudgetDecl {
        battery_pct_max,
        memory_mb_max,
        cpu_pct_max,
        gpu_pct_max,
        network_mbps_max,
        storage_mb_max,
        ..
    } = budget;
    let mut diags = Vec::new();
    let check_pct = |label: &str, value: Option<f64>, diags: &mut Vec<Diagnostic>| {
        if let Some(v) = value {
            if v <= 0.0 || v > 100.0 {
                diags.push(Diagnostic {
                    message: format!("Resource budget {label} must be in (0, 100] (got {v})."),
                    line: span.start.line,
                    column: span.start.column,
                });
            }
        }
    };
    check_pct("cpu", *cpu_pct_max, &mut diags);
    check_pct("gpu", *gpu_pct_max, &mut diags);
    check_pct("battery", *battery_pct_max, &mut diags);

    for (label, value) in [
        ("memory", *memory_mb_max),
        ("network", *network_mbps_max),
        ("storage", *storage_mb_max),
    ] {
        if let Some(v) = value {
            if v <= 0.0 {
                diags.push(Diagnostic {
                    message: format!("Resource budget {label} must be positive (got {v})."),
                    line: span.start.line,
                    column: span.start.column,
                });
            }
        }
    }

    if let (Some(cpu), Some(gpu)) = (cpu_pct_max, gpu_pct_max) {
        if cpu + gpu > 100.0 {
            diags.push(Diagnostic {
                message: format!(
                    "Resource budget cpu ({cpu}%) + gpu ({gpu}%) exceeds 100%. Suggestion: reduce one ceiling."
                ),
                line: span.start.line,
                column: span.start.column,
            });
        }
    }
    diags
}

pub fn validate_pipeline(pipeline: &spanda_ast::foundations::PipelineDecl) -> Vec<Diagnostic> {
    // Description:
    //     Validate pipeline.
    //
    // Inputs:
    //     pipeline: &spanda_ast::foundations::PipelineDecl
    //         Caller-supplied pipeline.
    //
    // Outputs:
    //     result: Vec<Diagnostic>
    //         Return value from `validate_pipeline`.
    //
    // Example:

    //     let result = spanda_typecheck::reliability_validation::validate_pipeline(pipeline);

    use spanda_ast::foundations::PipelineDecl;

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

pub fn validate_watchdog(
    watchdog: &spanda_ast::foundations::WatchdogDecl,
    task_names: &[String],
) -> Vec<Diagnostic> {
    // Description:
    //     Validate watchdog.
    //
    // Inputs:
    //     watchdog: &spanda_ast::foundations::WatchdogDecl
    //         Caller-supplied watchdog.
    //     ask_names: &[String]
    //         Caller-supplied ask names.
    //
    // Outputs:
    //     result: Vec<Diagnostic>
    //         Return value from `validate_watchdog`.
    //
    // Example:

    //     let result = spanda_typecheck::reliability_validation::validate_watchdog(watchdog, ask_names);

    use spanda_ast::foundations::WatchdogDecl;

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
    // Description:
    //     Validate recover.
    //
    // Inputs:
    //     recover: &spanda_ast::foundations::RecoverDecl
    //         Caller-supplied recover.
    //
    // Outputs:
    //     result: Vec<Diagnostic>
    //         Return value from `validate_recover`.
    //
    // Example:

    //     let result = spanda_typecheck::reliability_validation::validate_recover(recover);

    use spanda_ast::foundations::RecoverDecl;
    use spanda_ast::nodes::Stmt;

    let RecoverDecl::RecoverDecl {
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

pub fn resolve_std_import(path: &str) -> bool {
    // Description:
    //     Resolve std import.
    //
    // Inputs:
    //     path: &str
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: bool
    //         Return value from `resolve_std_import`.
    //
    // Example:

    //     let result = spanda_typecheck::reliability_validation::resolve_std_import(path);

    crate::type_system::std_namespaces().contains_key(path)
}
