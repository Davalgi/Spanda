//! Portable path helpers for runtime artifacts.
//!
use std::path::{Component, Path, PathBuf};

/// Return `target` relative to `base` when it lives under `base`, otherwise a `..` path.
pub fn relativize_path(base: &Path, target: &Path) -> PathBuf {
    // Description:
    //     Express `target` relative to `base` for portable artifact metadata.
    //
    // Parameters:
    // - `base` — anchor directory, typically the artifact parent directory
    // - `target` — path to relativize
    //
    // Returns:
    // Relative path when possible, otherwise the original `target`.
    //
    // Options:
    // None.
    //
    // Example:
    // let rel = relativize_path(trace_dir, source_path);

    if !target.is_absolute() {
        return target.to_path_buf();
    }

    let base = base.canonicalize().unwrap_or_else(|_| base.to_path_buf());
    let abs_target = target
        .canonicalize()
        .unwrap_or_else(|_| target.to_path_buf());

    if let Ok(stripped) = abs_target.strip_prefix(&base) {
        if stripped.as_os_str().is_empty() {
            return PathBuf::from(".");
        }
        return stripped.to_path_buf();
    }

    path_diff(&base, &abs_target).unwrap_or_else(|| target.to_path_buf())
}

/// Resolve a trace source label against the trace directory and current working directory.
pub fn resolve_trace_source_path(trace_parent: &Path, source: &str) -> PathBuf {
    // Description:
    //     Resolve a stored trace `source` label to a concrete filesystem path.
    //
    // Parameters:
    // - `trace_parent` — directory containing the `.trace` file
    // - `source` — stored source label from the trace metadata
    //
    // Returns:
    // Best-effort resolved source path.
    //
    // Options:
    // None.
    //
    // Example:
    // let source = resolve_trace_source_path(trace_dir, "mission.sd");

    let path = Path::new(source);

    if path.is_absolute() {
        return path.to_path_buf();
    }

    if path.is_file() {
        return path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    }

    let beside_trace = trace_parent.join(path);
    if beside_trace.is_file() {
        return beside_trace.canonicalize().unwrap_or(beside_trace);
    }

    path.to_path_buf()
}

/// Normalize a trace `source` label to a path relative to the `.trace` parent directory.
pub fn normalize_trace_source(trace_path: &Path, source: &str) -> String {
    // Description:
    //     Rewrite trace metadata so `source` is portable across machines.
    //
    // Parameters:
    // - `trace_path` — destination `.trace` file path
    // - `source` — runtime source label before persistence
    //
    // Returns:
    // Relative source label suitable for committing or sharing traces.
    //
    // Options:
    // None.
    //
    // Example:
    // let label = normalize_trace_source(Path::new("mission.trace"), "/tmp/mission.sd");

    let trace_parent = trace_path.parent().unwrap_or_else(|| Path::new("."));
    let resolved = resolve_trace_source_path(trace_parent, source);
    relativize_path(trace_parent, &resolved)
        .to_string_lossy()
        .into_owned()
}

fn path_diff(from: &Path, to: &Path) -> Option<PathBuf> {
    // Description:
    //     Build a relative path from `from` to `to` using `..` segments.
    //
    // Parameters:
    // - `from` — starting directory
    // - `to` — destination path
    //
    // Returns:
    // Relative path when components can be compared, otherwise `None`.
    //
    // Options:
    // None.
    //
    // Example:
    // let rel = path_diff(Path::new("a/b"), Path::new("a/c"))?;

    let from_parts = normalize_components(from);
    let to_parts = normalize_components(to);

    if from_parts.is_empty() && to_parts.is_empty() {
        return Some(PathBuf::from("."));
    }

    let mut shared = 0usize;
    while shared < from_parts.len()
        && shared < to_parts.len()
        && from_parts[shared] == to_parts[shared]
    {
        shared += 1;
    }

    let mut result = PathBuf::new();
    for _ in shared..from_parts.len() {
        result.push("..");
    }
    for part in &to_parts[shared..] {
        result.push(part);
    }

    if result.as_os_str().is_empty() {
        result.push(".");
    }

    Some(result)
}

fn normalize_components(path: &Path) -> Vec<PathBuf> {
    // Description:
    //     Expand a path into comparable component segments.
    //
    // Parameters:
    // - `path` — filesystem path to normalize
    //
    // Returns:
    // Component list with `.` and `..` removed where possible.
    //
    // Options:
    // None.
    //
    // Example:
    // let parts = normalize_components(Path::new("a/b/../c"));

    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                parts.pop();
            }
            Component::Normal(value) => parts.push(PathBuf::from(value)),
            Component::RootDir | Component::Prefix(_) => {
                parts.push(PathBuf::from(component.as_os_str()))
            }
        }
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn relativize_same_directory_filename() {
        let dir = tempdir().unwrap();
        let source = dir.path().join("mission.sd");
        fs::write(&source, "robot Mission {}").unwrap();
        let rel = relativize_path(dir.path(), &source);
        assert_eq!(rel, PathBuf::from("mission.sd"));
    }

    #[test]
    fn normalize_trace_source_strips_absolute_paths() {
        let dir = tempdir().unwrap();
        let source = dir.path().join("warehouse.sd");
        fs::write(&source, "robot Mission {}").unwrap();
        let trace = dir.path().join("warehouse.trace");
        let normalized = normalize_trace_source(&trace, &source.to_string_lossy());
        assert_eq!(normalized, "warehouse.sd");
    }

    #[test]
    fn normalize_trace_source_collapses_repo_relative_labels() {
        let root = tempdir().unwrap();
        let nested = root.path().join("examples/demo");
        fs::create_dir_all(&nested).unwrap();
        let source = nested.join("main.sd");
        fs::write(&source, "robot Mission {}").unwrap();
        let trace = nested.join("main.trace");
        let original = root
            .path()
            .join("examples/demo/main.sd")
            .to_string_lossy()
            .into_owned();
        std::env::set_current_dir(root.path()).unwrap();
        let normalized = normalize_trace_source(&trace, &original);
        assert_eq!(normalized, "main.sd");
    }

    #[test]
    fn lockfile_style_relative_path_diff() {
        let root = tempdir().unwrap();
        let nested = root.path().join("examples/anchor");
        fs::create_dir_all(&nested).unwrap();
        let target = root.path().join("packages/lib");
        fs::create_dir_all(&target).unwrap();
        let rel = path_diff(&nested, &target).expect("relative path");
        assert_eq!(rel, PathBuf::from("../../packages/lib"));
    }
}
