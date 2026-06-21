//! Bootstrap default provider registrations from core compatibility shims.
//!
use super::registry::ProviderRegistry;
use super::traits::TransportAdapterProvider;
use crate::transport::{MqttTransportAdapter, Ros2TransportAdapter};

/// Register built-in transport shims so legacy programs work without installed packages.
pub fn bootstrap_default_providers() -> ProviderRegistry {
    // Bootstrap default providers.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Registry pre-populated with core compatibility-shim transports.
    //
    // Options:
    // None.
    //
    // Example:
    // let registry = spanda_core::providers::bootstrap_default_providers();

    let mut registry = ProviderRegistry::new();
    registry.grant_capability("mqtt.publish");
    registry.grant_capability("mqtt.subscribe");
    registry.grant_capability("comm.ros2.publish");
    registry.grant_capability("comm.ros2.subscribe");

    registry.register_transport(Box::new(TransportAdapterProvider::new(
        "spanda-mqtt",
        "stub",
        MqttTransportAdapter::default(),
    )));
    registry.register_transport(Box::new(TransportAdapterProvider::new(
        "spanda-ros2",
        "stub",
        Ros2TransportAdapter::default(),
    )));
    registry
}
