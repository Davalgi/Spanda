//! HMAC-SHA256 hashing for Control Center API keys with constant-time verification.
//!
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Return the pepper used to hash API key tokens.
///
/// Set `SPANDA_API_KEY_PEPPER` in production. When unset, a dev-only default is used.
pub fn api_key_pepper() -> String {
    std::env::var("SPANDA_API_KEY_PEPPER").unwrap_or_else(|_| {
        std::env::var("SPANDA_CONTROL_CENTER_STATE_DIR")
            .unwrap_or_else(|_| ".spanda".into())
    })
}

/// Compute the HMAC-SHA256 hex digest for an API key token.
pub fn hash_api_key_token(token: &str, pepper: &str) -> String {
    // Derive a stable digest for persisting API key material.
    //
    // Parameters:
    // - `token` — raw bearer token presented by clients
    // - `pepper` — server-side secret from `api_key_pepper()`
    //
    // Returns:
    // Lowercase hex-encoded HMAC-SHA256 digest.
    //
    // Options:
    // None.
    //
    // Example:
    // let digest = hash_api_key_token("abc", &api_key_pepper());

    let mut mac = HmacSha256::new_from_slice(pepper.as_bytes())
        .expect("HMAC accepts arbitrary key length");
    mac.update(token.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Verify a presented token against an expected digest using constant-time comparison.
pub fn verify_api_key_token(token: &str, pepper: &str, expected_hash: &str) -> bool {
    // Compare digests without short-circuiting on the first differing byte.
    //
    // Parameters:
    // - `token` — bearer token from the client
    // - `pepper` — server-side pepper
    // - `expected_hash` — stored hex digest
    //
    // Returns:
    // `true` when the token matches the stored digest.
    //
    // Options:
    // None.
    //
    // Example:
    // let ok = verify_api_key_token(token, &pepper, &record.token_hash);

    let computed = hash_api_key_token(token, pepper);
    constant_time_eq(computed.as_bytes(), expected_hash.as_bytes())
}

/// Constant-time equality for two byte slices of equal length.
pub fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right.iter())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b))
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic() {
        let digest = hash_api_key_token("test-key", "pepper");
        assert_eq!(digest, hash_api_key_token("test-key", "pepper"));
        assert_ne!(digest, hash_api_key_token("other-key", "pepper"));
    }

    #[test]
    fn verify_rejects_wrong_token() {
        let digest = hash_api_key_token("good", "pepper");
        assert!(verify_api_key_token("good", "pepper", &digest));
        assert!(!verify_api_key_token("bad", "pepper", &digest));
    }
}
