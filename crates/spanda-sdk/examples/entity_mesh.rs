//! Entity mesh helpers — used by `scripts/entity_mesh_smoke.sh`.
//!
//! Requires Control Center with warehouse fixture (started by the smoke script).

use spanda_sdk::SpandaClient;

fn main() {
    let client = SpandaClient::local();

    let health = client.mesh_health().expect("mesh health");
    if health.get("health").is_none() {
        panic!("mesh health missing");
    }

    let nodes = client.mesh_nodes().expect("mesh nodes");
    if nodes.get("nodes").is_none() {
        panic!("mesh nodes missing");
    }

    let topology = client.mesh_topology().expect("mesh topology");
    if topology.get("topology").is_none() {
        panic!("mesh topology missing");
    }

    println!("rust-sdk mesh smoke ok");
}
