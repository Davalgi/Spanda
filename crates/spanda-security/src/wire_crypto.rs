//! AEAD wire encryption for Spanda transport frames (AES-256-GCM).

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use sha2::{Digest, Sha256};

/// Production session cipher for encrypted transport wire payloads.
#[derive(Debug, Clone)]
pub struct WireCryptoSession {
    key: [u8; 32],
    pub cipher_suite: String,
}

impl WireCryptoSession {
    /// Derive a 256-bit session key from configured cert/key material.
    pub fn from_material(material: &str) -> Self {
        // Description:
        //     From material.
        //
        // Inputs:
        //     aterial: &str
        //         Caller-supplied aterial.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_material`.
        //
        // Example:

        //     let result = spanda_security::wire_crypto::from_material(aterial);

        let digest = Sha256::digest(material.as_bytes());
        let mut key = [0u8; 32];
        key.copy_from_slice(&digest);
        Self {
            key,
            cipher_suite: "AES-256-GCM".into(),
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        // Description:
        //     Encrypt.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     plaintex: &[u8]
        //         Caller-supplied plaintex.
        //
        // Outputs:
        //     result: Result<Vec<u8>, String>
        //         Return value from `encrypt`.
        //
        // Example:

        //     let result = spanda_security::wire_crypto::encrypt(&self, plaintex);

        let cipher =
            Aes256Gcm::new_from_slice(&self.key).map_err(|e| format!("cipher init failed: {e}"))?;
        let mut nonce_bytes = [0u8; 12];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| format!("encrypt failed: {e}"))?;
        let mut out = Vec::with_capacity(12 + ciphertext.len());
        out.extend_from_slice(&nonce_bytes);
        out.extend_from_slice(&ciphertext);
        Ok(out)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Description:
        //     Decrypt.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     data: &[u8]
        //         Caller-supplied data.
        //
        // Outputs:
        //     result: Result<Vec<u8>, String>
        //         Return value from `decrypt`.
        //
        // Example:

        //     let result = spanda_security::wire_crypto::decrypt(&self, data);

        if data.len() < 13 {
            return Err("ciphertext too short".into());
        }
        let cipher =
            Aes256Gcm::new_from_slice(&self.key).map_err(|e| format!("cipher init failed: {e}"))?;
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("decrypt failed: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aead_roundtrip() {
        // Description:
        //     Aead roundtrip.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_security::wire_crypto::aead_roundtrip();

        let session = WireCryptoSession::from_material("rover-cert:rover-key");
        let plain = br#"{"v":1,"topic":"/cmd"}"#;
        let enc = session.encrypt(plain).unwrap();
        let dec = session.decrypt(&enc).unwrap();
        assert_eq!(dec, plain);
    }
}
