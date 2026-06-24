//! secrets support for Spanda.
//!
use crate::error::{SecurityError, SecurityResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Where a secret value is resolved from.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum SecretSource {
    Env { var: String },
    File { path: String },
    Literal { value: String },
}

/// Opaque handle to a resolved secret (value is not exposed in logs).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretHandle {
    pub name: String,
    pub source: SecretSource,
}

impl SecretHandle {
    pub fn resolve(&self) -> SecurityResult<String> {
        // Description:
        //     Resolve.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: SecurityResult<String>
        //         Return value from `resolve`.
        //
        // Example:
        //     let result = spanda_security::secrets::resolve(&self);

        // Match on source and handle each case.
        match &self.source {
            SecretSource::Env { var } => std::env::var(var).map_err(|_| {
                SecurityError::SecretNotFound(format!("environment variable '{var}'"))
            }),
            SecretSource::File { path } => std::fs::read_to_string(path)
                .map_err(|_| SecurityError::SecretNotFound(format!("secret file '{path}'"))),
            SecretSource::Literal { value } => Ok(value.clone()),
        }
    }

    /// Redacted representation safe for audit logs.
    pub fn redacted_label(&self) -> String {
        // Description:
        //     Redacted label.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: String
        //         Return value from `redacted_label`.
        //
        // Example:
        //     let result = spanda_security::secrets::redacted_label(&self);

        // Produce name) as the result.
        format!("secret:{}", self.name)
    }
}

/// In-memory secret store keyed by declaration name.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretStore {
    secrets: HashMap<String, SecretHandle>,
}

impl SecretStore {
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
        //     let value = spanda_security::secrets::new();

        // Build the result via default.
        Self::default()
    }

    pub fn register(&mut self, handle: SecretHandle) {
        // Description:
        //     Register.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     handle: SecretHandle
        //         Caller-supplied handle.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_security::secrets::register(&mut self, handle);

        // Append into self.
        self.secrets.insert(handle.name.clone(), handle);
    }

    pub fn get(&self, name: &str) -> SecurityResult<&SecretHandle> {
        // Description:
        //     Get.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: SecurityResult<&SecretHandle>
        //         Return value from `get`.
        //
        // Example:
        //     let result = spanda_security::secrets::get(&self, name);

        // Call secrets on the current instance.
        self.secrets
            .get(name)
            .ok_or_else(|| SecurityError::SecretNotFound(name.to_string()))
    }

    pub fn resolve(&self, name: &str) -> SecurityResult<String> {
        // Description:
        //     Resolve.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: SecurityResult<String>
        //         Return value from `resolve`.
        //
        // Example:
        //     let result = spanda_security::secrets::resolve(&self, name);

        // Call get on the current instance.
        self.get(name)?.resolve()
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        // Description:
        //     Names.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: impl Iterator<Item = &str>
        //         Return value from `names`.
        //
        // Example:
        //     let result = spanda_security::secrets::names(&self);

        // Transform self and continue the chain.
        self.secrets.keys().map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_secret_roundtrip() {
        // Description:
        //     Literal secret roundtrip.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_security::secrets::literal_secret_roundtrip();

        let mut store = SecretStore::new();
        store.register(SecretHandle {
            name: "api_key".into(),
            source: SecretSource::Literal {
                value: "test-key".into(),
            },
        });
        assert_eq!(store.resolve("api_key").unwrap(), "test-key");
    }
}
