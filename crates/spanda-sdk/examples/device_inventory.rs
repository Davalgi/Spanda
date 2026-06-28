//! List device entities from Control Center via the official SDK.
//!
//!   cargo run --example device_inventory -p spanda-sdk

use spanda_sdk::SpandaClient;

fn main() {
    let client = SpandaClient::local();
    match client.list_entities() {
        Ok(entities) => {
            for entity in entities {
                println!("{} ({:?})", entity.id, entity.kind);
            }
        }
        Err(err) => eprintln!("list_entities failed: {err}"),
    }
}
