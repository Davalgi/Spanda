//! Runtime state for watchdogs, pipelines, retries, and recovery handlers.

use spanda_ast::foundations::{PipelineDecl, RecoverDecl, RetryDecl, WatchdogDecl};
use spanda_ast::nodes::Stmt;
use std::collections::HashMap;

/// Loaded watchdog handler ready for sim-time evaluation.
#[derive(Debug, Clone)]
pub struct WatchdogRuntime {
    pub name: String,
    pub target: Option<String>,
    pub timeout_ms: f64,
    pub body: Vec<Stmt>,
    pub last_fired_at_ms: Option<f64>,
}

/// Loaded latency-budget pipeline.
#[derive(Debug, Clone)]
pub struct PipelineRuntime {
    pub name: String,
    pub budget_ms: f64,
    pub body: Vec<Stmt>,
}

/// Loaded retry policy with optional fallback block.
#[derive(Debug, Clone)]
pub struct RetryRuntime {
    pub attempts: u32,
    pub backoff_ms: f64,
    pub body: Vec<Stmt>,
    pub fallback: Vec<Stmt>,
    pub attempt: u32,
    pub exhausted: bool,
}

impl WatchdogRuntime {
    pub fn from_decl(decl: &WatchdogDecl) -> Self {
        // Description:
        //     From decl.
        //
        // Inputs:
        //     decl: &WatchdogDecl
        //         Caller-supplied decl.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_decl`.
        //
        // Example:
        //     let result = spanda_runtime::reliability_runtime::from_decl(decl);

        // Copy declaration fields into the runtime container.
        let WatchdogDecl::WatchdogDecl {
            name,
            target,
            timeout_ms,
            body,
            ..
        } = decl;
        Self {
            name: name.clone(),
            target: target.clone(),
            timeout_ms: *timeout_ms,
            body: body.clone(),
            last_fired_at_ms: None,
        }
    }
}

impl PipelineRuntime {
    pub fn from_decl(decl: &PipelineDecl) -> Self {
        // Description:
        //     From decl.
        //
        // Inputs:
        //     decl: &PipelineDecl
        //         Caller-supplied decl.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_decl`.
        //
        // Example:
        //     let result = spanda_runtime::reliability_runtime::from_decl(decl);

        // Copy declaration fields into the runtime container.
        let PipelineDecl::PipelineDecl {
            name,
            budget_ms,
            body,
            ..
        } = decl;
        Self {
            name: name.clone(),
            budget_ms: *budget_ms,
            body: body.clone(),
        }
    }
}

impl RetryRuntime {
    pub fn from_decl(decl: &RetryDecl) -> Self {
        // Description:
        //     From decl.
        //
        // Inputs:
        //     decl: &RetryDecl
        //         Caller-supplied decl.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_decl`.
        //
        // Example:
        //     let result = spanda_runtime::reliability_runtime::from_decl(decl);

        // Copy declaration fields into the runtime container.
        let RetryDecl::RetryDecl {
            attempts,
            backoff_ms,
            body,
            fallback,
            ..
        } = decl;
        Self {
            attempts: *attempts,
            backoff_ms: *backoff_ms,
            body: body.clone(),
            fallback: fallback.clone(),
            attempt: 0,
            exhausted: false,
        }
    }
}

/// Recovery handlers keyed by error or hardware event name.
pub type RecoverHandlers = HashMap<String, Vec<Stmt>>;

/// Loaded operating mode with configuration statements.
#[derive(Debug, Clone)]
pub struct ModeRuntime {
    pub name: String,
    pub body: Vec<Stmt>,
}

impl ModeRuntime {
    pub fn from_decl(decl: &spanda_ast::foundations::ModeDecl) -> Self {
        // Description:
        //     From decl.
        //
        // Inputs:
        //     decl: &spanda_ast::foundations::ModeDecl
        //         Caller-supplied decl.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_decl`.
        //
        // Example:
        //     let result = spanda_runtime::reliability_runtime::from_decl(decl);

        // Copy declaration fields into the runtime container.
        let spanda_ast::foundations::ModeDecl::ModeDecl { name, body, .. } = decl;
        Self {
            name: name.clone(),
            body: body.clone(),
        }
    }
}

pub fn recover_handlers_from_decls(recovers: &[RecoverDecl]) -> RecoverHandlers {
    // Description:
    //     Recover handlers from decls.
    //
    // Inputs:
    //     recovers: &[RecoverDecl]
    //         Caller-supplied recovers.
    //
    // Outputs:
    //     result: RecoverHandlers
    //         Return value from `recover_handlers_from_decls`.
    //
    // Example:
    //     let result = spanda_runtime::reliability_runtime::recover_handlers_from_decls(recovers);

    // Build a lookup table for runtime dispatch.
    let mut handlers = HashMap::new();
    for decl in recovers {
        let RecoverDecl::RecoverDecl {
            error_name, body, ..
        } = decl;
        handlers.insert(error_name.clone(), body.clone());
    }
    handlers
}
