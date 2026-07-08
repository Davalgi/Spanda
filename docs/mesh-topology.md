# Mesh Topology

Entity Mesh topology is a **projection of the Entity Graph** plus live link metrics — not a separate
disconnected graph.

## Core types

| Type | Role |
|------|------|
| `EntityMesh` | Root mesh state container |
| `MeshNode` | Per-entity reachability, trust, capabilities, neighbors |
| `MeshNeighbor` | Adjacent node with transport and link metrics |
| `MeshLink` | Directed link with latency, loss, trust |
| `MeshTopology` | Snapshot: nodes, links, coordinator, partitions |

## Node state

Each `MeshNode` tracks: `entity_id`, `node_id`, `transport`, `reachable`, `neighbors`,
`capabilities`, `health`, `readiness`, `trust_score`, `latency`, `bandwidth`, `packet_loss`,
`hop_count`, `last_seen`, `battery`, `role`, `coordinator_status`, `supported_protocols`,
`security_identity`.

## Discovery sources

- Local runtime (entity registry)
- Entity graph relationships
- DDS, ROS2, MQTT, mDNS, BLE, Wi-Fi subnet, manual config (via transport providers)

## CLI

```bash
spanda mesh topology
spanda mesh graph --json
```

## API

`GET /v1/mesh/topology` · `GET /v1/mesh/graph`
