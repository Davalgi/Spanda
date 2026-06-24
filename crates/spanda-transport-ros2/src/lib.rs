//! ROS 2 transport backend extracted from Spanda core for lean-core package architecture.
//!
//! Provides native rclrs dynamic loading, an rclpy daemon bridge, and optional live
//! `ros2` CLI / Python bridge fallbacks. Spanda core retains thin `RuntimeValue`
//! compatibility shims that delegate here.
//!
pub mod adapter;
pub mod daemon;
pub mod live_bridge;
mod python_bridge;
pub mod rclrs;

#[cfg_attr(target_arch = "wasm32", path = "native_stub.rs")]
#[cfg_attr(not(target_arch = "wasm32"), path = "native.rs")]
mod native_loader;

pub mod native {
    pub use super::native_loader::*;
}

pub use adapter::Ros2TransportAdapter;
pub use daemon::{
    daemon_publish, daemon_script_path, daemon_service_call, daemon_subscribe, python_available,
};

/// Whether in-process ROS2 transport is enabled (`SPANDA_ROS2_RCLRS` env var).
pub fn rclrs_enabled() -> bool {
    // Description:
    //     Rclrs enabled.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `rclrs_enabled`.
    //
    // Example:

    //     let result = spanda_transport_ros2::rclrs_enabled();

    std::env::var("SPANDA_ROS2_RCLRS").is_ok()
}

/// Alias for `rclrs_enabled`.
pub fn rclrs_available() -> bool {
    // Description:
    //     Rclrs available.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `rclrs_available`.
    //
    // Example:

    //     let result = spanda_transport_ros2::rclrs_available();

    rclrs_enabled()
}

pub fn native_sdk_available() -> bool {
    // Description:
    //     Native sdk available.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `native_sdk_available`.
    //
    // Example:

    //     let result = spanda_transport_ros2::native_sdk_available();

    native::sdk_available()
}

pub fn init_node(name: &str) -> Result<(), String> {
    // Description:
    //     Init node.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: Result<(), String>
    //         Return value from `init_node`.
    //
    // Example:

    //     let result = spanda_transport_ros2::init_node(name);

    if native::sdk_available() {
        return native::init_node(name);
    }
    if daemon_subscribe("/spanda/rclrs/init") {
        let _ = name;
        Ok(())
    } else {
        Err(
            "ROS2 rclrs SDK unavailable — build libspanda_ros2_rclrs_native and source ROS 2"
                .into(),
        )
    }
}

pub fn try_native_publish(topic: &str, payload: &str) -> bool {
    // Description:
    //     Try native publish.
    //
    // Inputs:
    //     opic: &str
    //         Caller-supplied opic.
    //     payload: &str
    //         Caller-supplied payload.
    //
    // Outputs:
    //     result: bool
    //         Return value from `try_native_publish`.
    //
    // Example:

    //     let result = spanda_transport_ros2::try_native_publish(opic, payload);

    native::publish(topic, payload)
}

pub fn try_native_subscribe(topic: &str) -> bool {
    // Description:
    //     Try native subscribe.
    //
    // Inputs:
    //     opic: &str
    //         Caller-supplied opic.
    //
    // Outputs:
    //     result: bool
    //         Return value from `try_native_subscribe`.
    //
    // Example:

    //     let result = spanda_transport_ros2::try_native_subscribe(opic);

    native::subscribe(topic)
}

pub fn try_native_service_call(service: &str, service_type: &str, request: &str) -> bool {
    // Description:
    //     Try native service call.
    //
    // Inputs:
    //     service: &str
    //         Caller-supplied service.
    //     service_type: &str
    //         Caller-supplied service type.
    //     request: &str
    //         Caller-supplied request.
    //
    // Outputs:
    //     result: bool
    //         Return value from `try_native_service_call`.
    //
    // Example:

    //     let result = spanda_transport_ros2::try_native_service_call(service, service_type, reques);

    native::service_call(service, service_type, request)
}
