# Warehouse Automation — Solution Blueprint

**Status:** Experimental · **Timeline:** Now · **Path:** `examples/end_to_end/warehouse_delivery/`

Official Solution Blueprint for autonomous mobile robots, pick-and-place logistics, and fleet coordination in warehouse environments.

**Full roadmap entry:** [ROADMAP.md § Warehouse Automation](../../ROADMAP.md#warehouse-automation)

---

## Purpose

Coordinate AMRs through loading zones, transit corridors, and dock areas with safety zones, fleet health requirements, mission continuity, and Control Center visibility.

## Platform pillars used

| Pillar | Capabilities |
|--------|--------------|
| Device & Fleet | Device tree, fleet `require`, continuity/takeover |
| Verification | Readiness go/no-go, assurance, recovery |
| Operations | Control Center, telemetry, alerting |
| Developer | Bundled demos, CI smoke |
| Packages | `spanda-nav`, `spanda-fleet`, `spanda-mqtt`, `spanda-opencv` |

## Reference architecture

```text
Warehouse Stack
├── AMR fleet (.sd programs)
├── Zone safety (dock, aisle, staging)
├── MQTT / ROS2 comms (optional packages)
├── Readiness gates (pre-shift)
├── Continuity on robot failure (reassign mission)
└── Control Center fleet dashboard
```

## Device tree

Warehouse TOML fixtures: `crates/spanda-config/tests/fixtures/warehouse/`

Guide: [device-tree.md](../device-tree.md)

## Packages & providers

| Package | Role |
|---------|------|
| `spanda-nav` | Route planning |
| `spanda-fleet` | Multi-robot coordination |
| `spanda-mqtt` | WMS / fleet bus integration |
| `spanda-opencv` | Barcode / lane vision |
| `spanda-mission-continuity` | Takeover on AMR failure |

## Mission examples

| Example | Path |
|---------|------|
| Warehouse delivery (package) | [examples/end_to_end/warehouse_delivery/](../../examples/end_to_end/warehouse_delivery/) |
| Continuity warehouse | [examples/showcase/continuity/warehouse.sd](../../examples/showcase/continuity/warehouse.sd) |
| Warehouse logistics | [examples/warehouse_logistics.sd](../../examples/warehouse_logistics.sd) |
| Warehouse robot | [examples/warehouse_robot.sd](../../examples/warehouse_robot.sd) |

## Health policies & readiness

- Fleet `require` clauses — [health-checks.md](../health-checks.md)
- Operational readiness — [readiness.md](../readiness.md)
- Continuity walkthrough — [tutorials/continuity-walkthrough.md](../tutorials/continuity-walkthrough.md)

## Assurance & recovery

- [mission-assurance.md](../mission-assurance.md)
- [self-healing.md](../self-healing.md)
- [mission-continuity.md](../mission-continuity.md)

## Control Center

```bash
spanda control-center serve --config spanda.toml --program src/main.sd
```

Guide: [control-center.md](../control-center.md)

## Simulation & replay

```bash
spanda sim examples/end_to_end/warehouse_delivery/src/main.sd
spanda replay mission.trace --deterministic
```

## Quick start

```bash
spanda check examples/end_to_end/warehouse_delivery/src/main.sd
spanda verify examples/end_to_end/warehouse_delivery/src/main.sd --target RoverV1
spanda continuity examples/showcase/continuity/warehouse.sd --json
```

## Smoke gates

`scripts/showcase_smoke.sh` · `scripts/continuity_smoke.sh` · [scripts/gates/README.md](../../scripts/gates/README.md)

---

**Related blueprints:** [Smart Factory](./smart-factory.md) · [Transportation](./adas.md)
