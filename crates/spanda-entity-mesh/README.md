# spanda-entity-mesh

**Spanda Autonomous Entity Mesh** — trust-aware inter-entity communication and resilience layer.

Sits **above** transport providers (MQTT, DDS, ROS2, BLE, Wi-Fi, etc.). Does **not** implement packet routing or replace existing transports.

## Capabilities

- Entity and capability discovery (from `EntityRegistry` + entity graph)
- Trust-aware and readiness-aware routing
- Partition detection, offline policies, merge
- Coordinator election (communication role only)
- Mission delegation hooks (via Recovery Orchestrator)
- Secure messaging integration (`SignedMessage`, nonce/TTL)
- Readiness, assurance, diagnosis, and recovery integration

## Usage

```rust
use spanda_config::build_entity_registry;
use spanda_entity_mesh::{build_entity_mesh, evaluate_mesh_health, find_capability};

let registry = build_entity_registry(&resolved);
let mesh = build_entity_mesh(&registry, "my-mesh");
let health = evaluate_mesh_health(&mesh, &Default::default());
let thermal = find_capability(&mesh, "thermal_camera");
```

## CLI / API

- CLI: `spanda mesh discover|list|health|route|find|…`
- REST: `GET /v1/mesh/topology`, `GET /v1/mesh/health`, …
- Docs: [docs/entity-mesh.md](../docs/entity-mesh.md)

## Status

**Experimental** — see [docs/feature-status.md](../docs/feature-status.md).

## Distinction from fleet mesh

`spanda fleet mesh start` runs the **HTTP fleet relay coordinator** in `spanda-fleet`. Entity Mesh is entity-level topology, trust routing, and partition resilience.
