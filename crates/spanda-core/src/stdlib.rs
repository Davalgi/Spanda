//! Standard library namespace registry for Spanda domain types.
//!
//! Types are resolved via [`crate::type_system::resolve_type_name`] and may be
//! referenced with or without the `std.<module>.` prefix.

pub use crate::type_system::std_namespaces;

pub fn resolve_std_import(path: &str) -> bool {
    // Check whether an import path refers to a registered std module.
    //
    // Parameters:
    //
    // - `path` — Import path (e.g. `"std.time"`, `"std.units"`).
    //
    // Returns:
    //
    // `true` when `path` is a known std namespace key.
    //
    // Example:
    //
    // use spanda_core::stdlib::resolve_std_import;
    // assert!(resolve_std_import("std.time"));
    // assert!(resolve_std_import("std.sensors"));
    // assert!(!resolve_std_import("vendor.custom"));
    std_namespaces().contains_key(path)
}
