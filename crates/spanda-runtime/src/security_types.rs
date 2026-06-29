//! Runtime security policy types shared across interpreter and service bridges.
//!
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Runtime trust tier for devices, packages, and communication endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    #[default]
    Untrusted,
    Restricted,
    Trusted,
    Certified,
}

impl TrustLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Untrusted => "untrusted",
            Self::Restricted => "restricted",
            Self::Trusted => "trusted",
            Self::Certified => "certified",
        }
    }
}

impl FromStr for TrustLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "untrusted" => Ok(Self::Untrusted),
            "restricted" => Ok(Self::Restricted),
            "trusted" => Ok(Self::Trusted),
            "certified" => Ok(Self::Certified),
            other => Err(format!("unknown trust level '{other}'")),
        }
    }
}

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
        match s {
            "none" => Ok(Self::None),
            "required" => Ok(Self::Required),
            other => Err(format!("unknown integrity mode '{other}'")),
        }
    }
}

/// Robot-wide default secure communication policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SecureCommPolicy {
    pub encryption: EncryptionMode,
    pub authentication: AuthenticationMode,
    pub integrity: IntegrityMode,
}

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

/// Named trust boundary declared in Spanda source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustBoundaryKind {
    RobotInternal,
    RobotToRobot,
    RobotToCloud,
    OperatorToRobot,
}

impl FromStr for TrustBoundaryKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "robot_internal" => Ok(Self::RobotInternal),
            "robot_to_robot" => Ok(Self::RobotToRobot),
            "robot_to_cloud" => Ok(Self::RobotToCloud),
            "operator_to_robot" => Ok(Self::OperatorToRobot),
            other => Err(format!("unknown trust boundary '{other}'")),
        }
    }
}

/// Where a secret value is resolved from.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum SecretSource {
    Env { var: String },
    File { path: String },
    Literal { value: String },
}

/// Opaque handle to a resolved secret.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretHandle {
    pub name: String,
    pub source: SecretSource,
}

/// Minimal robot identity for runtime security wiring.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotIdentity {
    pub id: String,
    pub public_key: String,
    pub trust: TrustLevel,
}

impl RobotIdentity {
    pub fn new(id: impl Into<String>, public_key: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            public_key: public_key.into(),
            trust: TrustLevel::Trusted,
        }
    }

    pub fn with_trust(mut self, trust: TrustLevel) -> Self {
        self.trust = trust;
        self
    }
}

/// Per-transport TLS / encryption configuration from `bus { ... }` declarations.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BusTransportSecurity {
    pub encryption: EncryptionMode,
    pub authentication: AuthenticationMode,
    pub integrity: IntegrityMode,
    pub cert_path: Option<String>,
    pub key_secret: Option<String>,
    pub key_path: Option<String>,
}

/// Transport bus setup passed from interpreter setup into `CommBusHost`.
#[derive(Debug, Clone, Default)]
pub struct CommTransportSetup {
    pub node_name: Option<String>,
    pub broker_url: Option<String>,
    pub namespace: Option<String>,
    pub domain_id: Option<u32>,
    pub client_id: Option<String>,
    pub security: BusTransportSecurity,
}
