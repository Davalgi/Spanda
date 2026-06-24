//! Compile pipeline entry points for Spanda source programs.
//!
use serde::{Deserialize, Serialize};
use spanda_ast::nodes::Program;
use spanda_error::SpandaError;
use spanda_lexer::Token;
use spanda_parser::parse;
use spanda_runtime_host::core_type_check_host;
use spanda_typecheck::{self, ModuleRegistry, TypeCheckError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    pub program: Program,
    pub source: String,
}

/// Tokenize Spanda source (maps lexer diagnostics to `SpandaError`).
pub fn tokenize(source: &str) -> Result<Vec<Token>, SpandaError> {
    // Description:
    //     Tokenize.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: Result<Vec<Token>, SpandaError>
    //         Return value from `tokenize`.
    //
    // Example:

    //     let result = spanda_driver::compile::tokenize(source);

    tokenize_source(source)
}

fn tokenize_source(source: &str) -> Result<Vec<Token>, SpandaError> {
    // Description:
    //     Tokenize source.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: Result<Vec<Token>, SpandaError>
    //         Return value from `tokenize_source`.
    //
    // Example:

    //     let result = spanda_driver::compile::tokenize_source(source);

    spanda_lexer::tokenize(source).map_err(SpandaError::from)
}

pub fn compile(source: &str) -> Result<CompileResult, SpandaError> {
    // Description:
    //     Compile.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: Result<CompileResult, SpandaError>
    //         Return value from `compile`.
    //
    // Example:

    //     let result = spanda_driver::compile::compile(source);

    let tokens = tokenize_source(source)?;
    let program = parse(tokens)?;
    spanda_typecheck::type_check(&program, core_type_check_host()).map_err(type_check_error)?;
    Ok(CompileResult {
        program,
        source: source.to_string(),
    })
}

pub fn check(source: &str) -> Result<(), SpandaError> {
    // Description:
    //     Check.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: Result<(), SpandaError>
    //         Return value from `check`.
    //
    // Example:

    //     let result = spanda_driver::compile::check(source);

    let tokens = tokenize_source(source)?;
    let program = parse(tokens)?;
    spanda_typecheck::check(&program, core_type_check_host()).map_err(type_check_error)
}

pub fn check_with_registry(source: &str, registry: &ModuleRegistry) -> Result<(), SpandaError> {
    // Description:
    //     Check with registry.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     registry: &ModuleRegistry
    //         Caller-supplied registry.
    //
    // Outputs:
    //     result: Result<(), SpandaError>
    //         Return value from `check_with_registry`.
    //
    // Example:

    //     let result = spanda_driver::compile::check_with_registry(source, registry);

    let tokens = tokenize_source(source)?;
    let program = parse(tokens)?;
    spanda_typecheck::check_with_registry(&program, registry, core_type_check_host())
        .map_err(type_check_error)
}

pub fn compile_with_registry(
    source: &str,
    registry: &ModuleRegistry,
) -> Result<CompileResult, SpandaError> {
    // Description:
    //     Compile with registry.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     registry: &ModuleRegistry
    //         Caller-supplied registry.
    //
    // Outputs:
    //     result: Result<CompileResult, SpandaError>
    //         Return value from `compile_with_registry`.
    //
    // Example:

    //     let result = spanda_driver::compile::compile_with_registry(source, registry);

    let tokens = tokenize_source(source)?;
    let program = parse(tokens)?;
    spanda_typecheck::check_with_registry(&program, registry, core_type_check_host())
        .map_err(type_check_error)?;
    Ok(CompileResult {
        program,
        source: source.to_string(),
    })
}

fn type_check_error(err: TypeCheckError) -> SpandaError {
    // Description:
    //     Type check error.
    //
    // Inputs:
    //     err: TypeCheckError
    //         Caller-supplied err.
    //
    // Outputs:
    //     result: SpandaError
    //         Return value from `type_check_error`.
    //
    // Example:

    //     let result = spanda_driver::compile::type_check_error(err);

    SpandaError::TypeCheck {
        diagnostics: err.diagnostics,
    }
}
