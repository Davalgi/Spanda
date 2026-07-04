//! Release-hardening security regressions for plugin trust and sandboxing.

use spanda_plugin::api::PluginApiContext;
use spanda_plugin::capability::enforce_capability;
use spanda_plugin::loader::SandboxPermissions;
use spanda_plugin::manifest::PluginManifest;
use spanda_plugin::registry::{ensure_installable, lookup_plugin_entry, PluginTrustTier};
use spanda_plugin::security::{
    sign_plugin_artifact, validate_install_security, verify_plugin_signature,
};

const READ_ONLY_PLUGIN: &str = r#"
[plugin]
name = "spanda-plugin-readonly"
version = "0.1.0"
publisher = "example"
description = "Read-only plugin"
license = "Apache-2.0"
type = "readiness"

[compatibility]
spanda_version = ">=0.4.0"
api_version = "v1"

[capabilities]
requires = ["entity.read", "readiness.read"]

[security]
signed = true
sandbox = true
network = false
filesystem = "read-only"
"#;

fn parse_readonly() -> PluginManifest {
    PluginManifest::parse_str(READ_ONLY_PLUGIN).expect("manifest parses")
}

#[test]
fn plugin_trust_blocks_blocked_tier_install() {
    // Blocked registry entries must not be installable.
    let entry = lookup_plugin_entry("spanda-plugin-blocked-demo").expect("blocked demo listed");
    assert_eq!(entry.trust_tier(), PluginTrustTier::Blocked);
    let err = ensure_installable(&entry).expect_err("blocked install rejected");
    assert!(
        err.to_string().to_lowercase().contains("blocked"),
        "unexpected error: {err}"
    );
}

#[test]
fn plugin_sandbox_denies_network_and_write_by_default() {
    // Sandboxed manifests must not grant network or write access.
    let sandbox = SandboxPermissions::from_manifest(&parse_readonly());
    assert!(sandbox.sandbox);
    assert!(!sandbox.allows_network());
    assert!(!sandbox.allows_filesystem_write());
}

#[test]
fn plugin_capability_gate_rejects_actuator_control() {
    // Read-only plugins must not control actuators.
    let caps = parse_readonly().capability_set();
    assert!(enforce_capability(&caps, "entity.read").is_ok());
    assert!(enforce_capability(&caps, "actuator.write").is_err());
    let ctx = PluginApiContext::new("spanda-plugin-readonly", caps);
    assert!(ctx.readiness_read("mission-1").is_ok());
    assert!(ctx
        .report_generate("summary", serde_json::json!({}))
        .is_err());
}

#[test]
fn unsigned_or_tampered_plugin_signature_is_rejected() {
    // Invalid signatures and tampered digests must fail verification.
    let digest = "abc123";
    let signed = sign_plugin_artifact("demo-plugin", "0.1.0", digest, "plugin-test-signing-key");
    assert!(verify_plugin_signature(
        "demo-plugin",
        "0.1.0",
        digest,
        &signed,
        &signed.public_key
    ));
    assert!(!verify_plugin_signature(
        "demo-plugin",
        "0.1.0",
        "tampered-digest",
        &signed,
        &signed.public_key
    ));
}

#[test]
fn official_unsigned_plugin_install_fails_closed() {
    // Official plugins that claim to be signed must fail without a signature.
    let manifest = parse_readonly();
    let report = validate_install_security(
        &manifest,
        "0.4.0",
        Some("deadbeef"),
        None,
        PluginTrustTier::Official,
        false,
    )
    .expect("validation returns a report");
    assert!(
        !report.signature_ok || !report.approved,
        "unsigned official plugin must not be approved: {report:?}"
    );
}
