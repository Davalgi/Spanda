//! DDS transport backend extracted from Spanda core for lean-core package architecture.
//!
pub mod adapter;

#[cfg(feature = "live")]
mod live;

pub use adapter::{DdsTransportAdapter, DdsTransportAdapterLive};

/// Live DDS bridge handle over UDP multicast.
#[derive(Debug, Default)]
pub struct LiveDdsBridge {
    #[cfg(feature = "live")]
    inner: Option<live::LiveDdsBridge>,
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

        //     let result = spanda_transport_dds::connect(domain_id);

        #[cfg(feature = "live")]
        {
            return Ok(Self {
                inner: Some(live::LiveDdsBridge::connect(domain_id)?),
            });
        }
        #[cfg(not(feature = "live"))]
        {
            let _ = domain_id;
            Err(
                "live DDS support not enabled (build spanda-transport-dds with --features live)"
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

        //     let result = spanda_transport_dds::publish(&self, opic, payload);

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

        //     let result = spanda_transport_dds::subscribe(&self, opic);

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

        //     let result = spanda_transport_dds::receive(&self, opic);

        #[cfg(feature = "live")]
        if let Some(inner) = &self.inner {
            return inner.receive(topic);
        }
        let _ = topic;
        None
    }
}
