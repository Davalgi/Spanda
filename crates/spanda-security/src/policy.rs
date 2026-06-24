//! Communication security policy modes for encryption, authentication, and integrity.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// When payload encryption is applied on a channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EncryptionMode {
    #[default]
    None,
    Optional,
    Required,
}

impl FromStr for EncryptionMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Description:
        //     From str.
        //
        // Inputs:
        //     s: &str
        //         Caller-supplied s.
        //
        // Outputs:
        //     result: Result<Self, Self::Err>
        //         Return value from `from_str`.
        //
        // Example:

        //     let result = spanda_security::policy::from_str(s);

        match s {
            "none" => Ok(Self::None),
            "optional" => Ok(Self::Optional),
            "required" => Ok(Self::Required),
            other => Err(format!("unknown encryption mode '{other}'")),
        }
    }
}

/// Peer authentication requirement for a channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationMode {
    #[default]
    None,
    Signed,
    Mutual,
}

impl FromStr for AuthenticationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Description:
        //     From str.
        //
        // Inputs:
        //     s: &str
        //         Caller-supplied s.
        //
        // Outputs:
        //     result: Result<Self, Self::Err>
        //         Return value from `from_str`.
        //
        // Example:

        //     let result = spanda_security::policy::from_str(s);

        match s {
            "none" => Ok(Self::None),
            "signed" => Ok(Self::Signed),
            "mutual" => Ok(Self::Mutual),
            other => Err(format!("unknown authentication mode '{other}'")),
        }
    }
}

/// Message integrity protection requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum IntegrityMode {
    #[default]
    None,
    Required,
}

impl FromStr for IntegrityMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Description:
        //     From str.
        //
        // Inputs:
        //     s: &str
        //         Caller-supplied s.
        //
        // Outputs:
        //     result: Result<Self, Self::Err>
        //         Return value from `from_str`.
        //
        // Example:

        //     let result = spanda_security::policy::from_str(s);

        match s {
            "none" => Ok(Self::None),
            "required" => Ok(Self::Required),
            other => Err(format!("unknown integrity mode '{other}'")),
        }
    }
}

/// Robot-wide default secure communication policy (`secure_comm { ... }`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureCommPolicy {
    pub encryption: EncryptionMode,
    pub authentication: AuthenticationMode,
    pub integrity: IntegrityMode,
}

impl SecureCommPolicy {
    pub fn dev_default() -> Self {
        // Description:
        //     Dev default.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `dev_default`.
        //
        // Example:

        //     let result = spanda_security::policy::dev_default();

        Self {
            encryption: EncryptionMode::Optional,
            authentication: AuthenticationMode::None,
            integrity: IntegrityMode::None,
        }
    }

    pub fn merge_bus(&self, bus: &BusSecurityConfig) -> Self {
        // Description:
        //     Merge bus.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     bus: &BusSecurityConfig
        //         Caller-supplied bus.
        //
        // Outputs:
        //     result: Self
        //         Return value from `merge_bus`.
        //
        // Example:

        //     let result = spanda_security::policy::merge_bus(&self, bus);

        Self {
            encryption: bus.encryption.unwrap_or(self.encryption),
            authentication: bus.authentication.unwrap_or(self.authentication),
            integrity: bus.integrity.unwrap_or(self.integrity),
        }
    }
}

/// Per-bus security overrides parsed from `bus name { encryption: required; ... }`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BusSecurityConfig {
    pub encryption: Option<EncryptionMode>,
    pub authentication: Option<AuthenticationMode>,
    pub integrity: Option<IntegrityMode>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption_mode_parsing() {
        // Description:
        //     Encryption mode parsing.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_security::policy::encryption_mode_parsing();

        assert_eq!(
            "required".parse::<EncryptionMode>().unwrap(),
            EncryptionMode::Required
        );
        assert!("invalid".parse::<EncryptionMode>().is_err());
    }
}
