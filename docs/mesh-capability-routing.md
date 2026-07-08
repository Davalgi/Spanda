# Mesh Capability Routing

Entities advertise capabilities through the existing **Capability Framework**. Mesh discovery projects registry capabilities into `MeshCapabilityAdvertisement`.

## Example

| Entity | Capabilities |
|--------|--------------|
| Robot A | `thermal_camera`, `victim_detection`, `local_mapping`, `relay_node` |
| Robot B | `heavy_payload`, `medical_delivery`, `LTE_gateway` |

## Routing modes

- Direct, Relay, Coordinator, Broadcast, Multicast
- **Capability-Based** — route to nearest capable trusted entity
- Trust-Weighted, Readiness-Weighted, Emergency

## CLI

```bash
spanda mesh capabilities
spanda mesh find --capability thermal_camera
spanda mesh route coordinator robot-a
```

## API

`POST /v1/mesh/find-capability`

Routing considers reachability, trust, readiness, health, identity, latency, bandwidth, hop count, battery, and mission priority.
