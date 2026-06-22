//! Planned FFI bridge import paths for compile-time import resolution.
//!

const FFI_BRIDGE_IMPORTS: &[&str] = &[
    "python.torch",
    "python.opencv",
    "python.numpy",
    "python.ros2",
    "cpp.ros2",
    "cpp.pcl",
    "cpp.opencv",
    "cpp.cuda",
];

pub fn resolve_ffi_import(path: &str) -> bool {
    // Resolve whether a dotted import path refers to a known FFI bridge namespace.
    //
    // Parameters:
    // - `path` — import path (e.g. `python.torch`)
    //
    // Returns:
    // true when the path matches a known or well-formed FFI bridge prefix.
    //
    // Options:
    // None.
    //
    // Example:
    // assert!(resolve_ffi_import("python.torch"));

    if FFI_BRIDGE_IMPORTS.contains(&path) {
        return true;
    }

    if let Some(suffix) = path.strip_prefix("python.") {
        return !suffix.is_empty()
            && suffix
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
    }

    if let Some(suffix) = path.strip_prefix("cpp.") {
        return !suffix.is_empty()
            && suffix
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_ffi_imports_resolve() {
        assert!(resolve_ffi_import("python.torch"));
        assert!(resolve_ffi_import("cpp.ros2"));
    }

    #[test]
    fn unknown_imports_do_not_resolve() {
        assert!(!resolve_ffi_import("java.awt"));
    }
}
