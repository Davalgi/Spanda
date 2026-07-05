//! Control Center product version constants (embedded UI semver).

/// Semver for the embedded Control Center UI (`packages/web`), set at build time.
pub const CONTROL_CENTER_UI_VERSION: &str = env!("CONTROL_CENTER_UI_VERSION");
