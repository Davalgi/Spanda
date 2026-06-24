//! Tarball integrity helpers (SHA-256 checksums).
//!

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Compute lowercase hex SHA-256 of a file.
pub fn sha256_file(path: &Path) -> Result<String, String> {
    // Description:
    //     Sha256 file.
    //
    // Inputs:
    //     path: &Path
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: Result<String, String>
    //         Return value from `sha256_file`.
    //
    // Example:

    //     let result = spanda_package::integrity::sha256_file(path);

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
    // Description:
    //     Checksum sidecar path.
    //
    // Inputs:
    //     arball: &Path
    //         Caller-supplied arball.
    //
    // Outputs:
    //     result: std::path::PathBuf
    //         Return value from `checksum_sidecar_path`.
    //
    // Example:

    //     let result = spanda_package::integrity::checksum_sidecar_path(arball);

    let name = tarball
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "bundle.tar.gz".into());
    tarball.with_file_name(format!("{name}.sha256"))
}

/// Write `{tarball}.sha256` and return the digest.
pub fn write_checksum_sidecar(tarball: &Path) -> Result<String, String> {
    // Description:
    //     Write checksum sidecar.
    //
    // Inputs:
    //     arball: &Path
    //         Caller-supplied arball.
    //
    // Outputs:
    //     result: Result<String, String>
    //         Return value from `write_checksum_sidecar`.
    //
    // Example:

    //     let result = spanda_package::integrity::write_checksum_sidecar(arball);

    let digest = sha256_file(tarball)?;
    let sidecar = checksum_sidecar_path(tarball);
    let mut file =
        File::create(&sidecar).map_err(|e| format!("create {}: {e}", sidecar.display()))?;
    writeln!(file, "{digest}").map_err(|e| format!("write {}: {e}", sidecar.display()))?;
    Ok(digest)
}

/// Read expected digest from a `.sha256` sidecar when present.
pub fn read_checksum_sidecar(tarball: &Path) -> Option<String> {
    // Description:
    //     Read checksum sidecar.
    //
    // Inputs:
    //     arball: &Path
    //         Caller-supplied arball.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `read_checksum_sidecar`.
    //
    // Example:

    //     let result = spanda_package::integrity::read_checksum_sidecar(arball);

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
    // Description:
    //     Verify sha256.
    //
    // Inputs:
    //     path: &Path
    //         Caller-supplied path.
    //     expected: &str
    //         Caller-supplied expected.
    //
    // Outputs:
    //     result: Result<(), String>
    //         Return value from `verify_sha256`.
    //
    // Example:

    //     let result = spanda_package::integrity::verify_sha256(path, expected);

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
    // Description:
    //     Registry require checksum.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `registry_require_checksum`.
    //
    // Example:

    //     let result = spanda_package::integrity::registry_require_checksum();

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
        // Description:
        //     Sidecar round trip.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_package::integrity::sidecar_round_trip();

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
