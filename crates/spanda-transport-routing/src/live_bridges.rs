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
        Ok(Self {
            inner: spanda_transport_mqtt::LiveMqttBridge::connect(broker_url, client_id)?,
        })
    }

    pub fn publish(&self, topic: &str, payload: &str) -> Result<(), String> {
        self.inner.publish(topic, payload)
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), String> {
        self.inner.subscribe(topic)
    }

    pub fn receive(&self, topic: &str) -> Option<RuntimeValue> {
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
        Ok(Self {
            inner: spanda_transport_dds::LiveDdsBridge::connect(domain_id)?,
        })
    }

    pub fn publish(&self, topic: &str, payload: &str) -> Result<(), String> {
        self.inner.publish(topic, payload)
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), String> {
        self.inner.subscribe(topic)
    }

    pub fn receive(&self, topic: &str) -> Option<RuntimeValue> {
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
        Ok(Self {
            inner: spanda_transport_websocket::LiveWebsocketBridge::connect(broker_url)?,
        })
    }

    pub fn publish(&self, topic: &str, payload: &str) -> Result<(), String> {
        self.inner.publish(topic, payload)
    }

    pub fn subscribe(&self, topic: &str) -> Result<(), String> {
        self.inner.subscribe(topic)
    }

    pub fn receive(&self, topic: &str) -> Option<RuntimeValue> {
        self.inner
            .receive(topic)
            .map(|value| RuntimeValue::String { value })
    }
}
