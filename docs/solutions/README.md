# Official Solution Blueprints

Spanda ships **Official Solution Blueprints** — reference architectures built entirely on existing platform capabilities. Each blueprint demonstrates how to compose language features, packages, verification, readiness, assurance, and operations for a specific industry without bloating the core.

| Blueprint | Status | Path |
|-----------|--------|------|
| **ADAS & Autonomous Driving** | Experimental | [examples/solutions/adas/](../../examples/solutions/adas/) |
| **Spatial Computing & Human-Robot Collaboration** | Planned | [examples/solutions/spatial-computing/](../../examples/solutions/spatial-computing/) |
| Autonomous Rover (flagship) | Stable | [examples/showcase/autonomous_rover/](../../examples/showcase/autonomous_rover/) |
| Compliance profiles | Experimental | [examples/showcase/compliance/](../../examples/showcase/compliance/) |
| Warehouse operations | Experimental | [examples/end_to_end/warehouse_delivery/](../../examples/end_to_end/warehouse_delivery/) |

## ADAS & Autonomous Driving

Safety-first intelligent vehicle workflows — lane keeping, adaptive cruise, emergency braking, sensor recovery, driver takeover, and highway pilot.

- **Architecture:** [adas.md](./adas.md)
- **Device tree:** [automotive-device-tree.md](../automotive-device-tree.md)
- **Readiness:** [adas-readiness.md](../adas-readiness.md)
- **Assurance:** [adas-assurance.md](../adas-assurance.md)
- **Security:** [adas-security.md](../adas-security.md)
- **Replay:** [adas-replay.md](../adas-replay.md)

```bash
spanda demo adas
./scripts/adas_smoke.sh
```

See also: [compliance-profiles.md](../compliance-profiles.md) (ISO 26262) · [mission-continuity.md](../mission-continuity.md) · [control-center.md](../control-center.md)

## Spatial Computing & Human-Robot Collaboration

Human–robot collaboration, wearables, AR/VR/XR, and collaborative autonomy — composes Device Registry, Capability Framework, Readiness, Continuity, Trust, and Control Center without core language extensions.

- **Architecture:** [spatial-computing.md](./spatial-computing.md)
- **Roadmap:** [human-interaction-spatial-computing-roadmap.md](../human-interaction-spatial-computing-roadmap.md)
- **Human entity:** [human-interaction.md](../human-interaction.md)
- **Operator capabilities:** [operator-capabilities.md](../operator-capabilities.md)
- **Human readiness:** [human-readiness.md](../human-readiness.md)
- **Packages:** [hri-packages.md](../hri-packages.md)

```bash
# When H1 ships:
cd examples/solutions/spatial-computing && spanda check warehouse-ar/pick_mission.sd
```

