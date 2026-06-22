//! Type-check host wiring for language reference metadata.
//!
use spanda_runtime_host::core_type_check_host;
use spanda_typecheck::{self, MethodSig};

#[allow(non_snake_case)]
pub fn BUILTIN_METHODS(
) -> std::collections::HashMap<String, std::collections::HashMap<String, MethodSig>> {
    spanda_typecheck::BUILTIN_METHODS(core_type_check_host())
}
