//! Parse and typecheck validation for generated Spanda source.

use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_runtime_host::core_type_check_host;
use spanda_typecheck::{self, ModuleRegistry};

/// Validate generated source through parse and typecheck gates.
pub fn validate_generated_source(source: &str) -> Result<(), String> {
    // Run lexer, parser, and typechecker on generated Spanda source.
    //
    // Parameters:
    // - `source` — generated program text
    //
    // Returns:
    // Ok when source is syntactically and semantically valid.
    //
    // Options:
    // None.
    //
    // Example:
    // validate_generated_source(&generated)?;

    let tokens = tokenize(source).map_err(|error| error.to_string())?;
    let program = parse(tokens).map_err(|error| error.to_string())?;
    let registry = ModuleRegistry::new();
    spanda_typecheck::check_with_registry(&program, &registry, core_type_check_host())
        .map_err(|error| format!("{error:?}"))
}
