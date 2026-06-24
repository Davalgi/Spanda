//! MQTT transport backend extracted from Spanda core for lean-core package architecture.
//!
//! Used by `spanda-mqtt` and wired through `spanda-core` compatibility shims until
//! all callers migrate to package-scoped provider registration.
//!
pub mod adapter;
mod python_bridge;

#[cfg(feature = "live")]
mod live;

pub use adapter::MqttTransportAdapter;

/// Live MQTT bridge handle; inactive unless built with the `live` feature.
#[derive(Default)]
pub struct LiveMqttBridge {
    #[cfg(feature = "live")]
    inner: Option<live::LiveMqttBridge>,
}

impl std::fmt::Debug for LiveMqttBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiveMqttBridge").finish_non_exhaustive()
    }
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

        //     let result = spanda_transport_mqtt::connect(broker_url, client_id);

        #[cfg(feature = "live")]
        {
            Ok(Self {
                inner: Some(live::LiveMqttBridge::connect(broker_url, client_id)?),
            })
        }
        #[cfg(not(feature = "live"))]
        {
            let _ = (broker_url, client_id);
            Err(
                "live MQTT support not enabled (build spanda-transport-mqtt with --features live)"
                    .into(),
            )
        }
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

        //     let result = spanda_transport_mqtt::publish(&self, opic, payload);

        #[cfg(feature = "live")]
        if let Some(inner) = &self.inner {
            return inner.publish(topic, payload);
        }
        let _ = (topic, payload);
        Ok(())
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

        //     let result = spanda_transport_mqtt::subscribe(&self, opic);

        #[cfg(feature = "live")]
        if let Some(inner) = &self.inner {
            return inner.subscribe(topic);
        }
        let _ = topic;
        Ok(())
    }

    pub fn receive(&self, topic: &str) -> Option<String> {
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
        //     result: Option<String>
        //         Return value from `receive`.
        //
        // Example:

        //     let result = spanda_transport_mqtt::receive(&self, opic);

        #[cfg(feature = "live")]
        if let Some(inner) = &self.inner {
            return inner.receive(topic);
        }
        let _ = topic;
        None
    }
}

pub use python_bridge::{mqtt_live_enabled, try_mqtt_publish};
