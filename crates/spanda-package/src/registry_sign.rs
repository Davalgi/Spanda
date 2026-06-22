//! Ed25519 signatures for published registry tarballs.
//!

use serde::{Deserialize, Serialize};
use spanda_audit::{public_key_from_material, sign, verify_signature};
use std::collections::BTreeMap;

/// Signature metadata for one published registry version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryVersionSignature {
    pub public_key: String,
    pub signature: String,
}

/// Build the canonical string signed for a registry tarball.
pub fn registry_signature_payload(name: &str, version: &str, sha256: &str) -> String {
    // Canonicalize name, version, and digest for Ed25519 signing.
    //
    // Parameters:
    // - `name` — package name
    // - `version` — semver string
    // - `sha256` — lowercase hex digest of the tarball
    //
    // Returns:
    // UTF-8 payload passed to `sign` / `verify_signature`.
    //
    // Options:
    // None.
    //
    // Example:
    // let payload = registry_signature_payload("spanda-mqtt", "0.1.0", &digest);

    format!("spanda-registry-v1\n{name}@{version}\n{sha256}\n")
}

/// Sign a registry tarball digest and return public key + signature hex.
pub fn sign_registry_tarball(
    name: &str,
    version: &str,
    sha256: &str,
    sign_key_material: &str,
) -> RegistryVersionSignature {
    // Sign publish metadata for a registry bundle.
    //
    // Parameters:
    // - `name` — package name
    // - `version` — semver string
    // - `sha256` — tarball digest
    // - `sign_key_material` — Ed25519 seed or signing passphrase
    //
    // Returns:
    // Public key and signature hex strings for the registry index.
    //
    // Options:
    // None.
    //
    // Example:
    // let sig = sign_registry_tarball("demo", "0.1.0", &digest, key)?;

    let payload = registry_signature_payload(name, version, sha256);
    RegistryVersionSignature {
        public_key: public_key_from_material(sign_key_material),
        signature: sign(&payload, sign_key_material),
    }
}

/// Verify a registry signature against trusted key material.
pub fn verify_registry_signature(
    name: &str,
    version: &str,
    sha256: &str,
    signature: &RegistryVersionSignature,
    trust_key_material: &str,
) -> bool {
    // Validate a registry index signature for one package version.
    //
    // Parameters:
    // - `name` — package name
    // - `version` — semver string
    // - `sha256` — expected tarball digest
    // - `signature` — index metadata
    // - `trust_key_material` — trusted public key hex or signing material
    //
    // Returns:
    // true when the signature matches the canonical payload.
    //
    // Options:
    // None.
    //
    // Example:
    // verify_registry_signature(name, version, &digest, &sig, trust_key);

    let payload = registry_signature_payload(name, version, sha256);
    verify_signature(&payload, &signature.signature, trust_key_material)
}

/// Return true when `SPANDA_REGISTRY_REQUIRE_SIGNATURE=1`.
pub fn registry_require_signature() -> bool {
    // Whether remote installs must carry a valid registry signature.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // true when strict signature mode is enabled.
    //
    // Options:
    // Reads `SPANDA_REGISTRY_REQUIRE_SIGNATURE`.
    //
    // Example:
    // if registry_require_signature() && sig.is_none() { ... }

    matches!(
        std::env::var("SPANDA_REGISTRY_REQUIRE_SIGNATURE").as_deref(),
        Ok("1") | Ok("true") | Ok("yes")
    )
}

/// Resolve trusted signing material from `SPANDA_REGISTRY_TRUST_KEY`.
pub fn registry_trust_key() -> Option<String> {
    // Load the trusted registry public key from the environment.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Key material when `SPANDA_REGISTRY_TRUST_KEY` is set.
    //
    // Options:
    // None.
    //
    // Example:
    // if let Some(key) = registry_trust_key() { verify... }

    std::env::var("SPANDA_REGISTRY_TRUST_KEY")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Resolve signing material from `SPANDA_REGISTRY_SIGN_KEY`.
pub fn registry_sign_key() -> Option<String> {
    // Load the registry signing key from the environment.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Signing material when `SPANDA_REGISTRY_SIGN_KEY` is set.
    //
    // Options:
    // None.
    //
    // Example:
    // if let Some(key) = registry_sign_key() { sign... }

    std::env::var("SPANDA_REGISTRY_SIGN_KEY")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub type RegistryVersionSignatures = BTreeMap<String, RegistryVersionSignature>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_and_verify_round_trip() {
        let digest = "abc123";
        let key = "registry-test-signing-key";
        let signed = sign_registry_tarball("demo-pkg", "0.1.0", digest, key);
        assert!(verify_registry_signature(
            "demo-pkg",
            "0.1.0",
            digest,
            &signed,
            &signed.public_key
        ));
        assert!(!verify_registry_signature(
            "demo-pkg",
            "0.1.0",
            "wrong",
            &signed,
            &signed.public_key
        ));
    }
}
