//! Plugin signature verification, sandbox policy, and install gates.

use crate::capability::has_dangerous_capabilities;
use crate::compatibility::validate_spanda_version;
use crate::error::{PluginError, PluginResult};
use crate::manifest::PluginManifest;
use crate::registry::{
    ensure_installable, lookup_plugin_entry, verify_plugin_registry_signature, PluginTrustTier,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spanda_audit::{public_key_from_material, sign, verify_signature};
use spanda_package::registry_sign::RegistryVersionSignature;
use std::path::Path;

/// Security validation outcome for plugin install/load.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityValidationReport {
    pub signature_ok: bool,
    pub manifest_ok: bool,
    pub compatibility_ok: bool,
    pub trust_tier: String,
    pub sandbox_ok: bool,
    pub dangerous_capabilities: bool,
    pub approved: bool,
    pub detail: Vec<String>,
}

pub fn plugin_signature_payload(name: &str, version: &str, sha256: &str) -> String {
    format!("spanda-plugin-v1\n{name}@{version}\n{sha256}\n")
}

pub fn sign_plugin_artifact(
    name: &str,
    version: &str,
    sha256: &str,
    sign_key_material: &str,
) -> RegistryVersionSignature {
    let payload = plugin_signature_payload(name, version, sha256);
    RegistryVersionSignature {
        public_key: public_key_from_material(sign_key_material),
        signature: sign(&payload, sign_key_material),
    }
}

pub fn verify_plugin_signature(
    name: &str,
    version: &str,
    sha256: &str,
    signature: &RegistryVersionSignature,
    trust_key_material: &str,
) -> bool {
    let payload = plugin_signature_payload(name, version, sha256);
    verify_signature(&payload, &signature.signature, trust_key_material)
}

pub fn sha256_file(path: &Path) -> PluginResult<String> {
    let bytes = std::fs::read(path)?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

pub fn validate_install_security(
    manifest: &PluginManifest,
    host_version: &str,
    artifact_sha256: Option<&str>,
    signature: Option<&RegistryVersionSignature>,
    trust_tier: PluginTrustTier,
    dangerous_approved: bool,
) -> PluginResult<SecurityValidationReport> {
    let mut detail = Vec::new();
    let caps = manifest.capability_set();
    let dangerous = has_dangerous_capabilities(&caps);

    let compatibility = validate_spanda_version(manifest, host_version)?;
    let compatibility_ok = compatibility.compatible;
    if !compatibility_ok {
        detail.push(compatibility.detail.clone());
    }

    let manifest_ok = manifest.validate().is_ok();
    if !manifest_ok {
        detail.push("manifest validation failed".into());
    }

    let sandbox_ok = manifest.security.sandbox;
    if !sandbox_ok {
        detail.push(
            "sandbox disabled in manifest; only trusted local plugins may disable sandbox".into(),
        );
    }

    let signature_ok = match (artifact_sha256, signature, trust_tier) {
        (_, _, PluginTrustTier::Blocked) => {
            detail.push("plugin is blocked".into());
            false
        }
        (Some(digest), Some(sig), PluginTrustTier::Official) => {
            let ok = verify_plugin_registry_signature(
                &manifest.plugin.name,
                &manifest.plugin.version,
                digest,
                sig,
            ) || verify_plugin_signature(
                &manifest.plugin.name,
                &manifest.plugin.version,
                digest,
                sig,
                &sig.public_key,
            );
            if !ok {
                detail.push("official plugin signature verification failed".into());
            }
            if manifest.security.signed && !ok {
                detail.push("unsigned official plugins are not allowed".into());
            }
            ok
        }
        (Some(digest), Some(sig), _) if manifest.security.signed => verify_plugin_signature(
            &manifest.plugin.name,
            &manifest.plugin.version,
            digest,
            sig,
            &sig.public_key,
        ),
        (_, _, PluginTrustTier::Official) if manifest.security.signed => {
            detail.push("official signed plugin missing signature metadata".into());
            false
        }
        _ => true,
    };

    if dangerous && !dangerous_approved {
        detail.push("plugin requests dangerous capabilities without approval".into());
    }

    let approved = manifest_ok
        && compatibility_ok
        && signature_ok
        && trust_tier.install_allowed()
        && (!dangerous || dangerous_approved)
        && (sandbox_ok
            || matches!(
                trust_tier,
                PluginTrustTier::Experimental | PluginTrustTier::Community
            ));

    Ok(SecurityValidationReport {
        signature_ok,
        manifest_ok,
        compatibility_ok,
        trust_tier: trust_tier.as_str().to_string(),
        sandbox_ok,
        dangerous_capabilities: dangerous,
        approved,
        detail,
    })
}

pub fn registry_trust_tier(name: &str) -> PluginTrustTier {
    lookup_plugin_entry(name)
        .map(|e| e.trust_tier())
        .unwrap_or(PluginTrustTier::Community)
}

pub fn validate_registry_entry(name: &str) -> PluginResult<()> {
    if let Some(entry) = lookup_plugin_entry(name) {
        ensure_installable(&entry)
    } else {
        Ok(())
    }
}

pub fn dangerous_capabilities() -> &'static [&'static str] {
    crate::capability::dangerous_capabilities()
}

pub fn validate_filesystem_access(
    manifest: &PluginManifest,
    write_requested: bool,
) -> PluginResult<()> {
    let fs = manifest.security.filesystem.as_str();
    if write_requested {
        if fs == "read-only" {
            return Err(PluginError::Security(
                "filesystem write denied: manifest declares read-only".into(),
            ));
        }
        crate::capability::enforce_capability(&manifest.capability_set(), "filesystem.write")?;
    } else {
        crate::capability::enforce_capability(&manifest.capability_set(), "filesystem.read")?;
    }
    Ok(())
}
