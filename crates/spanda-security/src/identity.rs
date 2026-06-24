//! Robot and device identity with trust metadata for secure Spanda programs.

use crate::trust::TrustLevel;
use serde::{Deserialize, Serialize};
use spanda_audit::DeviceIdentity;

/// Extended device identity with trust metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotIdentity {
    pub device: DeviceIdentity,
    pub trust: TrustLevel,
}

impl RobotIdentity {
    pub fn new(id: impl Into<String>, public_key: impl Into<String>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     id: impl Into<String>
        //         Caller-supplied id.
        //     public_key: impl Into<String>
        //         Caller-supplied public key.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_security::identity::new(id, public_key);

        // Assemble the struct fields and return it.
        Self {
            device: DeviceIdentity::new(id, public_key),
            trust: TrustLevel::Trusted,
        }
    }

    pub fn with_trust(mut self, trust: TrustLevel) -> Self {
        // Description:
        //     With trust.
        //
        // Inputs:
        //     mut self: input value
        //         Caller-supplied mut self.
        //     rus: TrustLevel
        //         Caller-supplied rus.
        //
        // Outputs:
        //     result: Self
        //         Return value from `with_trust`.
        //
        // Example:
        //     let result = spanda_security::identity::with_trust(mut self, rus);

        // Call trust = trust; on the current instance.
        self.trust = trust;
        self
    }

    pub fn id(&self) -> &str {
        // Description:
        //     Id.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &str
        //         Return value from `id`.
        //
        // Example:
        //     let result = spanda_security::identity::id(&self);

        // Return id from this handle.
        &self.device.id
    }

    pub fn public_key(&self) -> &str {
        // Description:
        //     Public key.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &str
        //         Return value from `public_key`.
        //
        // Example:
        //     let result = spanda_security::identity::public_key(&self);

        // Return public key from this handle.
        &self.device.public_key
    }

    pub fn signing_key(&self) -> String {
        // Description:
        //     Signing key.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: String
        //         Return value from `signing_key`.
        //
        // Example:
        //     let result = spanda_security::identity::signing_key(&self);

        // Call default key on the current instance.
        self.device.default_key()
    }
}

impl From<DeviceIdentity> for RobotIdentity {
    fn from(device: DeviceIdentity) -> Self {
        // Description:
        //     From.
        //
        // Inputs:
        //     device: DeviceIdentity
        //         Caller-supplied device.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from`.
        //
        // Example:
        //     let result = spanda_security::identity::from(device);

        // Assemble the struct fields and return it.
        Self {
            device,
            trust: TrustLevel::Trusted,
        }
    }
}
