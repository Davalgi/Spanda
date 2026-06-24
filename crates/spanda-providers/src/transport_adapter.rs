//! Wrap legacy `spanda-transport` adapters as runtime `TransportProvider` implementations.
//!
use spanda_runtime::providers::{
    traits::TransportProvider,
    transport_types::{
        AdapterMessage as RuntimeAdapterMessage, TransportConfig as RuntimeTransportConfig,
    },
    types::{ProviderId, ProviderMetadata, ProviderSafetyLevel},
};
use spanda_runtime::value::RuntimeValue;
use spanda_transport::{AdapterMessage, TransportAdapter, TransportConfig};

/// Map a full transport adapter config onto the lean provider contract.
pub fn adapter_config_to_runtime(config: &TransportConfig) -> RuntimeTransportConfig {
    // Description:
    //     Adapter config to runtime.
    //
    // Inputs:
    //     config: &TransportConfig
    //         Caller-supplied config.
    //
    // Outputs:
    //     result: RuntimeTransportConfig
    //         Return value from `adapter_config_to_runtime`.
    //
    // Example:

    //     let result = spanda_providers::transport_adapter::adapter_config_to_runtime(config);

    RuntimeTransportConfig {
        broker_url: config.broker_url.clone(),
        node_name: config.node_name.clone(),
        namespace: config.namespace.clone(),
        domain_id: config.domain_id,
        client_id: config.client_id.clone(),
    }
}

fn runtime_config_to_adapter(config: &RuntimeTransportConfig) -> TransportConfig {
    // Description:

    //     Runtime config to adapter.

    //

    // Inputs:

    //     config: &RuntimeTransportConfig

    //         Caller-supplied config.

    //

    // Outputs:

    //     result: TransportConfig

    //         Return value from `runtime_config_to_adapter`.

    //

    // Example:

    //     let result = spanda_providers::transport_adapter::runtime_config_to_adapter(config);
    TransportConfig {
        broker_url: config.broker_url.clone(),
        node_name: config.node_name.clone(),
        namespace: config.namespace.clone(),
        domain_id: config.domain_id,
        client_id: config.client_id.clone(),
        ..TransportConfig::default()
    }
}

fn adapter_message_to_runtime(message: AdapterMessage) -> RuntimeAdapterMessage {
    // Description:
    //     Adapter message to runtime.
    //
    // Inputs:
    //     essage: AdapterMessage
    //         Caller-supplied essage.
    //
    // Outputs:
    //     result: RuntimeAdapterMessage
    //         Return value from `adapter_message_to_runtime`.
    //
    // Example:

    //     let result = spanda_providers::transport_adapter::adapter_message_to_runtime(essage);

    RuntimeAdapterMessage {
        topic: message.topic,
        message_type: message.message_type,
        value: message.value,
    }
}

/// Blanket adapter: wrap an existing `TransportAdapter` as a `TransportProvider`.
pub struct TransportAdapterProvider<T: TransportAdapter> {
    id: ProviderId,
    inner: T,
}

impl<T: TransportAdapter> TransportAdapterProvider<T> {
    pub fn new(package: impl Into<String>, name: impl Into<String>, inner: T) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     package: impl Into<String>
        //         Caller-supplied package.
        //     name: impl Into<String>
        //         Caller-supplied name.
        //     inner: T
        //         Caller-supplied inner.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:

        //     let value = spanda_providers::transport_adapter::new(package, name, inner);

        Self {
            id: ProviderId::new(package, name),
            inner,
        }
    }

    pub fn into_inner(self) -> T {
        // Description:
        //     Into inner.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: T
        //         Return value from `into_inner`.
        //
        // Example:

        //     let result = instance.into_inner();

        self.inner
    }

    pub fn inner(&self) -> &T {
        // Description:
        //     Inner.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &T
        //         Return value from `inner`.
        //
        // Example:

        //     let result = spanda_providers::transport_adapter::inner(&self);

        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        // Description:
        //     Inner mut.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     result: &mut T
        //         Return value from `inner_mut`.
        //
        // Example:

        //     let result = spanda_providers::transport_adapter::inner_mut(&mut self);

        &mut self.inner
    }
}

impl<T: TransportAdapter + Send + Sync> TransportProvider for TransportAdapterProvider<T> {
    fn metadata(&self) -> ProviderMetadata {
        // Description:
        //     Metadata.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: ProviderMetadata
        //         Return value from `metadata`.
        //
        // Example:

        //     let result = spanda_providers::transport_adapter::metadata(&self);

        ProviderMetadata {
            id: self.id.clone(),
            description: format!("Transport adapter ({:?})", self.inner.kind()),
            safety_level: ProviderSafetyLevel::Development,
            capabilities_required: vec![],
            hardware_requirements: vec![],
        }
    }

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

        //     let result = spanda_providers::transport_adapter::kind(&self);

        self.inner.kind()
    }

    fn connect(&mut self, config: &RuntimeTransportConfig) -> Result<(), String> {
        // Description:
        //     Connect.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     config: &RuntimeTransportConfig
        //         Caller-supplied config.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `connect`.
        //
        // Example:

        //     let result = spanda_providers::transport_adapter::connect(&mut self, config);

        self.inner.connect(&runtime_config_to_adapter(config))
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

        //     let result = spanda_providers::transport_adapter::disconnect(&mut self);

        self.inner.disconnect();
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

        //     let result = spanda_providers::transport_adapter::is_connected(&self);

        self.inner.is_connected()
    }

    fn publish(&mut self, topic: &str, message_type: &str, value: RuntimeValue) {
        // Description:
        //     Publish.
        //
        // Inputs:
        //     &mut self: input value
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

        //     let result = spanda_providers::transport_adapter::publish(&mut self, opic, essage_type, value);

        self.inner.publish(topic, message_type, value);
    }

    fn subscribe(&mut self, topic: &str) {
        // Description:
        //     Subscribe.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_providers::transport_adapter::subscribe(&mut self, opic);

        self.inner.subscribe(topic);
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

        //     let result = spanda_providers::transport_adapter::receive(&mut self, opic);

        self.inner.receive(topic)
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

        //     let result = spanda_providers::transport_adapter::call_service(&mut self, service, service_type, reques);

        self.inner.call_service(service, service_type, request)
    }

    fn send_action(&mut self, action: &str, action_type: &str, goal: RuntimeValue) -> RuntimeValue {
        // Description:
        //     Send action.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     action: &str
        //         Caller-supplied action.
        //     action_type: &str
        //         Caller-supplied action type.
        //     goal: RuntimeValue
        //         Caller-supplied goal.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `send_action`.
        //
        // Example:

        //     let result = spanda_providers::transport_adapter::send_action(&mut self, action, action_type, goal);

        self.inner.send_action(action, action_type, goal)
    }

    fn published(&self) -> Vec<RuntimeAdapterMessage> {
        // Description:
        //     Published.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<RuntimeAdapterMessage>
        //         Return value from `published`.
        //
        // Example:

        //     let result = spanda_providers::transport_adapter::published(&self);

        self.inner
            .published()
            .into_iter()
            .map(adapter_message_to_runtime)
            .collect()
    }
}
