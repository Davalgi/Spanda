//! Compile-time reliability validation for tasks and resource budgets.
//!
use crate::Diagnostic;
use spanda_ast::foundations::{ResourceBudgetDecl, TaskDecl, TaskPriority};
use spanda_ast::nodes::Span;

pub fn validate_task_timing(task: &TaskDecl) -> Vec<Diagnostic> {
    // Validate periodic task period, deadline, and jitter constraints.
    //
    // Parameters:
    // - `task` — task declaration to inspect
    //
    // Returns:
    // Diagnostics for invalid timing configuration.
    //
    // Options:
    // None.
    //
    // Example:
    // let diags = validate_task_timing(&task);

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
    // Validate priority/isolation combinations for safety-critical tasks.
    //
    // Parameters:
    // - `task` — task declaration to inspect
    //
    // Returns:
    // Diagnostics for invalid priority configuration.
    //
    // Options:
    // None.
    //
    // Example:
    // let diags = validate_task_priority(&task);

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
    // Validate per-task resource budget ceilings for conflicts and invalid values.
    //
    // Parameters:
    // - `budget` — resource budget block
    // - `span` — enclosing task span for diagnostics
    //
    // Returns:
    // Diagnostics for invalid or conflicting budgets.
    //
    // Options:
    // None.
    //
    // Example:
    // let diags = validate_resource_budget(&budget, task_span);

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

pub fn resolve_std_import(path: &str) -> bool {
    // Check whether an import path refers to a registered std module namespace.
    //
    // Parameters:
    // - `path` — import path (e.g. `std.time`)
    //
    // Returns:
    // true when the path is a known std namespace key.
    //
    // Options:
    // None.
    //
    // Example:
    // assert!(resolve_std_import("std.time"));

    crate::type_system::std_namespaces().contains_key(path)
}
