//! Live WebSocket broker integration via tungstenite.
//!
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::net::TcpStream;
use std::sync::Mutex;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};

type WsStream = WebSocket<MaybeTlsStream<TcpStream>>;

#[derive(Serialize, Deserialize)]
struct WireEnvelope {
    topic: String,
    payload: String,
}

#[derive(Debug)]
pub struct LiveWebsocketBridge {
    socket: Mutex<WsStream>,
    inbound: Mutex<HashMap<String, VecDeque<String>>>,
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

        //     let result = spanda_transport_websocket::live::connect(broker_url);

        let (socket, _response) =
            connect(broker_url).map_err(|e| format!("websocket connect failed: {e}"))?;
        Ok(Self {
            socket: Mutex::new(socket),
            inbound: Mutex::new(HashMap::new()),
        })
    }

    fn poll_inbound(&self) {
        // Description:
        //     Poll inbound.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_transport_websocket::live::poll_inbound(&self);

        let mut guard = match self.socket.lock() {
            Ok(g) => g,
            Err(_) => return,
        };

        while let Ok(Message::Text(text)) = guard.read() {
            if let Ok(frame) = serde_json::from_str::<WireEnvelope>(&text) {
                if let Ok(mut map) = self.inbound.lock() {
                    map.entry(frame.topic).or_default().push_back(frame.payload);
                }
            }
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

        //     let result = spanda_transport_websocket::live::publish(&self, opic, payload);

        self.poll_inbound();
        let envelope = WireEnvelope {
            topic: topic.to_string(),
            payload: payload.to_string(),
        };
        let text = serde_json::to_string(&envelope)
            .map_err(|e| format!("websocket serialize failed: {e}"))?;
        let mut guard = self
            .socket
            .lock()
            .map_err(|e| format!("websocket lock failed: {e}"))?;
        guard
            .send(Message::Text(text))
            .map_err(|e| format!("websocket send failed: {e}"))
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

        //     let result = spanda_transport_websocket::live::subscribe(&self, opic);

        let envelope = WireEnvelope {
            topic: topic.to_string(),
            payload: "__subscribe__".into(),
        };
        let text = serde_json::to_string(&envelope)
            .map_err(|e| format!("websocket subscribe serialize failed: {e}"))?;
        let mut guard = self
            .socket
            .lock()
            .map_err(|e| format!("websocket lock failed: {e}"))?;
        guard
            .send(Message::Text(text))
            .map_err(|e| format!("websocket subscribe failed: {e}"))
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

        //     let result = spanda_transport_websocket::live::receive(&self, opic);

        self.poll_inbound();
        let mut map = self.inbound.lock().ok()?;
        map.get_mut(topic).and_then(|q| q.pop_front())
    }
}
