//! Bundled offline registry slice shipped inside the spanda CLI crate.

use std::env;
use std::path::{Path, PathBuf};

/// Return the bundled registry directory when `index.json` is present.
pub fn bundled_registry_dir() -> Option<PathBuf> {
    // Locate the offline registry slice shipped with the spanda CLI crate.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Path to `bundled-registry` when present.
    //
    // Options:
    // None.
    //
    // Example:
    // let dir = bundled_registry::bundled_registry_dir();

    let bundled = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bundled-registry");
    if bundled.join("index.json").is_file() {
        Some(bundled)
    } else {
        None
    }
}

/// Return a `file://` URL for the bundled registry when available.
pub fn bundled_registry_url() -> Option<String> {
    bundled_registry_dir().map(|dir| format!("file://{}", dir.display()))
}

/// Return a `file://` URL for the monorepo `registry/` index when present.
pub fn monorepo_registry_url() -> Option<String> {
    // Prefer the full in-repo registry over the incomplete CLI bundle slice.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // `file://…/registry` when `registry/index.json` is found.
    //
    // Options:
    // None.
    //
    // Example:
    // let url = bundled_registry::monorepo_registry_url();

    let mut candidates: Vec<PathBuf> = Vec::new();
    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("registry"),
    );
    if let Ok(cwd) = env::current_dir() {
        let mut dir = cwd;
        for _ in 0..8 {
            candidates.push(dir.join("registry"));
            if !dir.pop() {
                break;
            }
        }
    }
    candidates.into_iter().find_map(|dir| {
        if dir.join("index.json").is_file() {
            Some(format!("file://{}", dir.display()))
        } else {
            None
        }
    })
}

/// Set `SPANDA_REGISTRY_URL` to the monorepo or bundled registry when unset.
pub fn ensure_bundled_registry_env() {
    // Default registry resolution for offline demos and monorepo installs.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // None (may set process environment).
    //
    // Options:
    // Respects a non-empty existing `SPANDA_REGISTRY_URL`. Prefers the full
    // monorepo `registry/` index when present, otherwise the CLI bundle slice.
    //
    // Example:
    // bundled_registry::ensure_bundled_registry_env();

    if env::var("SPANDA_REGISTRY_URL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .is_some()
    {
        return;
    }
    // Prefer the full monorepo registry so flagship demos resolve all packages.
    if let Some(url) = monorepo_registry_url() {
        env::set_var("SPANDA_REGISTRY_URL", url);
        return;
    }
    // Fall back to the offline CLI bundle for cargo-install layouts.
    if let Some(url) = bundled_registry_url() {
        env::set_var("SPANDA_REGISTRY_URL", url);
    }
}

/// Return true when `path` lives under the bundled registry directory.
#[allow(dead_code)]
pub fn is_bundled_registry_path(path: &Path) -> bool {
    bundled_registry_dir()
        .map(|dir| path.starts_with(dir))
        .unwrap_or(false)
}
