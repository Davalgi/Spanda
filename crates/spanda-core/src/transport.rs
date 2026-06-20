//! Pluggable transport adapters for ROS2, MQTT, DDS, and WebSocket.
//!
//! Each adapter records operations for simulation/testing and exposes a uniform
//! interface that real broker/node integrations can implement later.

use crate::comm::{
    CommBus, DiscoverFilter, DiscoverTarget, InMemoryCommBus, PublishedCommMessage,
    SimNetworkConfig, TransportKind,
};
use crate::runtime::RuntimeValue;
use crate::transport_live as live;
use crate::transport_rclrs as rclrs;
use std::collections::{HashMap, VecDeque};

fn payload_string_for_service(value: &RuntimeValue) -> String {
    // Payload string for service.
    //
    // Parameters:
    // - `value` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::transport::payload_string_for_service(value);

    match value {
        RuntimeValue::String { value } => {
            format!(
                "{{data: \"{}\"}}",
                value.replace('\\', "\\\\").replace('"', "\\\"")
            )
        }
        RuntimeValue::Number { value, .. } => format!("{{value: {value}}}"),
        RuntimeValue::Bool { value } => format!("{{ok: {value}}}"),
        other => format!("{{raw: \"{other:?}\"}}"),
    }
}

// ── Transport adapter trait ───────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct TransportConfig {
    pub broker_url: Option<String>,
    pub node_name: Option<String>,
    pub namespace: Option<String>,
    pub domain_id: Option<u32>,
    pub client_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AdapterMessage {
    pub topic: String,
    pub message_type: String,
    pub value: RuntimeValue,
}

pub trait TransportAdapter {
    fn kind(&self) -> TransportKind;
    fn connect(&mut self, config: &TransportConfig) -> Result<(), String>;
    fn disconnect(&mut self);
    fn is_connected(&self) -> bool;
    fn publish(&mut self, topic: &str, message_type: &str, value: RuntimeValue);
    fn subscribe(&mut self, topic: &str);
    fn receive(&mut self, topic: &str) -> Option<RuntimeValue>;
    fn call_service(
        &mut self,
        service: &str,
        service_type: &str,
        request: Option<RuntimeValue>,
    ) -> RuntimeValue;
    fn send_action(&mut self, action: &str, action_type: &str, goal: RuntimeValue) -> RuntimeValue;
    fn published(&self) -> Vec<AdapterMessage>;
}

// ── Shared stub internals ─────────────────────────────────────────────────────

#[derive(Debug, Default)]
struct StubTransportState {
    connected: bool,
    config: TransportConfig,
    subscriptions: HashMap<String, VecDeque<RuntimeValue>>,
    published: Vec<AdapterMessage>,
}

impl StubTransportState {
    fn publish(&mut self, topic: &str, message_type: &str, value: RuntimeValue) {
        // Publish.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic` — input value
        // - `message_type` — input value
        // - `value` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.publish(topic, message_type, value);

        self.published.push(AdapterMessage {
            topic: topic.to_string(),
            message_type: message_type.to_string(),
            value: value.clone(),
        });
        if let Some(buf) = self.subscriptions.get_mut(topic) {
            buf.push_back(value);
        }
    }

    fn subscribe(&mut self, topic: &str) {
        // Subscribe.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.subscribe(topic);

        self.subscriptions.entry(topic.to_string()).or_default();
    }

    fn receive(&mut self, topic: &str) -> Option<RuntimeValue> {
        // Receive.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic` — input value
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.receive(topic);

        self.subscriptions
            .get_mut(topic)
            .and_then(|q| q.pop_front())
    }

    fn service_result(service_type: &str) -> RuntimeValue {
        // Service result.
        //
        // Parameters:
        // - `service_type` — input value
        //
        // Returns:
        // RuntimeValue.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::transport::service_result(service_type);

        RuntimeValue::Object {
            type_name: service_type.to_string(),
            fields: HashMap::from([("ok".into(), RuntimeValue::Bool { value: true })]),
        }
    }

    fn action_result(action_type: &str) -> RuntimeValue {
        // Action result.
        //
        // Parameters:
        // - `action_type` — input value
        //
        // Returns:
        // RuntimeValue.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::transport::action_result(action_type);

        RuntimeValue::Object {
            type_name: action_type.to_string(),
            fields: HashMap::from([("success".into(), RuntimeValue::Bool { value: true })]),
        }
    }
}

macro_rules! stub_adapter {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Default)]
        pub struct $name {
            state: StubTransportState,
        }

        impl TransportAdapter for $name {
            fn kind(&self) -> TransportKind {
                // Kind.
                //
                // Parameters:
                // - `self` — method receiver
                //
                // Returns:
                // TransportKind.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.kind();

                $kind
            }

            fn connect(&mut self, config: &TransportConfig) -> Result<(), String> {
                // Connect.
                //
                // Parameters:
                // - `self` — method receiver
                // - `config` — input value
                //
                // Returns:
                // Success value on completion, or an error.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.connect(config);

                self.state.connected = true;
                self.state.config = config.clone();
                Ok(())
            }

            fn disconnect(&mut self) {
                // Disconnect.
                //
                // Parameters:
                // - `self` — method receiver
                //
                // Returns:
                // Nothing.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.disconnect();

                self.state.connected = false;
            }

            fn is_connected(&self) -> bool {
                // Return whether connected.
                //
                // Parameters:
                // - `self` — method receiver
                //
                // Returns:
                // true or false.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.is_connected();

                self.state.connected
            }

            fn publish(&mut self, topic: &str, message_type: &str, value: RuntimeValue) {
                // Publish.
                //
                // Parameters:
                // - `self` — method receiver
                // - `topic` — input value
                // - `message_type` — input value
                // - `value` — input value
                //
                // Returns:
                // Nothing.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.publish(topic, message_type, value);

                if self.state.connected {
                    self.state.publish(topic, message_type, value);
                }
            }

            fn subscribe(&mut self, topic: &str) {
                // Subscribe.
                //
                // Parameters:
                // - `self` — method receiver
                // - `topic` — input value
                //
                // Returns:
                // Nothing.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.subscribe(topic);

                if self.state.connected {
                    self.state.subscribe(topic);
                }
            }

            fn receive(&mut self, topic: &str) -> Option<RuntimeValue> {
                // Receive.
                //
                // Parameters:
                // - `self` — method receiver
                // - `topic` — input value
                //
                // Returns:
                // Some value on success, otherwise none.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.receive(topic);

                if self.state.connected {
                    self.state.receive(topic)
                } else {
                    None
                }
            }

            fn call_service(
                &mut self,
                _service: &str,
                service_type: &str,
                _request: Option<RuntimeValue>,
            ) -> RuntimeValue {
                // Call service.
                //
                // Parameters:
                // - `self` — method receiver
                // - `_service` — input value
                // - `service_type` — input value
                // - `_request` — input value
                //
                // Returns:
                // RuntimeValue.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.call_service(_service, service_type, _request);

                StubTransportState::service_result(service_type)
            }

            fn send_action(
                &mut self,
                _action: &str,
                action_type: &str,
                _goal: RuntimeValue,
            ) -> RuntimeValue {
                // Send action.
                //
                // Parameters:
                // - `self` — method receiver
                // - `_action` — input value
                // - `action_type` — input value
                // - `_goal` — input value
                //
                // Returns:
                // RuntimeValue.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.send_action(_action, action_type, _goal);

                StubTransportState::action_result(action_type)
            }

            fn published(&self) -> Vec<AdapterMessage> {
                // Published.
                //
                // Parameters:
                // - `self` — method receiver
                //
                // Returns:
                // Vec<AdapterMessage>.
                //
                // Options:
                // None.
                //
                // Example:
                // let result = instance.published();

                self.state.published.clone()
            }
        }
    };
}

/// ROS2 transport adapter — logs locally; optionally forwards via Python bridge.
#[derive(Debug, Default)]
pub struct Ros2TransportAdapter {
    state: StubTransportState,
}

impl TransportAdapter for Ros2TransportAdapter {
    fn kind(&self) -> TransportKind {
        // Kind.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // TransportKind.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.kind();

        TransportKind::Ros2
    }

    fn connect(&mut self, config: &TransportConfig) -> Result<(), String> {
        // Connect.
        //
        // Parameters:
        // - `self` — method receiver
        // - `config` — input value
        //
        // Returns:
        // Success value on completion, or an error.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.connect(config);

        self.state.connected = true;
        self.state.config = config.clone();
        Ok(())
    }

    fn disconnect(&mut self) {
        // Disconnect.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.disconnect();

        self.state.connected = false;
    }

    fn is_connected(&self) -> bool {
        // Return whether connected.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // true or false.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.is_connected();

        self.state.connected
    }

    fn publish(&mut self, topic: &str, message_type: &str, value: RuntimeValue) {
        // Publish.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic` — input value
        // - `message_type` — input value
        // - `value` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.publish(topic, message_type, value);

        if self.state.connected {
            self.state.publish(topic, message_type, value.clone());
        }
        if rclrs::try_rclrs_publish(topic, &value) {
            return;
        }
        let _ = live::try_ros2_publish(topic, &value);
    }

    fn subscribe(&mut self, topic: &str) {
        // Subscribe.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.subscribe(topic);

        if self.state.connected {
            self.state.subscribe(topic);
        }
        if rclrs::try_rclrs_subscribe(topic) {
            return;
        }
        let _ = live::try_ros2_subscribe(topic);
    }

    fn receive(&mut self, topic: &str) -> Option<RuntimeValue> {
        // Receive.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic` — input value
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.receive(topic);

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
        // Call service.
        //
        // Parameters:
        // - `self` — method receiver
        // - `service` — input value
        // - `service_type` — input value
        // - `request` — input value
        //
        // Returns:
        // RuntimeValue.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.call_service(service, service_type, request);

        let request_text = request
            .as_ref()
            .map(payload_string_for_service)
            .unwrap_or_else(|| "{}".into());
        if rclrs::try_rclrs_service_call(service, service_type, &request_text) {
            return StubTransportState::service_result(service_type);
        }
        let _ = live::try_ros2_service_call(service, service_type, &request_text);
        StubTransportState::service_result(service_type)
    }

    fn send_action(
        &mut self,
        _action: &str,
        action_type: &str,
        _goal: RuntimeValue,
    ) -> RuntimeValue {
        // Send action.
        //
        // Parameters:
        // - `self` — method receiver
        // - `_action` — input value
        // - `action_type` — input value
        // - `_goal` — input value
        //
        // Returns:
        // RuntimeValue.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.send_action(_action, action_type, _goal);

        StubTransportState::action_result(action_type)
    }

    fn published(&self) -> Vec<AdapterMessage> {
        // Published.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Vec<AdapterMessage>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.published();

        self.state.published.clone()
    }
}

stub_adapter!(MqttTransportAdapter, TransportKind::Mqtt);
stub_adapter!(DdsTransportAdapter, TransportKind::Dds);
stub_adapter!(WebsocketTransportAdapter, TransportKind::Websocket);

// ── Routing comm bus ──────────────────────────────────────────────────────────
/// Routes publish/subscribe/service/action calls to transport-specific adapters
/// while preserving in-memory semantics for simulation and discovery.
#[derive(Debug)]
pub struct RoutingCommBus {
    memory: InMemoryCommBus,
    ros2: Ros2TransportAdapter,
    mqtt: MqttTransportAdapter,
    dds: DdsTransportAdapter,
    websocket: WebsocketTransportAdapter,
    config: TransportConfig,
}

impl Default for RoutingCommBus {
    fn default() -> Self {
        // Return the default value.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // A new instance of this type.
        //
        // Options:
        // None.
        //
        // Example:
        // let value = spanda_core::transport::default();

        Self::new()
    }
}

impl RoutingCommBus {
    pub fn new() -> Self {
        // Create a new instance.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // A new instance of this type.
        //
        // Options:
        // None.
        //
        // Example:
        // let value = spanda_core::transport::new();

        Self {
            memory: InMemoryCommBus::new(),
            ros2: Ros2TransportAdapter::default(),
            mqtt: MqttTransportAdapter::default(),
            dds: DdsTransportAdapter::default(),
            websocket: WebsocketTransportAdapter::default(),
            config: TransportConfig::default(),
        }
    }

    pub fn configure(&mut self, config: TransportConfig) {
        // Configure.
        //
        // Parameters:
        // - `self` — method receiver
        // - `config` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.configure(config);

        self.config = config.clone();
        let _ = self.ros2.connect(&config);
        let _ = self.mqtt.connect(&TransportConfig {
            broker_url: config
                .broker_url
                .clone()
                .or(Some("mqtt://localhost:1883".into())),
            client_id: config.client_id.clone().or(Some("spanda".into())),
            ..config.clone()
        });
        let _ = self.dds.connect(&TransportConfig {
            domain_id: config.domain_id.or(Some(0)),
            ..config.clone()
        });
        let _ = self.websocket.connect(&TransportConfig {
            broker_url: config
                .broker_url
                .clone()
                .or(Some("ws://localhost:9090".into())),
            ..config
        });
    }

    pub fn adapter(&self, kind: TransportKind) -> Option<&dyn TransportAdapter> {
        // Adapter.
        //
        // Parameters:
        // - `self` — method receiver
        // - `kind` — input value
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.adapter(kind);

        match kind {
            TransportKind::Ros2 => Some(&self.ros2),
            TransportKind::Mqtt => Some(&self.mqtt),
            TransportKind::Dds => Some(&self.dds),
            TransportKind::Websocket => Some(&self.websocket),
            TransportKind::Local | TransportKind::Sim => None,
        }
    }

    pub fn adapter_mut(&mut self, kind: TransportKind) -> Option<&mut dyn TransportAdapter> {
        // Adapter mut.
        //
        // Parameters:
        // - `self` — method receiver
        // - `kind` — input value
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.adapter_mut(kind);

        match kind {
            TransportKind::Ros2 => Some(&mut self.ros2),
            TransportKind::Mqtt => Some(&mut self.mqtt),
            TransportKind::Dds => Some(&mut self.dds),
            TransportKind::Websocket => Some(&mut self.websocket),
            TransportKind::Local | TransportKind::Sim => None,
        }
    }

    pub fn memory(&self) -> &InMemoryCommBus {
        // Memory.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // &InMemoryCommBus.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.memory();

        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut InMemoryCommBus {
        // Memory mut.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // &mut InMemoryCommBus.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.memory_mut();

        &mut self.memory
    }

    pub fn register_robot(&mut self, name: impl Into<String>) {
        // Register robot.
        //
        // Parameters:
        // - `self` — method receiver
        // - `name` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.register_robot(name);

        self.memory.register_robot(name);
    }

    pub fn register_agent(&mut self, name: impl Into<String>) {
        // Register agent.
        //
        // Parameters:
        // - `self` — method receiver
        // - `name` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.register_agent(name);

        self.memory.register_agent(name);
    }

    pub fn register_device(&mut self, name: impl Into<String>) {
        // Register device.
        //
        // Parameters:
        // - `self` — method receiver
        // - `name` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.register_device(name);

        self.memory.register_device(name);
    }

    pub fn publish_peer(
        &mut self,
        peer: &str,
        topic: &str,
        value: RuntimeValue,
        transport: TransportKind,
    ) {
        // Publish peer.
        //
        // Parameters:
        // - `self` — method receiver
        // - `peer` — input value
        // - `topic` — input value
        // - `value` — input value
        // - `transport` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.publish_peer(peer, topic, value, transport);

        self.memory.publish_peer(peer, topic, value, transport);
    }

    /// Poll external transport adapters for inbound messages on subscribed topics.
    pub fn poll_inbound(&mut self, transport: TransportKind) -> Vec<(String, RuntimeValue)> {
        // Poll inbound.
        //
        // Parameters:
        // - `self` — method receiver
        // - `transport` — input value
        //
        // Returns:
        // Vec<(String, RuntimeValue)>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.poll_inbound(transport);

        let paths = self.memory.subscription_paths();
        let mut inbound = Vec::new();
        let kinds = [
            transport,
            TransportKind::Ros2,
            TransportKind::Mqtt,
            TransportKind::Dds,
            TransportKind::Websocket,
        ];
        for path in paths {
            for kind in kinds {
                if let Some(adapter) = self.adapter_mut(kind) {
                    if adapter.is_connected() {
                        if let Some(value) = adapter.receive(&path) {
                            self.memory.push_inbound(&path, value.clone());
                            inbound.push((path.clone(), value));
                        }
                    }
                }
            }
        }
        inbound
    }
}

impl CommBus for RoutingCommBus {
    fn publish(
        &mut self,
        topic_path: &str,
        message_type: &str,
        value: RuntimeValue,
        transport: TransportKind,
    ) {
        // Publish.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic_path` — input value
        // - `message_type` — input value
        // - `value` — input value
        // - `transport` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.publish(topic_path, message_type, value, transport);

        self.memory
            .publish(topic_path, message_type, value.clone(), transport);
        if let Some(adapter) = self.adapter_mut(transport) {
            adapter.publish(topic_path, message_type, value);
        }
    }

    fn subscribe(&mut self, topic_path: &str, handler: &str) {
        // Subscribe.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic_path` — input value
        // - `handler` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.subscribe(topic_path, handler);

        self.memory.subscribe(topic_path, handler);
    }

    fn receive(&mut self, topic_path: &str) -> Option<RuntimeValue> {
        // Receive.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic_path` — input value
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.receive(topic_path);

        self.memory.receive(topic_path)
    }

    fn call_service(
        &mut self,
        service_name: &str,
        service_type: &str,
        request: Option<RuntimeValue>,
    ) -> RuntimeValue {
        // Call service.
        //
        // Parameters:
        // - `self` — method receiver
        // - `service_name` — input value
        // - `service_type` — input value
        // - `request` — input value
        //
        // Returns:
        // RuntimeValue.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.call_service(service_name, service_type, request);

        self.memory
            .call_service(service_name, service_type, request.clone())
    }

    fn send_action(
        &mut self,
        action_name: &str,
        action_type: &str,
        goal: RuntimeValue,
    ) -> RuntimeValue {
        // Send action.
        //
        // Parameters:
        // - `self` — method receiver
        // - `action_name` — input value
        // - `action_type` — input value
        // - `goal` — input value
        //
        // Returns:
        // RuntimeValue.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.send_action(action_name, action_type, goal);

        self.memory.send_action(action_name, action_type, goal)
    }

    fn discover(&self, target: DiscoverTarget, filter: &DiscoverFilter) -> Vec<String> {
        // Discover.
        //
        // Parameters:
        // - `self` — method receiver
        // - `target` — input value
        // - `filter` — input value
        //
        // Returns:
        // Vec<String>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.discover(target, filter);

        self.memory.discover(target, filter)
    }

    fn published_messages(&self) -> Vec<PublishedCommMessage> {
        // Published messages.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Vec<PublishedCommMessage>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.published_messages();

        self.memory.published_messages()
    }

    fn inject_fault(&mut self, fault: &str) {
        // Inject fault.
        //
        // Parameters:
        // - `self` — method receiver
        // - `fault` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.inject_fault(fault);

        self.memory.inject_fault(fault);
    }

    fn set_network_config(&mut self, config: SimNetworkConfig) {
        // Set network config.
        //
        // Parameters:
        // - `self` — method receiver
        // - `config` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.set_network_config(config);

        self.memory.set_network_config(config);
    }

    fn active_faults(&self) -> Vec<String> {
        // Active faults.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Vec<String>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.active_faults();

        self.memory.active_faults()
    }

    fn subscription_paths(&self) -> Vec<String> {
        // Subscription paths.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Vec<String>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.subscription_paths();

        self.memory.subscription_paths()
    }

    fn push_inbound(&mut self, topic_path: &str, value: RuntimeValue) {
        // Push inbound.
        //
        // Parameters:
        // - `self` — method receiver
        // - `topic_path` — input value
        // - `value` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.push_inbound(topic_path, value);

        self.memory.push_inbound(topic_path, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ros2_adapter_publish_when_connected() {
        // Ros2 adapter publish when connected.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::transport::ros2_adapter_publish_when_connected();

        let mut adapter = Ros2TransportAdapter::default();
        assert!(!adapter.is_connected());
        adapter
            .connect(&TransportConfig {
                node_name: Some("spanda".into()),
                ..Default::default()
            })
            .unwrap();
        adapter.publish("/scan", "Scan", RuntimeValue::Bool { value: true });
        assert_eq!(adapter.published().len(), 1);
        assert_eq!(adapter.published()[0].topic, "/scan");
    }

    #[test]
    fn routing_bus_delegates_ros2_publish() {
        // Routing bus delegates ros2 publish.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::transport::routing_bus_delegates_ros2_publish();

        let mut bus = RoutingCommBus::new();
        bus.configure(TransportConfig {
            node_name: Some("bot".into()),
            ..Default::default()
        });
        bus.publish(
            "/cmd_vel",
            "Velocity",
            RuntimeValue::Bool { value: true },
            TransportKind::Ros2,
        );
        assert_eq!(bus.published_messages().len(), 1);
        assert_eq!(bus.ros2.published().len(), 1);
    }

    #[test]
    fn sim_transport_stays_in_memory_only() {
        // Sim transport stays in memory only.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_core::transport::sim_transport_stays_in_memory_only();

        let mut bus = RoutingCommBus::new();
        bus.publish(
            "/local",
            "String",
            RuntimeValue::Bool { value: true },
            TransportKind::Sim,
        );
        assert_eq!(bus.published_messages().len(), 1);
        assert!(bus.ros2.published().is_empty());
    }
}
