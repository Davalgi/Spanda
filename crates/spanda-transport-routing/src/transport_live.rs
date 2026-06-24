//! RuntimeValue live transport hooks for ROS2 and MQTT backends.
//!
use spanda_runtime::value::RuntimeValue;

pub use spanda_transport_mqtt::{mqtt_live_enabled, try_mqtt_publish as try_mqtt_publish_str};
pub use spanda_transport_ros2::live_bridge::{
    ros2_live_enabled, ros2_native_enabled, try_ros2_bridge_publish, try_ros2_bridge_service_call,
    try_ros2_bridge_subscribe, try_ros2_native_publish, try_ros2_native_service_call,
    try_ros2_native_subscribe, try_ros2_publish as try_ros2_publish_str,
    try_ros2_service_call as try_ros2_service_call_str,
    try_ros2_subscribe as try_ros2_subscribe_str,
};

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

    //     let result = spanda_transport_routing::transport_live::payload_string(value);

    match value {
        RuntimeValue::String { value } => value.clone(),
        RuntimeValue::Number { value, .. } => value.to_string(),
        RuntimeValue::Bool { value } => value.to_string(),
        other => format!("{other:?}"),
    }
}

pub fn try_ros2_publish(topic: &str, value: &RuntimeValue) -> bool {
    // Description:
    //     Try ros2 publish.
    //
    // Inputs:
    //     opic: &str
    //         Caller-supplied opic.
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: bool
    //         Return value from `try_ros2_publish`.
    //
    // Example:

    //     let result = spanda_transport_routing::transport_live::try_ros2_publish(opic, value);

    try_ros2_publish_str(topic, &payload_string(value))
}

pub fn try_ros2_subscribe(topic: &str) -> bool {
    // Description:
    //     Try ros2 subscribe.
    //
    // Inputs:
    //     opic: &str
    //         Caller-supplied opic.
    //
    // Outputs:
    //     result: bool
    //         Return value from `try_ros2_subscribe`.
    //
    // Example:

    //     let result = spanda_transport_routing::transport_live::try_ros2_subscribe(opic);

    try_ros2_subscribe_str(topic)
}

pub fn try_ros2_service_call(service: &str, service_type: &str, request: &str) -> bool {
    // Description:
    //     Try ros2 service call.
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
    //         Return value from `try_ros2_service_call`.
    //
    // Example:

    //     let result = spanda_transport_routing::transport_live::try_ros2_service_call(service, service_type, reques);

    try_ros2_service_call_str(service, service_type, request)
}

pub fn try_mqtt_publish(topic: &str, value: &RuntimeValue) -> bool {
    // Description:
    //     Try mqtt publish.
    //
    // Inputs:
    //     opic: &str
    //         Caller-supplied opic.
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: bool
    //         Return value from `try_mqtt_publish`.
    //
    // Example:

    //     let result = spanda_transport_routing::transport_live::try_mqtt_publish(opic, value);

    try_mqtt_publish_str(topic, &payload_string(value))
}
