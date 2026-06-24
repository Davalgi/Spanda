//! In-process ROS2 transport orchestration (native rclrs, daemon, Python bridge).
//!
use spanda_runtime::RuntimeValue;

use crate::daemon::{daemon_publish, daemon_service_call, daemon_subscribe};
use crate::live_bridge::{
    try_ros2_bridge_publish, try_ros2_bridge_service_call, try_ros2_bridge_subscribe,
};
use crate::native;

fn payload_string(value: &RuntimeValue) -> String {
    // Description:

    //     Payload string.

    //

    // Inputs:

    //     value: &RuntimeValue

    //         Caller-supplied value.

    //

    // Outputs:

    //     result: String

    //         Return value from `payload_string`.

    //

    // Example:

    //     let result = spanda_transport_ros2::rclrs::payload_string(value);
    match value {
        RuntimeValue::String { value } => value.clone(),
        RuntimeValue::Number { value, .. } => value.to_string(),
        RuntimeValue::Bool { value } => value.to_string(),
        other => format!("{other:?}"),
    }
}

/// Publish on ROS2 when `SPANDA_ROS2_RCLRS` is set, trying native, daemon, then bridge.
pub fn try_rclrs_publish(topic: &str, value: &RuntimeValue) -> bool {
    // Description:
    //     Try rclrs publish.
    //
    // Inputs:
    //     opic: &str
    //         Caller-supplied opic.
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: bool
    //         Return value from `try_rclrs_publish`.
    //
    // Example:
    //     let result = spanda_transport_ros2::rclrs::try_rclrs_publish(opic, value);

    // Skip when in-process ROS2 transport is disabled.
    if !crate::rclrs_enabled() {
        return false;
    }

    // Prefer native rclrs, then daemon, then Python bridge.
    let payload = payload_string(value);
    if native::publish(topic, &payload) {
        return true;
    }
    if daemon_publish(topic, &payload) {
        return true;
    }
    try_ros2_bridge_publish(topic, &payload)
}

/// Subscribe on ROS2 when `SPANDA_ROS2_RCLRS` is set.
pub fn try_rclrs_subscribe(topic: &str) -> bool {
    // Description:
    //     Try rclrs subscribe.
    //
    // Inputs:
    //     opic: &str
    //         Caller-supplied opic.
    //
    // Outputs:
    //     result: bool
    //         Return value from `try_rclrs_subscribe`.
    //
    // Example:
    //     let result = spanda_transport_ros2::rclrs::try_rclrs_subscribe(opic);

    // Skip when in-process ROS2 transport is disabled.
    if !crate::rclrs_enabled() {
        return false;
    }

    // Prefer native rclrs, then daemon, then Python bridge.
    if native::subscribe(topic) {
        return true;
    }
    if daemon_subscribe(topic) {
        return true;
    }
    try_ros2_bridge_subscribe(topic)
}

/// Call a ROS2 service when `SPANDA_ROS2_RCLRS` is set.
pub fn try_rclrs_service_call(service: &str, service_type: &str, request: &str) -> bool {
    // Description:
    //     Try rclrs service call.
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
    //         Return value from `try_rclrs_service_call`.
    //
    // Example:
    //     let result = spanda_transport_ros2::rclrs::try_rclrs_service_call(service, service_type, reques);

    // Skip when in-process ROS2 transport is disabled.
    if !crate::rclrs_enabled() {
        return false;
    }

    // Prefer native rclrs, then daemon, then Python bridge.
    if native::service_call(service, service_type, request) {
        return true;
    }
    if daemon_service_call(service, service_type, request) {
        return true;
    }
    try_ros2_bridge_service_call(service, service_type, request)
}

#[cfg(test)]
mod tests {
    #[test]
    fn rclrs_off_by_default() {
        // Description:
        //     Rclrs off by default.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_transport_ros2::rclrs::rclrs_off_by_default();

        std::env::remove_var("SPANDA_ROS2_RCLRS");
        assert!(!crate::rclrs_enabled());
    }
}
