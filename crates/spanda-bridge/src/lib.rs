//! Python and C++ subprocess bridges for `extern` function calls.
//!
pub mod cpp;
#[cfg(feature = "cpp-native")]
pub mod cpp_native;
pub mod protocol;
pub mod python;
#[cfg(feature = "python-native")]
pub mod python_native;

use spanda_ffi::{ExternBridges, FfiRegistry};

/// Build an FFI registry wired to the default Python and C++ subprocess bridges.
pub fn default_ffi_registry() -> FfiRegistry {
    // Description:
    //     Default ffi registry.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: FfiRegistry
    //         Return value from `default_ffi_registry`.
    //
    // Example:

    //     let result = spanda_bridge::default_ffi_registry();

    FfiRegistry::with_bridges(ExternBridges {
        python: Some(python::call_extern),
        cpp: Some(cpp::call_extern),
    })
}

/// Compatibility alias for `spanda_core::ffi::new_with_core_bridges`.
pub fn new_with_core_bridges() -> FfiRegistry {
    // Description:
    //     New with core bridges.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: FfiRegistry
    //         Return value from `new_with_core_bridges`.
    //
    // Example:

    //     let result = spanda_bridge::new_with_core_bridges();

    default_ffi_registry()
}
