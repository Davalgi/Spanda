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
    // Description:

    //     Type check.

    //

    // Inputs:

    //     progra: &Program

    //         Caller-supplied progra.

    //

    // Outputs:

    //     result: Result<(), SpandaError>

    //         Return value from `type_check`.

    //

    // Example:

    //     let result = spanda_driver::type_check::type_check(progra);

    spanda_typecheck::type_check(program, core_type_check_host()).map_err(type_check_error)
}

pub fn check(program: &Program) -> Result<(), SpandaError> {
    // Description:

    //     Check.

    //

    // Inputs:

    //     progra: &Program

    //         Caller-supplied progra.

    //

    // Outputs:

    //     result: Result<(), SpandaError>

    //         Return value from `check`.

    //

    // Example:

    //     let result = spanda_driver::type_check::check(progra);

    spanda_typecheck::check(program, core_type_check_host()).map_err(type_check_error)
}

pub fn check_with_registry(
    program: &Program,
    registry: &ModuleRegistry,
) -> Result<(), SpandaError> {
    // Description:

    //     Check with registry.

    //

    // Inputs:

    //     progra: &Program

    //         Caller-supplied progra.

    //     registry: &ModuleRegistry

    //         Caller-supplied registry.

    //

    // Outputs:

    //     result: Result<(), SpandaError>

    //         Return value from `check_with_registry`.

    //

    // Example:

    //     let result = spanda_driver::type_check::check_with_registry(progra, registry);

    spanda_typecheck::check_with_registry(program, registry, core_type_check_host())
        .map_err(type_check_error)
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

    //     let result = spanda_driver::type_check::type_check_error(err);

    SpandaError::TypeCheck {
        diagnostics: err.diagnostics,
    }
}

#[allow(non_snake_case)]
pub fn BUILTIN_METHODS(
) -> std::collections::HashMap<String, std::collections::HashMap<String, spanda_typecheck::MethodSig>>
{
    // Description:
    //     BUILTIN METHODS.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: std::collections::HashMap<String, std::collections::HashMap<String, spanda_typecheck::MethodSig>>
    //         Return value from `BUILTIN_METHODS`.
    //
    // Example:
    //     let result = spanda_driver::type_check::BUILTIN_METHODS();

    spanda_typecheck::BUILTIN_METHODS(core_type_check_host())
}

#[allow(non_snake_case)]
pub fn SENSOR_TYPES() -> std::collections::HashMap<String, spanda_ast::nodes::SpandaType> {
    // Description:
    //     SENSOR TYPES.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: std::collections::HashMap<String, spanda_ast::nodes::SpandaType>
    //         Return value from `SENSOR_TYPES`.
    //
    // Example:
    //     let result = spanda_driver::type_check::SENSOR_TYPES();

    spanda_typecheck::SENSOR_TYPES(core_type_check_host())
}

pub fn get_library_for_sensor_type(sensor_type: &str) -> Option<String> {
    // Description:

    //     Get library for sensor type.

    //

    // Inputs:

    //     sensor_type: &str

    //         Caller-supplied sensor type.

    //

    // Outputs:

    //     result: Option<String>

    //         Return value from `get_library_for_sensor_type`.

    //

    // Example:

    //     let result = spanda_driver::type_check::get_library_for_sensor_type(sensor_type);

    spanda_typecheck::get_library_for_sensor_type(sensor_type, core_type_check_host())
}

pub fn merge_library_methods(
    methods: &mut std::collections::HashMap<
        String,
        std::collections::HashMap<String, spanda_typecheck::MethodSig>,
    >,
) {
    // Description:

    //     Merge library methods.

    //

    // Inputs:

    //     ethods: &mut std::collections::HashMap< String, std::collections::HashMap<String, spanda_typecheck::MethodSig>, >

    //         Caller-supplied ethods.

    //

    // Outputs:

    //     None.

    //

    // Example:

    //     let result = spanda_driver::type_check::merge_library_methods(ethods);

    spanda_typecheck::merge_library_methods(methods, core_type_check_host());
}
