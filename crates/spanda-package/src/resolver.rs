//! resolver support for Spanda.
//!
use crate::dependency::{
    parse_version, DependencySourceKind, DependencySpec, LockedDependency, LockedSource,
};
use crate::error::{PackageError, PackageResult};
use crate::manifest::{PackageManifest, MANIFEST_FILENAME};
use crate::registry::registry_package_dir;
use crate::registry_remote::{lookup_registry_entry, RegistryEntryLookup};
use semver::Version;
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::path::Path;

/// Options for dependency resolution.
#[derive(Debug, Clone, Default)]
pub struct ResolveOptions {
    pub offline: bool,
}

/// Result of resolving all dependencies for a project.
#[derive(Debug, Clone)]
pub struct ResolveResult {
    pub lockfile_deps: BTreeMap<String, LockedDependency>,
    pub warnings: Vec<String>,
}

pub fn resolve_dependencies(
    project_root: &Path,
    manifest: &PackageManifest,
    options: &ResolveOptions,
) -> PackageResult<ResolveResult> {
    // Description:
    //     Resolve dependencies.
    //
    // Inputs:
    //     project_roo: &Path
    //         Caller-supplied project roo.
    //     anifes: &PackageManifest
    //         Caller-supplied anifes.
    //     options: &ResolveOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: PackageResult<ResolveResult>
    //         Return value from `resolve_dependencies`.
    //
    // Example:
    //     let result = spanda_package::resolver::resolve_dependencies(project_roo, anifes, options);

    // Create mutable lockfile deps for accumulating results.
    let mut lockfile_deps = BTreeMap::new();
    let mut warnings = Vec::new();
    let mut pending: VecDeque<(String, DependencySpec, std::path::PathBuf)> = VecDeque::new();
    let mut queued: HashSet<String> = HashSet::new();

    // Seed the queue with direct manifest dependencies.
    for (name, spec) in manifest.all_dependencies() {
        if queued.insert(name.to_string()) {
            pending.push_back((name.to_string(), spec.clone(), project_root.to_path_buf()));
        }
    }

    // Resolve direct and transitive dependencies in breadth-first order.
    while let Some((name, spec, resolve_root)) = pending.pop_front() {
        if lockfile_deps.contains_key(&name) {
            continue;
        }
        let locked = resolve_one(&resolve_root, &name, &spec, options)?;
        if let Some(dep_manifest) = manifest_for_locked_dep(project_root, &locked) {
            let parent_root = package_root_for_locked(project_root, &locked);
            for (trans_name, trans_spec) in dep_manifest.all_dependencies() {
                if !lockfile_deps.contains_key(trans_name) && queued.insert(trans_name.to_string())
                {
                    pending.push_back((
                        trans_name.to_string(),
                        trans_spec.clone(),
                        parent_root.clone(),
                    ));
                }
            }
        }
        lockfile_deps.insert(name, locked);
    }

    // Take this path when options.offline.
    if options.offline {
        warnings.push("resolved in offline mode — registry packages use cached versions".into());
    }
    Ok(ResolveResult {
        lockfile_deps,
        warnings,
    })
}

fn resolve_one(
    project_root: &Path,
    name: &str,
    spec: &DependencySpec,
    _options: &ResolveOptions,
) -> PackageResult<LockedDependency> {
    // Description:
    //     Resolve one.
    //
    // Inputs:
    //     project_roo: &Path
    //         Caller-supplied project roo.
    //     name: &str
    //         Caller-supplied name.
    //     spec: &DependencySpec
    //         Caller-supplied spec.
    //     _options: &ResolveOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: PackageResult<LockedDependency>
    //         Return value from `resolve_one`.
    //
    // Example:
    //     let result = spanda_package::resolver::resolve_one(project_roo, name, spec, _options);

    // Match on source kind and handle each case.
    match spec.source_kind() {
        DependencySourceKind::Local => {
            let path = spec.local_path(project_root).ok_or_else(|| {
                PackageError::Dependency(format!("local path missing for {name}"))
            })?;
            let dep_manifest = load_dep_manifest(&path)?;
            let stored_path = lockfile_local_path(project_root, spec, &path);
            Ok(LockedDependency {
                name: name.to_string(),
                version: dep_manifest.package.version.clone(),
                source: LockedSource::Local { path: stored_path },
                checksum: None,
            })
        }
        DependencySourceKind::Git => {
            let detail = match spec {
                DependencySpec::Detail(d) => d,
                _ => {
                    return Err(PackageError::Dependency(format!(
                        "git dependency '{name}' requires inline table"
                    )));
                }
            };
            let url = detail
                .git
                .clone()
                .ok_or_else(|| PackageError::Dependency(format!("git URL missing for {name}")))?;
            let version = detail
                .tag
                .clone()
                .or_else(|| detail.rev.clone())
                .unwrap_or_else(|| "0.0.0-git".into());
            Ok(LockedDependency {
                name: name.to_string(),
                version,
                source: LockedSource::Git {
                    url,
                    branch: detail.branch.clone(),
                    tag: detail.tag.clone(),
                    rev: detail.rev.clone(),
                },
                checksum: None,
            })
        }
        DependencySourceKind::Registry => {
            let lookup = lookup_registry_entry(name).ok_or_else(|| {
                PackageError::Dependency(format!(
                    "registry package '{name}' not found — add to local registry, set SPANDA_REGISTRY_URL, or use path/git"
                ))
            })?;
            let version_req = spec.parse_version_req()?.ok_or_else(|| {
                PackageError::Dependency(format!("version constraint required for {name}"))
            })?;
            let resolved = select_registry_version(&lookup, &version_req)?;
            Ok(LockedDependency {
                name: name.to_string(),
                version: resolved.to_string(),
                source: LockedSource::Registry {
                    registry: lookup.registry_label().into(),
                },
                checksum: None,
            })
        }
    }
}

fn lockfile_local_path(
    project_root: &Path,
    spec: &DependencySpec,
    resolved: &Path,
) -> std::path::PathBuf {
    // Description:
    //     Persist local dependency paths relative to the project root when possible.
    //
    // Parameters:
    // - `project_root` — directory containing `spanda.toml`
    // - `spec` — declared dependency specification from the manifest
    // - `resolved` — absolute or project-relative resolved dependency path
    //
    // Returns:
    // Portable lockfile path suitable for committing `spanda.lock`.
    //
    // Options:
    // None.
    //
    // Example:
    // let stored = lockfile_local_path(root, &spec, &resolved);

    if let DependencySpec::Detail(detail) = spec {
        if let Some(declared) = &detail.path {
            if !declared.is_absolute() {
                return declared.clone();
            }
        }
    }

    spanda_runtime::path_util::relativize_path(project_root, resolved)
}

fn package_root_for_locked(project_root: &Path, locked: &LockedDependency) -> std::path::PathBuf {
    // Description:
    //     Package root for locked.
    //
    // Inputs:
    //     project_roo: &Path
    //         Caller-supplied project roo.
    //     locked: &LockedDependency
    //         Caller-supplied locked.
    //
    // Outputs:
    //     result: std::path::PathBuf
    //         Return value from `package_root_for_locked`.
    //
    // Example:

    //     let result = spanda_package::resolver::package_root_for_locked(project_roo, locked);

    match &locked.source {
        LockedSource::Local { path } => {
            if path.is_absolute() {
                path.clone()
            } else {
                project_root.join(path)
            }
        }
        LockedSource::Registry { .. } => registry_package_dir(&locked.name)
            .unwrap_or_else(|| project_root.join(".spanda/packages").join(&locked.name)),
        LockedSource::Git { .. } => project_root.join(".spanda/packages").join(&locked.name),
    }
}

fn manifest_for_locked_dep(
    project_root: &Path,
    locked: &LockedDependency,
) -> Option<PackageManifest> {
    // Description:
    //     Manifest for locked dep.
    //
    // Inputs:
    //     project_roo: &Path
    //         Caller-supplied project roo.
    //     locked: &LockedDependency
    //         Caller-supplied locked.
    //
    // Outputs:
    //     result: Option<PackageManifest>
    //         Return value from `manifest_for_locked_dep`.
    //
    // Example:

    //     let result = spanda_package::resolver::manifest_for_locked_dep(project_roo, locked);

    match &locked.source {
        LockedSource::Local { path } => {
            let abs = if path.is_absolute() {
                path.clone()
            } else {
                project_root.join(path)
            };
            load_dep_manifest(&abs).ok()
        }
        LockedSource::Registry { .. } => registry_package_dir(&locked.name)
            .and_then(|dir| PackageManifest::load_from_dir(&dir).ok()),
        LockedSource::Git { .. } => None,
    }
}

fn load_dep_manifest(path: &Path) -> PackageResult<PackageManifest> {
    // Description:
    //     Load dep manifest.
    //
    // Inputs:
    //     path: &Path
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: PackageResult<PackageManifest>
    //         Return value from `load_dep_manifest`.
    //
    // Example:
    //     let result = spanda_package::resolver::load_dep_manifest(path);

    // Compute manifest path for the following logic.
    let manifest_path = path.join(MANIFEST_FILENAME);

    // Continue only when the path is a regular file.
    if !manifest_path.is_file() {
        return Err(PackageError::Dependency(format!(
            "dependency at {} has no spanda.toml",
            path.display()
        )));
    }
    PackageManifest::load(&manifest_path)
}

fn select_registry_version(
    entry: &RegistryEntryLookup,
    req: &semver::VersionReq,
) -> PackageResult<Version> {
    // Description:
    //     Select registry version.
    //
    // Inputs:
    //     entry: &RegistryEntryLookup
    //         Caller-supplied entry.
    //     req: &semver::VersionReq
    //         Caller-supplied req.
    //
    // Outputs:
    //     result: PackageResult<Version>
    //         Return value from `select_registry_version`.
    //
    // Example:
    //     let result = spanda_package::resolver::select_registry_version(entry, req);

    // Create mutable candidates for accumulating results.
    let mut candidates: Vec<Version> = entry
        .versions()
        .iter()
        .filter_map(|v| parse_version(v).ok())
        .filter(|v| req.matches(v))
        .collect();
    candidates.sort();
    candidates.pop().ok_or_else(|| {
        PackageError::Dependency(format!(
            "no version of '{}' satisfies constraint {}",
            entry.name(),
            req
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dependency::DependencyDetail;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn write_manifest(dir: &Path, name: &str, version: &str) {
        // Description:
        //     Write manifest.
        //
        // Inputs:
        //     dir: &Path
        //         Caller-supplied dir.
        //     name: &str
        //         Caller-supplied name.
        //     version: &str
        //         Caller-supplied version.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_package::resolver::write_manifest(dir, name, version);

        // Compute content for the following logic.
        let content = format!(
            r#"
[package]
name = "{name}"
version = "{version}"
"#
        );
        fs::write(dir.join(MANIFEST_FILENAME), content).unwrap();
    }

    #[test]
    fn resolves_local_dependency() {
        // Description:
        //     Resolves local dependency.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_package::resolver::resolves_local_dependency();

        let root = TempDir::new().unwrap();
        let lib = root.path().join("lib");
        fs::create_dir(&lib).unwrap();
        write_manifest(&lib, "my_lib", "0.2.0");
        write_manifest(root.path(), "app", "0.1.0");

        let mut manifest = PackageManifest::load_from_dir(root.path()).unwrap();
        manifest.dependencies.insert(
            "my_lib".into(),
            DependencySpec::Detail(DependencyDetail {
                version: None,
                path: Some(PathBuf::from("lib")),
                git: None,
                branch: None,
                tag: None,
                rev: None,
            }),
        );

        let result =
            resolve_dependencies(root.path(), &manifest, &ResolveOptions::default()).unwrap();
        let locked = result.lockfile_deps.get("my_lib").unwrap();
        assert_eq!(locked.version, "0.2.0");
        match &locked.source {
            LockedSource::Local { path } => assert_eq!(path, &PathBuf::from("lib")),
            other => panic!("expected local source, got {other:?}"),
        }
    }

    #[test]
    fn resolves_registry_dependency() {
        // Description:
        //     Resolves registry dependency.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_package::resolver::resolves_registry_dependency();

        let root = TempDir::new().unwrap();
        write_manifest(root.path(), "app", "0.1.0");

        let mut manifest = PackageManifest::load_from_dir(root.path()).unwrap();
        manifest.dependencies.insert(
            "spanda-ros2".into(),
            DependencySpec::Version("0.1.0".into()),
        );

        let result =
            resolve_dependencies(root.path(), &manifest, &ResolveOptions::default()).unwrap();
        let locked = result.lockfile_deps.get("spanda-ros2").unwrap();
        assert_eq!(locked.version, "0.1.0");
    }

    #[test]
    fn resolves_transitive_local_dependencies() {
        // Description:
        //     Resolves transitive local dependencies.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_package::resolver::resolves_transitive_local_dependencies();

        let root = TempDir::new().unwrap();
        let shared = root.path().join("shared");
        let lib = root.path().join("lib");
        fs::create_dir_all(&shared).unwrap();
        fs::create_dir_all(&lib).unwrap();
        write_manifest(&shared, "shared_utils", "0.1.0");
        write_manifest(&lib, "my_lib", "0.2.0");
        let lib_manifest = format!(
            r#"
[package]
name = "my_lib"
version = "0.2.0"

[dependencies]
shared_utils = {{ path = "../shared" }}
"#
        );
        fs::write(lib.join(MANIFEST_FILENAME), lib_manifest).unwrap();
        write_manifest(root.path(), "app", "0.1.0");

        let mut manifest = PackageManifest::load_from_dir(root.path()).unwrap();
        manifest.dependencies.insert(
            "my_lib".into(),
            DependencySpec::Detail(DependencyDetail {
                version: None,
                path: Some(PathBuf::from("lib")),
                git: None,
                branch: None,
                tag: None,
                rev: None,
            }),
        );

        let result =
            resolve_dependencies(root.path(), &manifest, &ResolveOptions::default()).unwrap();
        assert!(result.lockfile_deps.contains_key("my_lib"));
        assert!(result.lockfile_deps.contains_key("shared_utils"));
    }

    #[test]
    fn parses_git_dependency() {
        // Description:
        //     Parses git dependency.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_package::resolver::parses_git_dependency();

        let root = TempDir::new().unwrap();
        write_manifest(root.path(), "app", "0.1.0");

        let mut manifest = PackageManifest::load_from_dir(root.path()).unwrap();
        manifest.dependencies.insert(
            "spanda-nav".into(),
            DependencySpec::Detail(DependencyDetail {
                version: None,
                path: None,
                git: Some("https://github.com/spanda/spanda-nav".into()),
                branch: Some("main".into()),
                tag: None,
                rev: None,
            }),
        );

        let result =
            resolve_dependencies(root.path(), &manifest, &ResolveOptions::default()).unwrap();
        let locked = result.lockfile_deps.get("spanda-nav").unwrap();
        assert!(matches!(locked.source, LockedSource::Git { .. }));
    }

    #[test]
    fn preserves_declared_parent_local_path_in_lockfile() {
        let root = TempDir::new().unwrap();
        let package = root.path().join("pkg");
        let audit = root.path().join("audit");
        fs::create_dir_all(&package).unwrap();
        fs::create_dir_all(&audit).unwrap();
        write_manifest(&package, "spanda-ledger", "0.1.0");
        write_manifest(&audit, "ledger-anchor-audit", "0.1.0");

        let mut manifest = PackageManifest::load_from_dir(&audit).unwrap();
        manifest.dependencies.insert(
            "spanda-ledger".into(),
            DependencySpec::Detail(DependencyDetail {
                version: None,
                path: Some(PathBuf::from("../pkg")),
                git: None,
                branch: None,
                tag: None,
                rev: None,
            }),
        );

        let result = resolve_dependencies(&audit, &manifest, &ResolveOptions::default()).unwrap();
        let locked = result.lockfile_deps.get("spanda-ledger").unwrap();
        match &locked.source {
            LockedSource::Local { path } => assert_eq!(path, &PathBuf::from("../pkg")),
            other => panic!("expected local source, got {other:?}"),
        }
    }
}
