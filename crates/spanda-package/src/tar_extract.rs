//! Safe `.tar.gz` extraction with path-traversal guards.
//!

use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::path::{Component, Path, PathBuf};
use tar::Archive;

/// Extract a gzip-compressed tarball into `dest`, rejecting path traversal entries.
pub fn extract_tarball_safe(tarball: &Path, dest: &Path) -> Result<(), String> {
    // Extract a registry or publish bundle without tar-slip paths.
    //
    // Parameters:
    // - `tarball` — `.tar.gz` file on disk
    // - `dest` — output directory (created if missing)
    //
    // Returns:
    // Ok on success, or an error when the archive is invalid or unsafe.
    //
    // Options:
    // None.
    //
    // Example:
    // extract_tarball_safe(bundle_path, vendor_dir)?;

    fs::create_dir_all(dest).map_err(|e| format!("create extract dir: {e}"))?;
    let base = fs::canonicalize(dest).map_err(|e| format!("canonicalize dest: {e}"))?;
    let file = File::open(tarball).map_err(|e| format!("open tarball: {e}"))?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    for entry in archive.entries().map_err(|e| format!("read tar entries: {e}"))? {
        let mut entry = entry.map_err(|e| format!("read tar entry: {e}"))?;
        let entry_path = entry.path().map_err(|e| format!("tar entry path: {e}"))?;
        let target = safe_join(&base, &entry_path)?;

        if entry.header().entry_type().is_dir() {
            fs::create_dir_all(&target).map_err(|e| format!("create dir {}: {e}", target.display()))?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("create parent {}: {e}", parent.display()))?;
            }
            entry
                .unpack(&target)
                .map_err(|e| format!("unpack {}: {e}", target.display()))?;
        }
    }
    Ok(())
}

fn safe_join(base: &Path, entry: &Path) -> Result<PathBuf, String> {
    // Resolve a tarball member path under `base`, rejecting escapes.
    //
    // Parameters:
    // - `base` — canonical extraction root
    // - `entry` — path from the tar header
    //
    // Returns:
    // Absolute destination path, or an error for unsafe entries.
    //
    // Options:
    // None.
    //
    // Example:
    // let out = safe_join(&base, Path::new("src/main.sd"))?;

    let mut out = base.to_path_buf();
    for component in entry.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                return Err(format!(
                    "tar entry has absolute path: {}",
                    entry.display()
                ));
            }
            Component::ParentDir => {
                return Err(format!(
                    "tar entry traverses parent directories: {}",
                    entry.display()
                ));
            }
            Component::Normal(part) => out.push(part),
            Component::CurDir => {}
        }
    }
    if !out.starts_with(base) {
        return Err(format!(
            "tar entry escapes destination: {}",
            entry.display()
        ));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use tar::{Builder, Header};

    fn write_tarball(path: &Path, members: &[(&str, &[u8])]) {
        let file = File::create(path).expect("create tarball");
        let encoder = GzEncoder::new(file, Compression::default());
        let mut builder = Builder::new(encoder);
        for (name, data) in members {
            let mut header = Header::new_gnu();
            header.set_path(name).expect("set path");
            header.set_size(data.len() as u64);
            header.set_cksum();
            builder
                .append(&header, &data[..])
                .expect("append member");
        }
        builder.into_inner().expect("finish tar").finish().expect("finish gzip");
    }

    #[test]
    fn extracts_normal_members() {
        let root = std::env::temp_dir().join(format!("spanda-tar-safe-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("tmpdir");
        let tarball = root.join("bundle.tar.gz");
        let dest = root.join("out");
        write_tarball(&tarball, &[("src/hello.sd", b"robot R {}")]);
        extract_tarball_safe(&tarball, &dest).expect("extract");
        assert!(dest.join("src/hello.sd").is_file());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn rejects_parent_traversal_paths() {
        let root = std::env::temp_dir().join(format!("spanda-tar-slip-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("tmpdir");
        let base = fs::canonicalize(&root).expect("canonical");
        let err = safe_join(&base, Path::new("../escape.txt")).expect_err("should reject");
        assert!(err.contains("parent"), "got: {err}");
        let _ = fs::remove_dir_all(&root);
    }
}
