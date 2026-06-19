pub const DEPRECATED_SYN_EXTENSION_WARNING: &str =
    ".syn files are deprecated. Use .sd files instead.";

/// Emit a deprecation warning when the path uses the legacy `.syn` extension.
pub fn warn_deprecated_source_extension(path: &str) {
    if path.ends_with(".syn") {
        eprintln!("warning: {DEPRECATED_SYN_EXTENSION_WARNING}");
    }
}
