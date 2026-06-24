//! Application-level package permissions derived from manifest capability declarations.

use crate::capability::CapabilitySet;
use serde::{Deserialize, Serialize};

/// Application-level permissions for package validation and runtime gating.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackagePermissions {
    pub capabilities: CapabilitySet,
}

impl PackagePermissions {
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
        //     let value = spanda_security::permissions::new();

        // Build the result via default.
        Self::default()
    }

    pub fn permissive() -> Self {
        // Description:
        //     Permissive.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `permissive`.
        //
        // Example:
        //     let result = spanda_security::permissions::permissive();

        // Assemble the struct fields and return it.
        Self {
            capabilities: CapabilitySet::permissive(),
        }
    }

    pub fn from_capabilities(caps: impl IntoIterator<Item = impl Into<String>>) -> Self {
        // Description:
        //     From capabilities.
        //
        // Inputs:
        //     caps: impl IntoIterator<Item = impl Into<String>>
        //         Caller-supplied caps.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_capabilities`.
        //
        // Example:
        //     let result = spanda_security::permissions::from_capabilities(caps);

        // Create mutable set for accumulating results.
        let mut set = CapabilitySet::new();
        set.grant_all(caps);
        Self { capabilities: set }
    }
}
