use crate::record::Hash;
use sha2::{Digest, Sha256};

/// Compute SHA-256 hash of UTF-8 data, returned as hex string.
pub fn sha256(data: &str) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    Hash(hex::encode(hasher.finalize()))
}

/// Sign data with a device key (MVP: HMAC-style keyed hash over payload + key).
pub fn sign(data: &str, key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

/// Verify a signature produced by [`sign`].
pub fn verify_signature(data: &str, signature: &str, key: &str) -> bool {
    sign(data, key) == signature
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_is_deterministic() {
        let h1 = sha256("hello");
        let h2 = sha256("hello");
        assert_eq!(h1.0, h2.0);
        assert_eq!(h1.0.len(), 64);
    }

    #[test]
    fn sign_and_verify_roundtrip() {
        let sig = sign("payload", "device-key-001");
        assert!(verify_signature("payload", &sig, "device-key-001"));
        assert!(!verify_signature("payload", &sig, "wrong-key"));
    }
}
