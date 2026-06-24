//! SIR lowering and embedded test entry points.
//!
use spanda_error::SpandaError;
use spanda_sir::{lower_program, SirProgram};
use spanda_typecheck::ModuleRegistry;

use crate::compile::compile;
use crate::run::run_tests_with_registry as driver_run_tests_with_registry;

pub fn lower_to_sir(source: &str) -> Result<SirProgram, SpandaError> {
    // Description:
    //     Lower to sir.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: Result<SirProgram, SpandaError>
    //         Return value from `lower_to_sir`.
    //
    // Example:

    //     let result = spanda_driver::pipeline::lower_to_sir(source);

    let program = compile(source)?.program;
    Ok(lower_program(&program))
}

pub fn run_tests(source: &str) -> Result<spanda_interpreter::TestRunResult, SpandaError> {
    // Description:
    //     Run tests.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: Result<spanda_interpreter::TestRunResult, SpandaError>
    //         Return value from `run_tests`.
    //
    // Example:

    //     let result = spanda_driver::pipeline::run_tests(source);

    driver_run_tests_with_registry(source, &ModuleRegistry::new())
}
