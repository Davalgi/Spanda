# Maintenance & Optimization

**Functional domain:** [Maintenance &
Optimization](./functional-domains.md#maintenance--optimization)
**Status: Stable** — OTA, fleet operations, and `spanda maintenance window` list/set.

> Canonical architecture:
> [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)

## Purpose

Schedule and execute maintenance, calibration, cleanup, updates, and resource optimization during
low-risk windows without compromising mission safety.

## Responsibilities

| Responsibility | Platform service | Tier |
|----------------|------------------|------|
| OTA rollouts | Fleet OTA, `spanda deploy rollout` | **Stable** |
| Maintenance windows | `spanda maintenance window`, `/v1/autonomy/maintenance/windows` | **Stable** |
| Calibration | Device pool + verify | **Stable** |
| Log rotation / cleanup | Telemetry store | **Stable** |
| Backup | Config snapshots | **Stable** |
| Sleep / low-activity mode | `SleepMode`, `LowActivityMode` | **Stable** |

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

Maintenance window CLI:

```bash
spanda maintenance window list
spanda maintenance window set --id nightly --start 2026-07-12T02:00:00Z --end 2026-07-12T04:00:00Z --activity ota
```

## API

| Surface | Endpoint |
|---------|----------|
| OTA | `/v1/ota/*` |
| Maintenance windows | `GET/POST /v1/autonomy/maintenance/windows` (POST requires Operate) |
| Homeostasis trigger | `GET /v1/autonomy/homeostasis` |
| Governance windows | `GET /v1/governance` |

## Control Center

OTA tab (**Stable**). Maintenance schedule panel on Cognitive & Resilience tab (**Stable**).

## Related

- [maintenance-mode.md](./maintenance-mode.md) — type reference
- [platform-homeostasis.md](./platform-homeostasis.md) — stability triggers
- [cognitive-resilience-maturity.md](./cognitive-resilience-maturity.md) — promotion criteria
