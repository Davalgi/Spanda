//! secure comm support for Spanda.
//!
use crate::capability::CapabilitySet;
use crate::encrypted::EncryptedMessage;
use crate::error::{SecurityError, SecurityResult};
use crate::identity::RobotIdentity;
use crate::policy::{AuthenticationMode, EncryptionMode, IntegrityMode};
use crate::signed::SignedMessage;
use crate::trust::TrustLevel;
use serde::{Deserialize, Serialize};

/// Security policy attached to a topic, service, or action endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurePolicy {
    pub signed: bool,
    pub min_trust: Option<TrustLevel>,
    pub requires: Vec<String>,
    pub encryption: EncryptionMode,
    pub authentication: AuthenticationMode,
    pub integrity: IntegrityMode,
    pub trusted_sources: Vec<String>,
    pub reject_untrusted: bool,
}

impl SecurePolicy {
    pub fn open() -> Self {
        // Description:
        //     Open.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `open`.
        //
        // Example:
        //     let result = spanda_security::secure_comm::open();

        // Build the result via default.
        Self::default()
    }

    pub fn signed_trusted() -> Self {
        // Description:
        //     Signed trusted.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `signed_trusted`.
        //
        // Example:
        //     let result = spanda_security::secure_comm::signed_trusted();

        // Assemble the struct fields and return it.
        Self {
            signed: true,
            min_trust: Some(TrustLevel::Trusted),
            requires: vec!["identity.verify".into()],
            encryption: EncryptionMode::None,
            authentication: AuthenticationMode::Signed,
            integrity: IntegrityMode::None,
            trusted_sources: Vec::new(),
            reject_untrusted: false,
        }
    }

    pub fn encrypted_signed() -> Self {
        // Description:
        //     Encrypted signed.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `encrypted_signed`.
        //
        // Example:

        //     let result = spanda_security::secure_comm::encrypted_signed();

        Self {
            signed: true,
            encryption: EncryptionMode::Required,
            authentication: AuthenticationMode::Signed,
            integrity: IntegrityMode::Required,
            ..Self::signed_trusted()
        }
    }

    pub fn check_trust(&self, trust: TrustLevel) -> SecurityResult<()> {
        // Description:
        //     Check trust.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     rus: TrustLevel
        //         Caller-supplied rus.
        //
        // Outputs:
        //     result: SecurityResult<()>
        //         Return value from `check_trust`.
        //
        // Example:
        //     let result = spanda_security::secure_comm::check_trust(&self, rus);
        // use required when min trust is present.

        // Emit output when min trust provides a required.
        if let Some(required) = self.min_trust {
            // Take the branch when satisfies is false.
            if !trust.satisfies(required) {
                return Err(SecurityError::TrustInsufficient {
                    required: required.as_str().into(),
                    actual: trust.as_str().into(),
                });
            }
        }
        Ok(())
    }

    pub fn check_capabilities(&self, caps: &CapabilitySet) -> SecurityResult<()> {
        // Description:
        //     Check capabilities.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     caps: &CapabilitySet
        //         Caller-supplied caps.
        //
        // Outputs:
        //     result: SecurityResult<()>
        //         Return value from `check_capabilities`.
        //
        // Example:
        //     let result = spanda_security::secure_comm::check_capabilities(&self, caps);

        // Validate each requested capability.
        for cap in &self.requires {
            caps.require(cap)?;
        }
        Ok(())
    }

    pub fn prepare_outbound(
        &self,
        payload: &str,
        identity: Option<&RobotIdentity>,
        caps: &CapabilitySet,
        endpoint: &str,
    ) -> SecurityResult<Option<SignedMessage>> {
        // Description:
        //     Prepare outbound.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     payload: &str
        //         Caller-supplied payload.
        //     identity: Option<&RobotIdentity>
        //         Caller-supplied identity.
        //     caps: &CapabilitySet
        //         Caller-supplied caps.
        //     endpoin: &str
        //         Caller-supplied endpoin.
        //
        // Outputs:
        //     result: SecurityResult<Option<SignedMessage>>
        //         Return value from `prepare_outbound`.
        //
        // Example:

        //     let result = spanda_security::secure_comm::prepare_outbound(&self, payload, identity, caps, endpoin);

        let secured = self.signed
            || self.min_trust.is_some()
            || !self.requires.is_empty()
            || self.encryption != EncryptionMode::None
            || self.authentication != AuthenticationMode::None
            || self.integrity != IntegrityMode::None;

        if secured {
            self.check_capabilities(caps)?;

            if self.encryption == EncryptionMode::Required {
                caps.require("crypto.encrypt")?;
            }
            if self.authentication == AuthenticationMode::Mutual {
                caps.require("identity.verify")?;
            }

            if let Some(id) = identity {
                self.check_trust(id.trust)?;

                if self.signed {
                    caps.require("identity.sign")?;
                    return Ok(Some(SignedMessage::sign(payload, id)));
                }
                return Ok(None);
            }
            return Err(SecurityError::IdentityRequired {
                operation: endpoint.to_string(),
            });
        }
        Ok(None)
    }

    pub fn encrypt_payload(
        &self,
        payload: &str,
        caps: &CapabilitySet,
        session_material: &str,
    ) -> SecurityResult<String> {
        // Description:
        //     Encrypt payload.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     payload: &str
        //         Caller-supplied payload.
        //     caps: &CapabilitySet
        //         Caller-supplied caps.
        //     session_material: &str
        //         Caller-supplied session material.
        //
        // Outputs:
        //     result: SecurityResult<String>
        //         Return value from `encrypt_payload`.
        //
        // Example:

        //     let result = spanda_security::secure_comm::encrypt_payload(&self, payload, caps, session_material);

        if self.encryption == EncryptionMode::None {
            return Ok(payload.to_string());
        }
        if self.encryption == EncryptionMode::Required {
            caps.require("crypto.encrypt")?;
        }
        let enc = EncryptedMessage::<String>::encrypt(&payload.to_string(), session_material)?;
        Ok(enc.ciphertext().to_string())
    }

    pub fn verify_inbound(
        &self,
        signed: Option<&SignedMessage>,
        identity: Option<&RobotIdentity>,
        caps: &CapabilitySet,
        endpoint: &str,
        source_id: Option<&str>,
    ) -> SecurityResult<()> {
        // Description:
        //     Verify inbound.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     signed: Option<&SignedMessage>
        //         Caller-supplied signed.
        //     identity: Option<&RobotIdentity>
        //         Caller-supplied identity.
        //     caps: &CapabilitySet
        //         Caller-supplied caps.
        //     endpoin: &str
        //         Caller-supplied endpoin.
        //     source_id: Option<&str>
        //         Caller-supplied source id.
        //
        // Outputs:
        //     result: SecurityResult<()>
        //         Return value from `verify_inbound`.
        //
        // Example:

        //     let result = spanda_security::secure_comm::verify_inbound(&self, signed, identity, caps, endpoin, source_id);

        if !self.trusted_sources.is_empty() {
            let sid =
                source_id.ok_or_else(|| SecurityError::UntrustedSource("unknown".to_string()))?;
            self.check_trusted_source(sid)?;
        }

        let secured = self.signed
            || self.min_trust.is_some()
            || !self.requires.is_empty()
            || self.encryption != EncryptionMode::None
            || self.authentication != AuthenticationMode::None
            || self.integrity != IntegrityMode::None;

        if secured {
            self.check_capabilities(caps)?;
            let id = identity.ok_or_else(|| SecurityError::IdentityRequired {
                operation: endpoint.to_string(),
            })?;
            self.check_trust(id.trust)?;

            if self.encryption == EncryptionMode::Required {
                caps.require("crypto.decrypt")?;
            }

            if self.signed || self.integrity == IntegrityMode::Required {
                let msg = signed.ok_or_else(|| SecurityError::SecureEndpoint {
                    endpoint: endpoint.to_string(),
                    reason: "missing signature".into(),
                })?;

                if !msg.verify(id)? {
                    return Err(SecurityError::SignatureInvalid);
                }
            }
        }
        Ok(())
    }

    pub fn check_trusted_source(&self, source_id: &str) -> SecurityResult<()> {
        // Description:
        //     Check trusted source.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     source_id: &str
        //         Caller-supplied source id.
        //
        // Outputs:
        //     result: SecurityResult<()>
        //         Return value from `check_trusted_source`.
        //
        // Example:

        //     let result = spanda_security::secure_comm::check_trusted_source(&self, source_id);

        if self.trusted_sources.is_empty() {
            return Ok(());
        }
        if self.trusted_sources.iter().any(|s| s == source_id) {
            Ok(())
        } else if self.reject_untrusted {
            Err(SecurityError::UntrustedSource(source_id.to_string()))
        } else {
            Err(SecurityError::SecureEndpoint {
                endpoint: "trusted_sources".into(),
                reason: format!("untrusted source '{source_id}'"),
            })
        }
    }
}

/// Registry of secure policies keyed by endpoint path.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecureEndpointRegistry {
    policies: std::collections::HashMap<String, SecurePolicy>,
}

impl SecureEndpointRegistry {
    pub fn new() -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_security::secure_comm::new();

        // Build the result via default.
        Self::default()
    }

    pub fn register(&mut self, path: impl Into<String>, policy: SecurePolicy) {
        // Description:
        //     Register.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     path: impl Into<String>
        //         Caller-supplied path.
        //     policy: SecurePolicy
        //         Caller-supplied policy.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_security::secure_comm::register(&mut self, path, policy);

        // Append into self.
        self.policies.insert(path.into(), policy);
    }

    pub fn get(&self, path: &str) -> Option<&SecurePolicy> {
        // Description:
        //     Get.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     path: &str
        //         Caller-supplied path.
        //
        // Outputs:
        //     result: Option<&SecurePolicy>
        //         Return value from `get`.
        //
        // Example:
        //     let result = spanda_security::secure_comm::get(&self, path);

        // Call get on the current instance.
        self.policies.get(path)
    }

    pub fn policy_or_open(&self, path: &str) -> SecurePolicy {
        // Description:
        //     Policy or open.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     path: &str
        //         Caller-supplied path.
        //
        // Outputs:
        //     result: SecurePolicy
        //         Return value from `policy_or_open`.
        //
        // Example:
        //     let result = spanda_security::secure_comm::policy_or_open(&self, path);

        // Call get on the current instance.
        self.get(path).cloned().unwrap_or_default()
    }

    pub fn len(&self) -> usize {
        // Description:
        //     Len.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: usize
        //         Return value from `len`.
        //
        // Example:
        //     let result = spanda_security::secure_comm::len(&self);

        // Call len on the current instance.
        self.policies.len()
    }

    pub fn is_empty(&self) -> bool {
        // Description:
        //     Is empty.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_empty`.
        //
        // Example:
        //     let result = spanda_security::secure_comm::is_empty(&self);

        // Call is empty on the current instance.
        self.policies.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::CapabilitySet;

    #[test]
    fn secure_topic_requires_identity() {
        // Description:
        //     Secure topic requires identity.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_security::secure_comm::secure_topic_requires_identity();

        let policy = SecurePolicy::signed_trusted();
        let mut caps = CapabilitySet::new();
        caps.grant("identity.sign");
        caps.grant("identity.verify");
        let err = policy
            .prepare_outbound("data", None, &caps, "/cmd")
            .unwrap_err();
        assert!(matches!(err, SecurityError::IdentityRequired { .. }));
    }
}
