//! Shared types for lean-core provider contracts.
//!
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Stable identifier for a registered provider implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProviderId {
    pub package: String,
    pub name: String,
}

impl ProviderId {
    pub fn new(package: impl Into<String>, name: impl Into<String>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     package: impl Into<String>
        //         Caller-supplied package.
        //     name: impl Into<String>
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_runtime::provider_types::new(package, name);

        // let id = ProviderId::new("spanda-gps", "nmea");

        Self {
            package: package.into(),
            name: name.into(),
        }
    }
}

/// Capability tokens a provider may require from the runtime or other packages.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProviderCapability(pub String);

impl ProviderCapability {
    pub fn new(value: impl Into<String>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     value: impl Into<String>
        //         Caller-supplied value.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:

        //     let value = spanda_runtime::provider_types::new(value);

        Self(value.into())
    }
}

/// Safety tier declared by a provider package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderSafetyLevel {
    Experimental,
    Development,
    Production,
}

/// Metadata describing a registered provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderMetadata {
    pub id: ProviderId,
    pub description: String,
    pub safety_level: ProviderSafetyLevel,
    pub capabilities_required: Vec<ProviderCapability>,
    pub hardware_requirements: Vec<String>,
}

/// Runtime error returned by provider operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderError {
    pub provider: ProviderId,
    pub message: String,
}

impl ProviderError {
    pub fn new(provider: ProviderId, message: impl Into<String>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     provider: ProviderId
        //         Caller-supplied provider.
        //     essage: impl Into<String>
        //         Caller-supplied essage.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:

        //     let value = spanda_runtime::provider_types::new(provider, essage);

        Self {
            provider,
            message: message.into(),
        }
    }
}

pub type ProviderResult<T> = Result<T, ProviderError>;

/// In-memory capability set used by the provider registry.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProviderCapabilitySet {
    inner: HashSet<String>,
}

impl ProviderCapabilitySet {
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

        //     let value = spanda_runtime::provider_types::new();

        Self {
            inner: HashSet::new(),
        }
    }

    pub fn insert(&mut self, cap: impl Into<String>) {
        // Description:
        //     Insert.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     cap: impl Into<String>
        //         Caller-supplied cap.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::provider_types::insert(&mut self, cap);

        self.inner.insert(cap.into());
    }

    pub fn contains(&self, cap: &str) -> bool {
        // Description:
        //     Contains.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     cap: &str
        //         Caller-supplied cap.
        //
        // Outputs:
        //     result: bool
        //         Return value from `contains`.
        //
        // Example:

        //     let result = spanda_runtime::provider_types::contains(&self, cap);

        self.inner.contains(cap)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        // Description:
        //     Iter.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: impl Iterator<Item = &String>
        //         Return value from `iter`.
        //
        // Example:

        //     let result = spanda_runtime::provider_types::iter(&self);

        self.inner.iter()
    }
}
