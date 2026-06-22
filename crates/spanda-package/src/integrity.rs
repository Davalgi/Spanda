//! Tarball integrity helpers (SHA-256 checksums).
//!

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Compute lowercase hex SHA-256 of a file.
pub fn sha256_file(path: &Path) -> Result<String, String> {
    // Hash a tarball or sidecar file on disk.
    //
    // Parameters:
    // - `path` — file to hash
    //
    // Returns:
    // Lowercase hex digest, or an I/O error string.
    //
    // Options:
    // None.
    //
    // Example:
    // let digest = sha256_file(bundle_path)?;

    let mut file = File::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let read = file
            .read(&mut buf)
            .map_err(|e| format!("read {}: {e}", path.display()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buf[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Sidecar path for a tarball checksum (`bundle.tar.gz.sha256`).
pub fn checksum_sidecar_path(tarball: &Path) -> std::path::PathBuf {
    // Derive the checksum sidecar filename for a bundle.
    //
    // Parameters:
    // - `tarball` — `.tar.gz` bundle path
    //
    // Returns:
    // Adjacent `.sha256` path.
    //
    // Options:
    // None.
    //
    // Example:
    // let sidecar = checksum_sidecar_path(&bundle);

    let name = tarball
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "bundle.tar.gz".into());
    tarball.with_file_name(format!("{name}.sha256"))
}

/// Write `{tarball}.sha256` and return the digest.
pub fn write_checksum_sidecar(tarball: &Path) -> Result<String, String> {
    // Publish-time helper: hash bundle and write sidecar file.
    //
    // Parameters:
    // - `tarball` — `.tar.gz` bundle path
    //
    // Returns:
    // Hex digest written to the sidecar.
    //
    // Options:
    // None.
    //
    // Example:
    // let digest = write_checksum_sidecar(&report.bundle_path)?;

    let digest = sha256_file(tarball)?;
    let sidecar = checksum_sidecar_path(tarball);
    let mut file = File::create(&sidecar)
        .map_err(|e| format!("create {}: {e}", sidecar.display()))?;
    writeln!(file, "{digest}").map_err(|e| format!("write {}: {e}", sidecar.display()))?;
    Ok(digest)
}

/// Read expected digest from a `.sha256` sidecar when present.
pub fn read_checksum_sidecar(tarball: &Path) -> Option<String> {
    // Load a checksum sidecar adjacent to a local tarball.
    //
    // Parameters:
    // - `tarball` — `.tar.gz` bundle path
    //
    // Returns:
    // Hex digest when the sidecar exists and is non-empty.
    //
    // Options:
    // None.
    //
    // Example:
    // let expected = read_checksum_sidecar(&bundle_path);

    let sidecar = checksum_sidecar_path(tarball);
    let text = std::fs::read_to_string(&sidecar).ok()?;
    let digest = text.split_whitespace().next()?.trim().to_string();
    if digest.is_empty() {
        None
    } else {
        Some(digest)
    }
}

/// Compare a file digest to an expected hex string.
pub fn verify_sha256(path: &Path, expected: &str) -> Result<(), String> {
    // Verify tarball bytes against a registry or sidecar digest.
    //
    // Parameters:
    // - `path` — file on disk
    // - `expected` — lowercase hex SHA-256
    //
    // Returns:
    // Ok when digests match, otherwise an error string.
    //
    // Options:
    // None.
    //
    // Example:
    // verify_sha256(&tarball, &entry.version_checksums["0.1.0"])?;

    let actual = sha256_file(path)?;
    let expected = expected.trim().to_ascii_lowercase();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "checksum mismatch for {}: expected {expected}, got {actual}",
            path.display()
        ))
    }
}

/// Return true when `SPANDA_REGISTRY_REQUIRE_CHECKSUM=1`.
pub fn registry_require_checksum() -> bool {
    // Whether remote installs must have a known checksum.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // true when strict checksum mode is enabled.
    //
    // Options:
    // Reads `SPANDA_REGISTRY_REQUIRE_CHECKSUM`.
    //
    // Example:
    // if registry_require_checksum() && expected.is_none() { ... }

    matches!(
        std::env::var("SPANDA_REGISTRY_REQUIRE_CHECKSUM").as_deref(),
        Ok("1") | Ok("true") | Ok("yes")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn sidecar_round_trip() {
        let root = std::env::temp_dir().join(format!("spanda-sha-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("tmpdir");
        let bundle = root.join("demo-0.1.0.tar.gz");
        let mut file = File::create(&bundle).expect("bundle");
        file.write_all(b"payload").expect("write");
        let digest = write_checksum_sidecar(&bundle).expect("sidecar");
        assert_eq!(read_checksum_sidecar(&bundle), Some(digest.clone()));
        verify_sha256(&bundle, &digest).expect("verify");
        let _ = std::fs::remove_dir_all(&root);
    }
}
