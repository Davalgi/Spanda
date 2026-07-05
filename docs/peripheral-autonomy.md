# Peripheral Autonomy

**Status: Beta** — extends distributed decisions hierarchy.

## Purpose

Avoid over-centralization. Represent autonomous execution hierarchy:

```text
Control Center
        ↓
Regional / Site Coordinator
        ↓
Fleet / Swarm Coordinator
        ↓
Robot / Entity Runtime
        ↓
Device / Reflex Controller
```

## Types

- `PeripheralNode`, `EdgeCoordinator`, `LocalAutonomyNode`, `RegionalCoordinator`

## Integration

Entity graph, device tree, fleet, mission continuity, delegation, takeover, offline policies.

See [distributed-decisions.md](./distributed-decisions.md) and
[cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md).
