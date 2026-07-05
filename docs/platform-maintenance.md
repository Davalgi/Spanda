# Maintenance & Optimization

**Functional domain:** [Maintenance &
Optimization](./functional-domains.md#maintenance--optimization)
**Status: Beta** — OTA and fleet operations **Stable**; maintenance window types **Beta**.

> Canonical architecture:
> [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)

## Purpose

Schedule and execute maintenance, calibration, cleanup, updates, and resource optimization during
low-risk windows without compromising mission safety.

## Responsibilities

| Responsibility | Platform service | Tier |
|----------------|------------------|------|
| OTA rollouts | Fleet OTA, `spanda deploy rollout` | **Stable** |
| Maintenance windows | `spanda-autonomy::maintenance` | **Beta** |
| Calibration | Device pool + verify | **Beta** |
| Log rotation / cleanup | Telemetry store | **Stable** |
| Backup | Config snapshots | **Stable** |
| Sleep / low-activity mode | `SleepMode`, `LowActivityMode` | **Beta** |

## Types (`spanda-autonomy`)

`MaintenanceWindow`, `SleepMode`, `LowActivityMode`, `ScheduledRecovery`, `CalibrationWindow`,
`UpdateWindow`

## Entity integration

| Field | Use |
|-------|-----|
| `lifecycle_state` | Maintenance vs active |
| `firmware_version` / `software_version` | OTA target tracking |
| `Entity.homeostasis` | Defer maintenance when unstable |

## Examples

- Nightly calibration during `SleepMode`
- Firmware update in `UpdateWindow` when fleet readiness partial
- Log compaction when storage drift detected via homeostasis
- Sensor self-test before mission resume

## CLI

OTA and deploy (Stable):

```bash
spanda deploy plan
spanda deploy rollout
spanda deploy rollback
```

Maintenance window CLI — **Planned** (`spanda maintenance window`).

## API

| Surface | Endpoint |
|---------|----------|
| OTA | `/v1/ota/*` |
| Homeostasis trigger | `GET /v1/autonomy/homeostasis` |
| Governance windows | `GET /v1/governance` |

## Control Center

OTA tab (**Stable**). Maintenance scheduling panel — **Planned**; homeostasis drift may recommend
maintenance in Cognitive & Resilience tab.

## Related

- [maintenance-mode.md](./maintenance-mode.md) — type reference
- [platform-homeostasis.md](./platform-homeostasis.md) — stability triggers
- [cognitive-resilience-maturity.md](./cognitive-resilience-maturity.md) — promotion criteria
