//! RuntimeValue live transport bridges for MQTT, DDS, and WebSocket backends.
//!
use spanda_runtime::value::RuntimeValue;

pub use spanda_transport_mqtt::LiveMqttBridge as MqttLiveBridge;

/// Live MQTT bridge with Spanda runtime value conversion.
#[derive(Debug, Default)]
pub struct LiveMqttBridge {
    inner: spanda_transport_mqtt::LiveMqttBridge,
}

impl LiveMqttBridge {
    pub fn connect(broker_url: &str, client_id: &str) -> Result<Self, String> {
        // Description:
        //     Connect.
        //
        // Inputs:
        //     broker_url: &str
        //         Caller-supplied broker url.
        //     client_id: &str
        //         Caller-supplied client id.
        //
        // Outputs:
        //     result: Result<Self, String>
        //         Return value from `connect`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::connect(broker_url, client_id);

        Ok(Self {
            inner: spanda_transport_mqtt::LiveMqttBridge::connect(broker_url, client_id)?,
        })
    }

    pub fn publish(&self, topic: &str, payload: &str) -> Result<(), String> {
        // Description:
        //     Publish.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic: &str
        //         Caller-supplied opic.
        //     payload: &str
        //         Caller-supplied payload.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `publish`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::publish(&self, opic, payload);

        self.inner.publish(topic, payload)
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), String> {
        // Description:
        //     Subscribe.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `subscribe`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::subscribe(&self, opic);

        self.inner.subscribe(topic)
    }

    pub fn receive(&self, topic: &str) -> Option<RuntimeValue> {
        // Description:
        //     Receive.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     result: Option<RuntimeValue>
        //         Return value from `receive`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::receive(&self, opic);

        self.inner
            .receive(topic)
            .map(|value| RuntimeValue::String { value })
    }
}

/// Live DDS bridge with Spanda runtime value conversion.
#[derive(Debug, Default)]
pub struct LiveDdsBridge {
    inner: spanda_transport_dds::LiveDdsBridge,
}

impl LiveDdsBridge {
    pub fn connect(domain_id: u32) -> Result<Self, String> {
        // Description:
        //     Connect.
        //
        // Inputs:
        //     domain_id: u32
        //         Caller-supplied domain id.
        //
        // Outputs:
        //     result: Result<Self, String>
        //         Return value from `connect`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::connect(domain_id);

        Ok(Self {
            inner: spanda_transport_dds::LiveDdsBridge::connect(domain_id)?,
        })
    }

    pub fn publish(&self, topic: &str, payload: &str) -> Result<(), String> {
        // Description:
        //     Publish.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic: &str
        //         Caller-supplied opic.
        //     payload: &str
        //         Caller-supplied payload.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `publish`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::publish(&self, opic, payload);

        self.inner.publish(topic, payload)
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), String> {
        // Description:
        //     Subscribe.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `subscribe`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::subscribe(&self, opic);

        self.inner.subscribe(topic)
    }

    pub fn receive(&self, topic: &str) -> Option<RuntimeValue> {
        // Description:
        //     Receive.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     result: Option<RuntimeValue>
        //         Return value from `receive`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::receive(&self, opic);

        self.inner
            .receive(topic)
            .map(|value| RuntimeValue::String { value })
    }
}

/// Live WebSocket bridge with Spanda runtime value conversion.
#[derive(Debug, Default)]
pub struct LiveWebsocketBridge {
    inner: spanda_transport_websocket::LiveWebsocketBridge,
}

impl LiveWebsocketBridge {
    pub fn connect(broker_url: &str) -> Result<Self, String> {
        // Description:
        //     Connect.
        //
        // Inputs:
        //     broker_url: &str
        //         Caller-supplied broker url.
        //
        // Outputs:
        //     result: Result<Self, String>
        //         Return value from `connect`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::connect(broker_url);

        Ok(Self {
            inner: spanda_transport_websocket::LiveWebsocketBridge::connect(broker_url)?,
        })
    }

    pub fn publish(&self, topic: &str, payload: &str) -> Result<(), String> {
        // Description:
        //     Publish.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic: &str
        //         Caller-supplied opic.
        //     payload: &str
        //         Caller-supplied payload.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `publish`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::publish(&self, opic, payload);

        self.inner.publish(topic, payload)
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), String> {
        // Description:
        //     Subscribe.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `subscribe`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::subscribe(&self, opic);

        self.inner.subscribe(topic)
    }

    pub fn receive(&self, topic: &str) -> Option<RuntimeValue> {
        // Description:
        //     Receive.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic: &str
        //         Caller-supplied opic.
        //
        // Outputs:
        //     result: Option<RuntimeValue>
        //         Return value from `receive`.
        //
        // Example:

        //     let result = spanda_transport_routing::live_bridges::receive(&self, opic);

        self.inner
            .receive(topic)
            .map(|value| RuntimeValue::String { value })
    }
}
