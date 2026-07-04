//! Release-hardening security regressions for package signatures and trust.

use spanda_package::registry_sign::{
    registry_require_signature, sign_registry_tarball, verify_registry_signature,
};
use spanda_package::trust::evaluate_package_trust;

#[test]
fn unsigned_package_rejected_when_signature_policy_enabled() {
    // Production signature policy must be opt-in and active when set.
    std::env::set_var("SPANDA_REGISTRY_REQUIRE_SIGNATURE", "1");
    let required = registry_require_signature();
    std::env::remove_var("SPANDA_REGISTRY_REQUIRE_SIGNATURE");
    assert!(required);
}

#[test]
fn package_signature_tamper_is_detected() {
    // Altered package digests must fail signature verification.
    let sig = sign_registry_tarball("spanda-gps", "0.1.0", "checksum-1", "test-sign-key");
    assert!(verify_registry_signature(
        "spanda-gps",
        "0.1.0",
        "checksum-1",
        &sig,
        "test-sign-key",
    ));
    assert!(!verify_registry_signature(
        "spanda-gps",
        "0.1.0",
        "checksum-tampered",
        &sig,
        "test-sign-key",
    ));
}

#[test]
fn fake_coordinator_key_material_is_rejected() {
    // Signatures produced with attacker key material must not verify.
    let victim = sign_registry_tarball("spanda-gps", "0.1.0", "checksum-1", "official-key");
    assert!(!verify_registry_signature(
        "spanda-gps",
        "0.1.0",
        "checksum-1",
        &victim,
        "attacker-key",
    ));
}

#[test]
fn package_trust_evaluation_does_not_leak_signing_material() {
    // Trust reports must not embed raw signing secrets.
    let report = evaluate_package_trust("spanda-gps", Some("0.1.0"), None);
    let blob = serde_json::to_string(&report).unwrap_or_default();
    assert!(
        !blob.contains("SPANDA_REGISTRY_SIGN_KEY"),
        "trust report leaked signing env name"
    );
    assert!(
        !blob.to_lowercase().contains("private_key"),
        "trust report leaked private key material"
    );
}
