//! capability support for Spanda.
//!
use crate::error::{SecurityError, SecurityResult};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Known package/runtime capability identifiers.
pub fn known_capabilities() -> &'static [&'static str] {
    // Description:
    //     Known capabilities.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: &'static [&'static str]
    //         Return value from `known_capabilities`.
    //
    // Example:
    //     let result = spanda_security::capability::known_capabilities();

    // Return the static list of known values.
    &[
        "network.outbound",
        "network.inbound",
        "camera.read",
        "lidar.read",
        "imu.read",
        "gps.read",
        "network.status",
        "wifi.connect",
        "bluetooth.scan",
        "bluetooth.pair",
        "cellular.connect",
        "network.failover",
        "motion.propose",
        "actuator.execute",
        "actuator.execute.safe",
        "serial.port",
        "storage.read",
        "storage.write",
        "ai.inference",
        "ros2.publish",
        "ros2.subscribe",
        "audit.write",
        "audit.read",
        "identity.sign",
        "identity.verify",
        "identity.read",
        "ledger.anchor",
        "crypto.encrypt",
        "crypto.decrypt",
        "crypto.sign",
        "crypto.verify",
        "secret.read",
        "secure_topic.publish",
        "secure_topic.subscribe",
        "positioning.read",
        "mqtt.publish",
        "mqtt.subscribe",
        "connectivity.wifi",
        "connectivity.ble",
        "connectivity.cellular",
        "navigation.plan",
        "fleet.orchestrate",
        "slam.localize",
        "slam.map",
        "deploy.rollout",
        "deploy.rollback",
        "dds.publish",
        "dds.subscribe",
        "ai.invoke",
        "vision.detect",
        "simulation.step",
        "cloud.invoke",
        "audit.append",
        "maintenance.health",
        "manipulation.plan",
    ]
}

pub fn is_known_capability(cap: &str) -> bool {
    // Description:
    //     Is known capability.
    //
    // Inputs:
    //     cap: &str
    //         Caller-supplied cap.
    //
    // Outputs:
    //     result: bool
    //         Return value from `is_known_capability`.
    //
    // Example:
    //     let result = spanda_security::capability::is_known_capability(cap);

    // Produce contains as the result.
    known_capabilities().contains(&cap)
}

/// Granted capability token (maps to package `[capabilities]` and robot `permissions`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission {
    pub capability: String,
}

impl Permission {
    pub fn new(capability: impl Into<String>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     capability: impl Into<String>
        //         Caller-supplied capability.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_security::capability::new(capability);

        // Assemble the struct fields and return it.
        Self {
            capability: capability.into(),
        }
    }
}

/// Set of granted capabilities with runtime enforcement.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapabilitySet {
    granted: HashSet<String>,
    permissive: bool,
}

impl CapabilitySet {
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
        //     let value = spanda_security::capability::new();

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
        //     let result = spanda_security::capability::permissive();

        // Assemble the struct fields and return it.
        Self {
            granted: known_capabilities()
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
            permissive: true,
        }
    }

    pub fn grant(&mut self, capability: impl Into<String>) {
        // Description:
        //     Grant.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     capability: impl Into<String>
        //         Caller-supplied capability.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_security::capability::grant(&mut self, capability);

        // Append into self.
        self.granted.insert(capability.into());
    }

    pub fn grant_all(&mut self, caps: impl IntoIterator<Item = impl Into<String>>) {
        // Description:
        //     Grant all.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     caps: impl IntoIterator<Item = impl Into<String>>
        //         Caller-supplied caps.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_security::capability::grant_all(&mut self, caps);

        // Validate each requested capability.
        for cap in caps {
            self.grant(cap);
        }
    }

    pub fn has(&self, capability: &str) -> bool {
        // Description:
        //     Has.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     capability: &str
        //         Caller-supplied capability.
        //
        // Outputs:
        //     result: bool
        //         Return value from `has`.
        //
        // Example:
        //     let result = spanda_security::capability::has(&self, capability);

        // Call contains on the current instance.
        self.permissive || self.granted.contains(capability)
    }

    pub fn require(&self, capability: &str) -> SecurityResult<()> {
        // Description:
        //     Require.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     capability: &str
        //         Caller-supplied capability.
        //
        // Outputs:
        //     result: SecurityResult<()>
        //         Return value from `require`.
        //
        // Example:
        //     let result = spanda_security::capability::require(&self, capability);

        // take this path when self.has(capability).
        if self.has(capability) {
            Ok(())
        } else {
            Err(SecurityError::CapabilityDenied(capability.to_string()))
        }
    }

    pub fn granted(&self) -> impl Iterator<Item = &str> {
        // Description:
        //     Granted.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: impl Iterator<Item = &str>
        //         Return value from `granted`.
        //
        // Example:
        //     let result = spanda_security::capability::granted(&self);

        // Iterate over granted.
        self.granted.iter().map(String::as_str)
    }
}

/// Maps high-level runtime operations to required package capabilities.
pub fn capability_for_operation(operation: &str) -> Option<&'static str> {
    // Description:
    //     Capability for operation.
    //
    // Inputs:
    //     operation: &str
    //         Caller-supplied operation.
    //
    // Outputs:
    //     result: Option<&'static str>
    //         Return value from `capability_for_operation`.
    //
    // Example:
    //     let result = spanda_security::capability::capability_for_operation(operation);

    // Match on operation and handle each case.
    match operation {
        "audit.record" | "audit.append" => Some("audit.write"),
        "audit.export" | "audit.read" => Some("audit.read"),
        "sign" | "identity.sign" | "crypto.sign" => Some("identity.sign"),
        "verify_signature" | "identity.verify" | "crypto.verify" => Some("identity.verify"),
        "crypto.encrypt" => Some("crypto.encrypt"),
        "crypto.decrypt" => Some("crypto.decrypt"),
        "secret.resolve" | "secret.read" => Some("secret.read"),
        "secure_topic.publish" => Some("secure_topic.publish"),
        "secure_topic.subscribe" => Some("secure_topic.subscribe"),
        "ledger.anchor" => Some("ledger.anchor"),
        "actuator.execute" => Some("actuator.execute"),
        "cellular.sim_identity" => Some("cellular.connect"),
        "network.publish" => Some("network.outbound"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_enforcement() {
        // Description:
        //     Capability enforcement.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_security::capability::capability_enforcement();

        let mut caps = CapabilitySet::new();
        caps.grant("audit.write");
        assert!(caps.require("audit.write").is_ok());
        assert!(caps.require("ledger.anchor").is_err());
    }
}
