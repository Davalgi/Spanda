//! Stub native rclrs loader for WASM targets.

pub fn sdk_available() -> bool {
    // Description:
    //     Sdk available.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `sdk_available`.
    //
    // Example:

    //     let result = spanda_transport_ros2::native_stub::sdk_available();

    false
}

pub fn init_node(_name: &str) -> Result<(), String> {
    // Description:
    //     Init node.
    //
    // Inputs:
    //     _name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: Result<(), String>
    //         Return value from `init_node`.
    //
    // Example:

    //     let result = spanda_transport_ros2::native_stub::init_node(_name);

    Err("native rclrs is unavailable on wasm32".into())
}

pub fn publish(_topic: &str, _payload: &str) -> bool {
    // Description:
    //     Publish.
    //
    // Inputs:
    //     _topic: &str
    //         Caller-supplied topic.
    //     _payload: &str
    //         Caller-supplied payload.
    //
    // Outputs:
    //     result: bool
    //         Return value from `publish`.
    //
    // Example:

    //     let result = spanda_transport_ros2::native_stub::publish(_topic, _payload);

    false
}

pub fn subscribe(_topic: &str) -> bool {
    // Description:
    //     Subscribe.
    //
    // Inputs:
    //     _topic: &str
    //         Caller-supplied topic.
    //
    // Outputs:
    //     result: bool
    //         Return value from `subscribe`.
    //
    // Example:

    //     let result = spanda_transport_ros2::native_stub::subscribe(_topic);

    false
}

pub fn service_call(_service: &str, _service_type: &str, _request: &str) -> bool {
    // Description:
    //     Service call.
    //
    // Inputs:
    //     _service: &str
    //         Caller-supplied service.
    //     _service_type: &str
    //         Caller-supplied service type.
    //     request: &str
    //         Caller-supplied request.
    //
    // Outputs:
    //     result: bool
    //         Return value from `service_call`.
    //
    // Example:

    //     let result = spanda_transport_ros2::native_stub::service_call(_service, _service_type, _reques);

    false
}
