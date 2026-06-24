//! Live MQTT broker integration via rumqttc.
//!
use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct LiveMqttBridge {
    client: Client,
    inbound: Arc<Mutex<HashMap<String, VecDeque<String>>>>,
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

        //     let result = spanda_transport_mqtt::live::connect(broker_url, client_id);

        let (host, port) = parse_broker_url(broker_url)?;
        let mut options = MqttOptions::new(client_id, host, port);
        options.set_keep_alive(Duration::from_secs(5));
        let (client, mut connection) = Client::new(options, 10);
        let inbound: Arc<Mutex<HashMap<String, VecDeque<String>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let inbound_poll = Arc::clone(&inbound);
        thread::spawn(move || loop {
            match connection.iter().next() {
                Some(Ok(Event::Incoming(Incoming::Publish(packet)))) => {
                    let payload = String::from_utf8_lossy(&packet.payload).to_string();
                    if let Ok(mut map) = inbound_poll.lock() {
                        map.entry(packet.topic).or_default().push_back(payload);
                    }
                }
                Some(Ok(_)) => {}
                Some(Err(e)) => {
                    eprintln!("live mqtt connection error: {e}");
                    break;
                }
                None => break,
            }
        });
        Ok(Self { client, inbound })
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

        //     let result = spanda_transport_mqtt::live::publish(&self, opic, payload);

        self.client
            .publish(topic, QoS::AtMostOnce, false, payload.as_bytes())
            .map_err(|e| format!("mqtt publish failed: {e}"))
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

        //     let result = spanda_transport_mqtt::live::subscribe(&self, opic);

        self.client
            .subscribe(topic, QoS::AtMostOnce)
            .map_err(|e| format!("mqtt subscribe failed: {e}"))
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

        //     let result = spanda_transport_mqtt::live::receive(&self, opic);

        let mut map = self.inbound.lock().ok()?;
        map.get_mut(topic).and_then(|q| q.pop_front())
    }
}

fn parse_broker_url(url: &str) -> Result<(String, u16), String> {
    // Description:
    //     Parse broker url.
    //
    // Inputs:
    //     url: &str
    //         Caller-supplied url.
    //
    // Outputs:
    //     result: Result<(String, u16), String>
    //         Return value from `parse_broker_url`.
    //
    // Example:

    //     let result = spanda_transport_mqtt::live::parse_broker_url(rl);

    let stripped = url
        .trim_start_matches("mqtts://")
        .trim_start_matches("mqtt://")
        .trim_start_matches("ssl://");
    let (host, port) = stripped
        .split_once(':')
        .map(|(h, p)| (h.to_string(), p.parse().unwrap_or(1883)))
        .unwrap_or((stripped.to_string(), 1883));
    Ok((host, port))
}
