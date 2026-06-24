//! Driver helpers for targeted interpreter recovery on deployed programs.
//!
use spanda_ast::nodes::Program;
use spanda_error::SpandaError;
use spanda_interpreter::{
    execute_recovery_on_program as interpreter_execute_recovery_on_program, RecoveryRunOptions,
    RecoveryRunResult,
};

use crate::compile::compile;

/// Compile source and run interpreter-backed recovery for a failure issue.
pub fn execute_recovery_source(
    source: &str,
    issue: &str,
    options: RecoveryRunOptions,
) -> Result<RecoveryRunResult, SpandaError> {
    // Description:
    //     Execute recovery source.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     issue: &str
    //         Caller-supplied issue.
    //     options: RecoveryRunOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Result<RecoveryRunResult, SpandaError>
    //         Return value from `execute_recovery_source`.
    //
    // Example:

    //     let result = spanda_driver::recovery_run::execute_recovery_source(source, issue, options);

    let program = compile(source)?.program;
    execute_recovery_on_program(&program, issue, options)
}

/// Run interpreter-backed recovery on an already parsed program.
pub fn execute_recovery_on_program(
    program: &Program,
    issue: &str,
    options: RecoveryRunOptions,
) -> Result<RecoveryRunResult, SpandaError> {
    // Description:
    //     Execute recovery on program.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     issue: &str
    //         Caller-supplied issue.
    //     options: RecoveryRunOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Result<RecoveryRunResult, SpandaError>
    //         Return value from `execute_recovery_on_program`.
    //
    // Example:

    //     let result = spanda_driver::recovery_run::execute_recovery_on_program(progra, issue, options);

    interpreter_execute_recovery_on_program(program, issue, options)
}
