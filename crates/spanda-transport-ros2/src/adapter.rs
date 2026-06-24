//! ROS2 `TransportAdapter` implementation with rclrs and live-bridge fallbacks.
//!
use spanda_runtime::RuntimeValue;
use spanda_transport::{
    payload_string_for_service, AdapterMessage, StubTransportState, TransportAdapter,
    TransportConfig,
};

use crate::live_bridge;
use crate::rclrs;

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

    //     let result = spanda_transport_ros2::adapter::payload_string(value);

    match value {
        RuntimeValue::String { value } => value.clone(),
        RuntimeValue::Number { value, .. } => value.to_string(),
        RuntimeValue::Bool { value } => value.to_string(),
        other => format!("{other:?}"),
    }
}

/// ROS2 transport adapter — logs locally; optionally forwards via rclrs or Python bridge.
#[derive(Debug, Default)]
pub struct Ros2TransportAdapter {
    state: StubTransportState,
}

impl TransportAdapter for Ros2TransportAdapter {
    fn kind(&self) -> spanda_ast::comm_decl::TransportKind {
        // Description:
        //     Kind.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: spanda_ast::comm_decl::TransportKind
        //         Return value from `kind`.
        //
        // Example:

        //     let result = spanda_transport_ros2::adapter::kind(&self);

        spanda_ast::comm_decl::TransportKind::Ros2
    }

    fn connect(&mut self, config: &TransportConfig) -> Result<(), String> {
        // Description:

        //     Connect.

        //

        // Inputs:

        //     &mut self: value

        //         Caller-supplied &mut self.

        //     config: &TransportConfig

        //         Caller-supplied config.

        //

        // Outputs:

        //     result: Result<(), String>

        //         Return value from `connect`.

        //

        // Example:

        //     let result = spanda_transport_ros2::adapter::connect(&mut self, config);
        self.state.connected = true;
        self.state.config = config.clone();
        Ok(())
    }

    fn disconnect(&mut self) {
        // Description:
        //     Disconnect.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_transport_ros2::adapter::disconnect(&mut self);

        self.state.connected = false;
    }

    fn is_connected(&self) -> bool {
        // Description:
        //     Is connected.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_connected`.
        //
        // Example:

        //     let result = spanda_transport_ros2::adapter::is_connected(&self);

        self.state.connected
    }

    fn publish(&mut self, topic: &str, message_type: &str, value: RuntimeValue) {
        // Description:

        //     Publish.

        //

        // Inputs:

        //     &mut self: value

        //         Caller-supplied &mut self.

        //     opic: &str

        //         Caller-supplied opic.

        //     essage_type: &str

        //         Caller-supplied essage type.

        //     value: RuntimeValue

        //         Caller-supplied value.

        //

        // Outputs:

        //     None.

        //

        // Example:

        //     let result = spanda_transport_ros2::adapter::publish(&mut self, opic, essage_type, value);
        if self.state.connected {
            self.state.publish(topic, message_type, value.clone());
        }

        // Forward to rclrs or live bridge when available.
        if rclrs::try_rclrs_publish(topic, &value) {
            return;
        }
        let _ = live_bridge::try_ros2_publish(topic, &payload_string(&value));
    }

    fn subscribe(&mut self, topic: &str) {
        // Description:

        //     Subscribe.

        //

        // Inputs:

        //     &mut self: value

        //         Caller-supplied &mut self.

        //     opic: &str

        //         Caller-supplied opic.

        //

        // Outputs:

        //     None.

        //

        // Example:

        //     let result = spanda_transport_ros2::adapter::subscribe(&mut self, opic);
        if self.state.connected {
            self.state.subscribe(topic);
        }

        // Forward to rclrs or live bridge when available.
        if rclrs::try_rclrs_subscribe(topic) {
            return;
        }
        let _ = live_bridge::try_ros2_subscribe(topic);
    }

    fn receive(&mut self, topic: &str) -> Option<RuntimeValue> {
        // Description:
        //     Receive.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     result: Option<RuntimeValue>
        //         Return value from `receive`.
        //
        // Example:

        //     let result = spanda_transport_ros2::adapter::receive(&mut self, opic);

        if self.state.connected {
            self.state.receive(topic)
        } else {
            None
        }
    }

    fn call_service(
        &mut self,
        service: &str,
        service_type: &str,
        request: Option<RuntimeValue>,
    ) -> RuntimeValue {
        // Description:
        //     Call service.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     service: &str
        //         Caller-supplied service.
        //     service_type: &str
        //         Caller-supplied service type.
        //     request: Option<RuntimeValue>
        //         Caller-supplied request.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `call_service`.
        //
        // Example:

        //     let result = spanda_transport_ros2::adapter::call_service(&mut self, service, service_type, reques);

        let request_text = request
            .as_ref()
            .map(payload_string_for_service)
            .unwrap_or_else(|| "{}".into());

        // Forward to rclrs or live bridge when available.
        if rclrs::try_rclrs_service_call(service, service_type, &request_text) {
            return StubTransportState::service_result(service_type);
        }
        let _ = live_bridge::try_ros2_service_call(service, service_type, &request_text);
        StubTransportState::service_result(service_type)
    }

    fn send_action(
        &mut self,
        _action: &str,
        action_type: &str,
        _goal: RuntimeValue,
    ) -> RuntimeValue {
        // Description:
        //     Send action.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     _action: &str
        //         Caller-supplied action.
        //     action_type: &str
        //         Caller-supplied action type.
        //     _goal: RuntimeValue
        //         Caller-supplied goal.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `send_action`.
        //
        // Example:

        //     let result = spanda_transport_ros2::adapter::send_action(&mut self, _action, action_type, _goal);

        StubTransportState::action_result(action_type)
    }

    fn published(&self) -> Vec<AdapterMessage> {
        // Description:
        //     Published.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<AdapterMessage>
        //         Return value from `published`.
        //
        // Example:

        //     let result = spanda_transport_ros2::adapter::published(&self);

        self.state.published.clone()
    }
}
