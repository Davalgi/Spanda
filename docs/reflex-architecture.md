# Reflex Architecture

**Status: Beta** — built on distributed decision layer 0, kill switch, and runtime fault detection.

## Purpose

Immediate local safety response without waiting for cloud or fleet coordination.

## Flow

```text
Sensor / detector
        ↓
Reflex controller
        ↓
Immediate safe action
        ↓
Audit + Control Center notification
```

## Examples

- Emergency stop
- Obstacle imminent
- Actuator overcurrent
- Thermal runaway
- Collision risk
- Untrusted command rejected
- Unsafe actuator request blocked

## Integration

- [Distributed decisions](./distributed-decisions.md) — `DecisionLayer::Reflex`
- Kill switch, runtime fault detection, safety validation, recovery, replay

## CLI

```bash
spanda reflex list
spanda reflex simulate emergency
spanda reflex trace obstacle
```

## API

`GET /v1/autonomy/reflex`

See [bio-inspired-architecture.md](./bio-inspired-architecture.md).
