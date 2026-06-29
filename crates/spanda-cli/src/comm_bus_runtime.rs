//! CLI-injected routing comm bus factory for live transport runs.
//!
use spanda_comm::CommBusFactory;
use spanda_transport_routing::runtime_bridge::routing_comm_bus_factory_fn;

/// Comm bus factory for default `spanda` CLI runs with routing transport.
pub fn default_comm_bus_factory() -> CommBusFactory {
    routing_comm_bus_factory_fn()
}
