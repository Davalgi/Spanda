//! Resolve installed official lean-core packages from project manifests.
//!
use crate::adapter::framework_packages;
use crate::error::{PackageError, PackageResult};
use crate::lockfile::{Lockfile, LOCKFILE_FILENAME};
use crate::manifest::{PackageManifest, MANIFEST_FILENAME};
use std::collections::HashSet;
use std::path::Path;

/// Return dependency names that match known official framework packages.
pub fn installed_official_packages<'a>(
    dependency_names: impl IntoIterator<Item = &'a str>,
) -> Vec<&'static str> {
    // Collect installed official package names from a dependency list.
    //
    // Parameters:
    // - `dependency_names` — keys from `spanda.toml` `[dependencies]`
    //
    // Returns:
    // Sorted list of official package names present in the manifest.
    //
    // Options:
    // None.
    //
    // Example:

    // let names = installed_official_packages(["spanda-ros2", "my-local-lib"]);

    let official: HashSet<&str> = framework_packages().iter().map(|p| p.name).collect();
    let mut found: Vec<&str> = dependency_names
        .into_iter()
        .filter_map(|name| official.get(name).copied())
        .collect();
    found.sort_unstable();
    found.dedup();
    found
}

/// Whether a package name is a registered official framework package.
pub fn is_official_package(name: &str) -> bool {
    // Description:
    //     Is official package.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: bool
    //         Return value from `is_official_package`.
    //
    // Example:

    //     let result = spanda_package::official::is_official_package(name);

    framework_packages().iter().any(|p| p.name == name)
}

/// Resolve official packages declared in a project manifest.
pub fn official_packages_from_manifest(manifest: &PackageManifest) -> Vec<String> {
    // Description:
    //     Official packages from manifest.
    //
    // Inputs:
    //     anifes: &PackageManifest
    //         Caller-supplied anifes.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `official_packages_from_manifest`.
    //
    // Example:

    //     let result = spanda_package::official::official_packages_from_manifest(anifes);

    installed_official_packages(manifest.dependencies.keys().map(String::as_str))
        .into_iter()
        .map(str::to_string)
        .collect()
}

/// Resolve official packages from a resolved lockfile.
pub fn official_packages_from_lockfile(lockfile: &Lockfile) -> Vec<String> {
    // Description:
    //     Official packages from lockfile.
    //
    // Inputs:
    //     lockfile: &Lockfile
    //         Caller-supplied lockfile.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `official_packages_from_lockfile`.
    //
    // Example:

    //     let result = spanda_package::official::official_packages_from_lockfile(lockfile);

    installed_official_packages(lockfile.dependencies.keys().map(String::as_str))
        .into_iter()
        .map(str::to_string)
        .collect()
}

/// Load official package names for a project directory (prefers lockfile over manifest).
pub fn load_official_packages_for_project(root: &Path) -> PackageResult<Vec<String>> {
    // Description:
    //     Load official packages for project.
    //
    // Inputs:
    //     roo: &Path
    //         Caller-supplied roo.
    //
    // Outputs:
    //     result: PackageResult<Vec<String>>
    //         Return value from `load_official_packages_for_project`.
    //
    // Example:

    //     let result = spanda_package::official::load_official_packages_for_project(roo);

    let lock_path = root.join(LOCKFILE_FILENAME);
    if lock_path.is_file() {
        let lockfile = Lockfile::load(&lock_path)?;
        return Ok(official_packages_from_lockfile(&lockfile));
    }
    let manifest_path = root.join(MANIFEST_FILENAME);
    if manifest_path.is_file() {
        let manifest = PackageManifest::load_from_dir(root)?;
        return Ok(official_packages_from_manifest(&manifest));
    }
    Err(PackageError::Manifest(format!(
        "no {MANIFEST_FILENAME} or {LOCKFILE_FILENAME} in {}",
        root.display()
    )))
}

/// Resolve official packages for a source file by walking up to the project root.
pub fn load_official_packages_for_source(source: &Path) -> Vec<String> {
    // Description:
    //     Load official packages for source.
    //
    // Inputs:
    //     source: &Path
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `load_official_packages_for_source`.
    //
    // Example:

    //     let result = spanda_package::official::load_official_packages_for_source(source);

    let start = if source.is_dir() {
        source.to_path_buf()
    } else {
        source
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    };
    let Some(root) = crate::manifest::find_project_root(&start) else {
        return Vec::new();
    };
    load_official_packages_for_project(&root).unwrap_or_default()
}
