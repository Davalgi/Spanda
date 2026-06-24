//! Encrypted and verified message envelopes for secure Spanda communication.

use crate::error::{SecurityError, SecurityResult};
use crate::identity::RobotIdentity;
use crate::signed::SignedMessage;
use crate::wire_crypto::WireCryptoSession;
use serde::{Deserialize, Serialize};

/// Opaque session key handle (never logged).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionKey {
    pub id: String,
}

/// Public key material for identity verification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicKey {
    pub material: String,
}

/// Private key handle (value resolved via secret store, never logged).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateKey {
    pub secret_name: String,
}

/// X.509 or PEM certificate reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    pub path: String,
}

/// Trusted publisher identity for secure topic gating.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustedSource {
    pub id: String,
}

/// Encrypted payload envelope — plaintext is inaccessible until decryption.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptedMessage<T> {
    ciphertext: String,
    session_key_id: String,
    #[serde(skip)]
    decrypted: Option<T>,
}

impl<T: Clone + Serialize + for<'de> Deserialize<'de>> EncryptedMessage<T> {
    pub fn encrypt(payload: &T, session_material: &str) -> SecurityResult<Self> {
        // Description:
        //     Encrypt.
        //
        // Inputs:
        //     payload: &T
        //         Caller-supplied payload.
        //     session_material: &str
        //         Caller-supplied session material.
        //
        // Outputs:
        //     result: SecurityResult<Self>
        //         Return value from `encrypt`.
        //
        // Example:

        //     let result = spanda_security::encrypted::encrypt(payload, session_material);

        let plaintext = serde_json::to_string(payload)
            .map_err(|e| SecurityError::Other(format!("serialize failed: {e}")))?;
        let ciphertext = wire_encrypt_string(&plaintext, session_material);
        Ok(Self {
            ciphertext,
            session_key_id: session_material.to_string(),
            decrypted: None,
        })
    }

    pub fn decrypt(&mut self) -> SecurityResult<&T> {
        // Description:
        //     Decrypt.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     result: SecurityResult<&T>
        //         Return value from `decrypt`.
        //
        // Example:

        //     let result = spanda_security::encrypted::decrypt(&mut self);

        if self.decrypted.is_none() {
            let plaintext = wire_decrypt_string(&self.ciphertext, &self.session_key_id)?;
            let value: T = serde_json::from_str(&plaintext)
                .map_err(|e| SecurityError::Other(format!("deserialize failed: {e}")))?;
            self.decrypted = Some(value);
        }
        Ok(self.decrypted.as_ref().unwrap())
    }

    pub fn ciphertext(&self) -> &str {
        // Description:
        //     Ciphertext.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &str
        //         Return value from `ciphertext`.
        //
        // Example:

        //     let result = spanda_security::encrypted::ciphertext(&self);

        &self.ciphertext
    }
}

/// Signed payload envelope — must be verified before trusted use.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifiedMessage<T> {
    signed: SignedMessage,
    #[serde(skip)]
    verified: Option<T>,
}

impl<T: Clone + for<'de> Deserialize<'de>> VerifiedMessage<T> {
    pub fn from_signed(signed: SignedMessage) -> Self {
        // Description:
        //     From signed.
        //
        // Inputs:
        //     signed: SignedMessage
        //         Caller-supplied signed.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_signed`.
        //
        // Example:

        //     let result = spanda_security::encrypted::from_signed(signed);

        Self {
            signed,
            verified: None,
        }
    }

    pub fn verify_and_open(&mut self, identity: &RobotIdentity) -> SecurityResult<&T> {
        // Description:
        //     Verify and open.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     identity: &RobotIdentity
        //         Caller-supplied identity.
        //
        // Outputs:
        //     result: SecurityResult<&T>
        //         Return value from `verify_and_open`.
        //
        // Example:

        //     let result = spanda_security::encrypted::verify_and_open(&mut self, identity);

        if self.verified.is_none() {
            if !self.signed.verify(identity)? {
                return Err(SecurityError::SignatureInvalid);
            }
            let value: T = serde_json::from_str(&self.signed.payload)
                .map_err(|e| SecurityError::Other(format!("deserialize failed: {e}")))?;
            self.verified = Some(value);
        }
        Ok(self.verified.as_ref().unwrap())
    }
}

fn wire_encrypt_string(plaintext: &str, session_material: &str) -> String {
    // Description:
    //     Wire encrypt string.
    //
    // Inputs:
    //     plaintex: &str
    //         Caller-supplied plaintex.
    //     session_material: &str
    //         Caller-supplied session material.
    //
    // Outputs:
    //     result: String
    //         Return value from `wire_encrypt_string`.
    //
    // Example:

    //     let result = spanda_security::encrypted::wire_encrypt_string(plaintex, session_material);

    let session = WireCryptoSession::from_material(session_material);
    let encrypted = session
        .encrypt(plaintext.as_bytes())
        .expect("encrypt message payload");
    format!("enc:{session_material}:{}", hex::encode(encrypted))
}

fn wire_decrypt_string(ciphertext: &str, session_material: &str) -> SecurityResult<String> {
    // Description:
    //     Wire decrypt string.
    //
    // Inputs:
    //     ciphertex: &str
    //         Caller-supplied ciphertex.
    //     session_material: &str
    //         Caller-supplied session material.
    //
    // Outputs:
    //     result: SecurityResult<String>
    //         Return value from `wire_decrypt_string`.
    //
    // Example:

    //     let result = spanda_security::encrypted::wire_decrypt_string(ciphertex, session_material);

    let prefix = format!("enc:{session_material}:");
    let hex_payload = ciphertext
        .strip_prefix(&prefix)
        .ok_or(SecurityError::SecureEndpoint {
            endpoint: "decrypt".into(),
            reason: "invalid ciphertext prefix".into(),
        })?;
    let bytes = hex::decode(hex_payload).map_err(|e| SecurityError::Other(format!("hex: {e}")))?;
    let session = WireCryptoSession::from_material(session_material);
    let plain = session
        .decrypt(&bytes)
        .map_err(|e| SecurityError::SecureEndpoint {
            endpoint: "decrypt".into(),
            reason: e,
        })?;
    String::from_utf8(plain).map_err(|e| SecurityError::Other(format!("utf8: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypted_message_requires_decrypt() {
        // Description:
        //     Encrypted message requires decrypt.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_security::encrypted::encrypted_message_requires_decrypt();

        let session_material = "sess-1";
        let mut msg =
            EncryptedMessage::<String>::encrypt(&"hello".to_string(), session_material).unwrap();
        assert!(msg.decrypted.is_none());
        assert_eq!(msg.decrypt().unwrap(), "hello");
    }
}
