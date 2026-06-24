//! SHA-256 helpers for program artifacts and deployment proofs.

use std::fs;
use std::path::Path;

use sha2::{Digest, Sha256};

/// Compute a SHA-256 hex digest of a program artifact on disk.
pub fn hash_program_artifact(program_path: &str) -> Option<String> {
    // Description:
    //     Hash program artifact.
    //
    // Inputs:
    //     program_path: &str
    //         Caller-supplied program path.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `hash_program_artifact`.
    //
    // Example:

    //     let result = spanda_certify::artifact::hash_program_artifact(program_path);

    let path = Path::new(program_path);
    if !path.exists() {
        return None;
    }
    let bytes = fs::read(path).ok()?;
    Some(hex::encode(Sha256::digest(bytes)))
}
