# Spanda Autonomous Entity Mesh

**Trust-aware inter-entity communication and resilience** — not packet routing.

Entity Mesh sits **above** transport providers (MQTT, DDS, ROS2, BLE, Wi-Fi, Ethernet, 5G, LoRa, satellite, local runtime) and adds entity-level capabilities:

- Entity and capability discovery
- Trust-aware and readiness-aware routing
- Topology awareness and heartbeats
- Partition detection, offline policies, and merge
- Coordinator election (communication role only)
- Mission-aware delegation (via Recovery Orchestrator)
- State synchronization (no secrets)
- Assurance and diagnosis evidence

## Layering

```
Transport (MQTT, DDS, ROS2, BLE, …)
        ↓
Mesh Transport Provider
        ↓
Spanda Entity Mesh          ← this layer
        ↓
Entity Graph (spanda-config)
        ↓
Decision / Recovery / Mission / Readiness
```

## Positioning

| Call it | Do not call it |
|---------|----------------|
| **Autonomous Entity Mesh** | "mesh networking" |
| **Spanda Entity Mesh** | packet routing |

## CLI

```bash
spanda mesh discover
spanda mesh list
spanda mesh inspect <entity-id>
spanda mesh topology
spanda mesh graph
spanda mesh health
spanda mesh route <source> <target>
spanda mesh find --capability thermal_camera
spanda mesh capabilities
spanda mesh simulate-partition <entity-id> [...]
spanda mesh merge-report
```

Use `--config crates/spanda-config/tests/fixtures/warehouse/spanda.toml` for the warehouse demo.

## API / SDK

| Method | Endpoint | gRPC RPC |
|--------|----------|----------|
| `meshTopology()` | `GET /v1/mesh/topology` | `GetMeshTopology` |
| `meshNodes()` | `GET /v1/mesh/nodes` | `GetMeshNodes` |
| `meshRoutes()` | `GET /v1/mesh/routes` | `GetMeshRoutes` |
| `meshHealth()` | `GET /v1/mesh/health` | `GetMeshHealth` |
| `meshPartitions()` | `GET /v1/mesh/partitions` | `GetMeshPartitions` |
| — | `GET /v1/mesh/graph` | `GetMeshGraph` |
| — | `GET /v1/mesh/merge-report` | `GetMeshMergeReport` |
| — | `POST /v1/mesh/discover` | `DiscoverMesh` |
| `meshFindCapability()` | `POST /v1/mesh/find-capability` | `FindMeshCapability` |
| — | `POST /v1/mesh/simulate-partition` | `SimulateMeshPartition` |

Pin gRPC proto semver via `GET /v1/version` (currently **1.0.15**). Rust `GrpcClient` (`grpc` feature):
`get_mesh_topology`, `get_mesh_nodes`, `get_mesh_routes`, `get_mesh_health`, `get_mesh_graph`,
`find_mesh_capability`.

REST/SDK reference: [entity-apis.md](./entity-apis.md), [entity-sdk.md](./entity-sdk.md).

## Compatibility rules

Entity Mesh is **additive only**:

- All mesh messages use **secure messaging** (`SignedMessage` envelope)
- Takeover goes through **Recovery Orchestrator / Mission Continuity**
- Mesh topology is projected as **Entity Graph** relationships (`CommunicatesWith`)
- Coordinator election selects a **communication role only** — no new safety or actuator authority
- Partition mode is **more restrictive** than normal mode

## Related docs

- [Mesh topology](./mesh-topology.md)
- [Mesh security](./mesh-security.md)
- [Partition handling](./mesh-partition-handling.md)
- [Leader election](./mesh-leader-election.md)
- [Capability routing](./mesh-capability-routing.md)
- [Mesh sync](./mesh-sync.md)
- [Entity model](./entity-model.md)

## Status

**Experimental** — see [feature-status.md](./feature-status.md). Stable promotion checklist:
[entity-mesh-stable-promotion.md](./entity-mesh-stable-promotion.md).
