//! SIR lowering and embedded test entry points.
//!
use spanda_error::SpandaError;
use spanda_sir::{lower_program, SirProgram};
use spanda_typecheck::ModuleRegistry;

use crate::compile::compile;
use crate::run::run_tests_with_registry as driver_run_tests_with_registry;

pub fn lower_to_sir(source: &str) -> Result<SirProgram, SpandaError> {
    // Compile source and lower the typed AST to SIR.
    //
    // Parameters:
    // - `source` — full `.sd` source text
    //
    // Returns:
    // Lowered SIR program, or a compile diagnostic error.
    //
    // Options:
    // None.
    //
    // Example:
    // let sir = lower_to_sir(source)?;

    let program = compile(source)?.program;
    Ok(lower_program(&program))
}

pub fn run_tests(source: &str) -> Result<spanda_interpreter::TestRunResult, SpandaError> {
    // Run embedded module tests with an empty project registry.
    //
    // Parameters:
    // - `source` — full `.sd` source text
    //
    // Returns:
    // Test pass/fail summary from the interpreter.
    //
    // Options:
    // None.
    //
    // Example:
    // let summary = run_tests(source)?;

    driver_run_tests_with_registry(source, &ModuleRegistry::new())
}
