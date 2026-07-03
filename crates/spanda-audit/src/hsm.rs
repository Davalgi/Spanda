//! External HSM signing backends (script and tpm2) for production key custody.

use crate::crypto::sign as software_sign;
use std::io::Write;
use std::process::{Command, Stdio};

/// Run an external signing command with payload on stdin; stdout must be hex Ed25519 signature.
pub fn sign_via_external_command(
    command: &str,
    key_id: &str,
    payload: &str,
) -> Result<String, String> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .env("SPANDA_HSM_KEY_ID", key_id)
        .env("SPANDA_DECISION_SIGNING_KEY_ID", key_id)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to spawn HSM sign command: {e}"))?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(payload.as_bytes())
            .map_err(|e| format!("failed to write payload to HSM sign command: {e}"))?;
    }
    let output = child
        .wait_with_output()
        .map_err(|e| format!("HSM sign command failed: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "HSM sign command exited {}: {}",
            output.status, stderr
        ));
    }
    let sig = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if sig.is_empty() {
        return Err("HSM sign command returned empty signature".into());
    }
    Ok(sig)
}

/// Resolve tpm2/script signing command from environment.
pub fn external_sign_command_from_env() -> Option<String> {
    std::env::var("SPANDA_HSM_SIGN_SCRIPT")
        .ok()
        .filter(|v| !v.is_empty())
        .or_else(|| {
            std::env::var("SPANDA_TPM2_SIGN_SCRIPT")
                .ok()
                .filter(|v| !v.is_empty())
        })
}

/// Sign via external HSM script when configured; otherwise fall back to software signing.
pub fn sign_with_external_hsm(data: &str, key_id: &str, fallback_material: &str) -> String {
    if let Some(command) = external_sign_command_from_env() {
        match sign_via_external_command(&command, key_id, data) {
            Ok(sig) => return sig,
            Err(error) => {
                if std::env::var("SPANDA_HSM_SIGN_REQUIRED")
                    .map(|v| matches!(v.as_str(), "1" | "true" | "yes" | "on"))
                    .unwrap_or(false)
                {
                    eprintln!("HSM sign required but failed: {error}");
                    return String::new();
                }
                eprintln!("HSM sign failed, falling back to software: {error}");
            }
        }
    }
    software_sign(data, fallback_material)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_command_signs_payload() {
        let cmd = "cat >/dev/null; echo aa".to_string();
        let sig = sign_via_external_command(&cmd, "fleet-key-1", "payload").expect("sign");
        assert_eq!(sig, "aa");
    }
}
