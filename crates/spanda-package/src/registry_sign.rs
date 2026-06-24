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
    // Description:
    //     Registry signature payload.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //     version: &str
    //         Caller-supplied version.
    //     sha256: &str
    //         Caller-supplied sha256.
    //
    // Outputs:
    //     result: String
    //         Return value from `registry_signature_payload`.
    //
    // Example:

    //     let result = spanda_package::registry_sign::registry_signature_payload(name, version, sha256);

    format!("spanda-registry-v1\n{name}@{version}\n{sha256}\n")
}

/// Sign a registry tarball digest and return public key + signature hex.
pub fn sign_registry_tarball(
    name: &str,
    version: &str,
    sha256: &str,
    sign_key_material: &str,
) -> RegistryVersionSignature {
    // Description:
    //     Sign registry tarball.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //     version: &str
    //         Caller-supplied version.
    //     sha256: &str
    //         Caller-supplied sha256.
    //     sign_key_material: &str
    //         Caller-supplied sign key material.
    //
    // Outputs:
    //     result: RegistryVersionSignature
    //         Return value from `sign_registry_tarball`.
    //
    // Example:

    //     let result = spanda_package::registry_sign::sign_registry_tarball(name, version, sha256, sign_key_material);

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
    // Description:
    //     Verify registry signature.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //     version: &str
    //         Caller-supplied version.
    //     sha256: &str
    //         Caller-supplied sha256.
    //     signature: &RegistryVersionSignature
    //         Caller-supplied signature.
    //     rust_key_material: &str
    //         Caller-supplied rust key material.
    //
    // Outputs:
    //     result: bool
    //         Return value from `verify_registry_signature`.
    //
    // Example:

    //     let result = spanda_package::registry_sign::verify_registry_signature(name, version, sha256, signature, rust_key_material);

    let payload = registry_signature_payload(name, version, sha256);
    verify_signature(&payload, &signature.signature, trust_key_material)
}

/// Return true when `SPANDA_REGISTRY_REQUIRE_SIGNATURE=1`.
pub fn registry_require_signature() -> bool {
    // Description:
    //     Registry require signature.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `registry_require_signature`.
    //
    // Example:

    //     let result = spanda_package::registry_sign::registry_require_signature();

    matches!(
        std::env::var("SPANDA_REGISTRY_REQUIRE_SIGNATURE").as_deref(),
        Ok("1") | Ok("true") | Ok("yes")
    )
}

/// Resolve trusted signing material from `SPANDA_REGISTRY_TRUST_KEY`.
pub fn registry_trust_key() -> Option<String> {
    // Description:
    //     Registry trust key.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `registry_trust_key`.
    //
    // Example:

    //     let result = spanda_package::registry_sign::registry_trust_key();

    std::env::var("SPANDA_REGISTRY_TRUST_KEY")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Resolve signing material from `SPANDA_REGISTRY_SIGN_KEY`.
pub fn registry_sign_key() -> Option<String> {
    // Description:
    //     Registry sign key.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `registry_sign_key`.
    //
    // Example:

    //     let result = spanda_package::registry_sign::registry_sign_key();

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
        // Description:
        //     Sign and verify round trip.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_package::registry_sign::sign_and_verify_round_trip();

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
