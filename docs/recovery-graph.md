# Recovery Graph

The recovery graph models entity relationships for dependency analysis, impact analysis, and recovery planning.

## Graph views

| View | Purpose |
|------|---------|
| **Dependency graph** | Upstream dependencies (`depends_on`, `consumes`, `controls`) |
| **Impact graph** | Downstream entities affected by a failure |
| **Recovery graph** | Critical recovery paths |

## Example chain

```
Mission → Robot → Camera → Firmware → Provider → Package → Gateway → Cloud
```

## Building the graph

```rust
use spanda_recovery::build_recovery_graph;

let graph = build_recovery_graph(&registry, Some("robot-1"));
let (upstream, downstream) = spanda_recovery::analyze_impact(&graph, "camera-1");
```

## CLI

```bash
spanda recovery graph rover.sd --entity robot-1 --json
```

## REST API

`GET /v1/recovery/graph?entity_id=robot-1`

## SDK

```rust
client.get_recovery_graph(Some("robot-1"))?;
```

## Control Center

The Recovery Graph panel visualizes nodes, dependency edges, and impact edges alongside the entity graph.

## See also

- [entity-model.md](./entity-model.md)
- [recovery-orchestrator.md](./recovery-orchestrator.md)
