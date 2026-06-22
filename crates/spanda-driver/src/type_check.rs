//! Type-check entry points wired to the core runtime host.
//!
use spanda_ast::nodes::Program;
use spanda_error::SpandaError;
use spanda_runtime_host::core_type_check_host;
use spanda_typecheck::{self, ModuleRegistry, TypeCheckError};

pub use spanda_typecheck::{
    format_type_name, units_compatible, Diagnostic, MethodSig, TypeChecker, ACTION_TYPES,
    ACTUATOR_TYPES, AI_MODEL_TYPES, AI_VALUE_TYPES, BUILTIN_FUNCTIONS, MESSAGE_TYPES,
    OBJECT_PROPERTIES, ROBOT_METHODS, SCAN_PROPERTIES, SERVICE_TYPES,
};

pub fn type_check(program: &Program) -> Result<(), SpandaError> {
    // Type-check a parsed program with the default runtime host.
    //
    // Parameters:
    // - `program` — parsed Spanda AST
    //
    // Returns:
    // Unit on success, or a type diagnostic error.
    //
    // Options:
    // None.
    //
    // Example:
    // type_check(&program)?;

    spanda_typecheck::type_check(program, core_type_check_host()).map_err(type_check_error)
}

pub fn check(program: &Program) -> Result<(), SpandaError> {
    // Validate a parsed program without retaining checker state.
    //
    // Parameters:
    // - `program` — parsed Spanda AST
    //
    // Returns:
    // Unit on success, or a type diagnostic error.
    //
    // Options:
    // None.
    //
    // Example:
    // check(&program)?;

    spanda_typecheck::check(program, core_type_check_host()).map_err(type_check_error)
}

pub fn check_with_registry(
    program: &Program,
    registry: &ModuleRegistry,
) -> Result<(), SpandaError> {
    // Type-check with a project module registry for import resolution.
    //
    // Parameters:
    // - `program` — parsed Spanda AST
    // - `registry` — loaded project modules
    //
    // Returns:
    // Unit on success, or a type diagnostic error.
    //
    // Options:
    // None.
    //
    // Example:
    // check_with_registry(&program, &registry)?;

    spanda_typecheck::check_with_registry(program, registry, core_type_check_host())
        .map_err(type_check_error)
}

fn type_check_error(err: TypeCheckError) -> SpandaError {
    SpandaError::TypeCheck {
        diagnostics: err.diagnostics,
    }
}

#[allow(non_snake_case)]
pub fn BUILTIN_METHODS(
) -> std::collections::HashMap<String, std::collections::HashMap<String, spanda_typecheck::MethodSig>>
{
    spanda_typecheck::BUILTIN_METHODS(core_type_check_host())
}

#[allow(non_snake_case)]
pub fn SENSOR_TYPES() -> std::collections::HashMap<String, spanda_ast::nodes::SpandaType> {
    spanda_typecheck::SENSOR_TYPES(core_type_check_host())
}

pub fn get_library_for_sensor_type(sensor_type: &str) -> Option<String> {
    spanda_typecheck::get_library_for_sensor_type(sensor_type, core_type_check_host())
}

pub fn merge_library_methods(
    methods: &mut std::collections::HashMap<
        String,
        std::collections::HashMap<String, spanda_typecheck::MethodSig>,
    >,
) {
    spanda_typecheck::merge_library_methods(methods, core_type_check_host());
}
