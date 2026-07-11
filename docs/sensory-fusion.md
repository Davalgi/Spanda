# Sensory Fusion

**Functional domain:** [Sensory Fusion](./functional-domains.md#sensory-fusion)  
**Status: Stable** — rule-based entity-derived fusion (`health_status`, `readiness_status`,
`trust_status`); conflicts feed readiness partial scoring; live multi-sensor pipeline
**Stable-with-env-gate** via `SPANDA_LIVE_FUSION_SENSORS=1`.

> Canonical architecture:
> [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)

## Purpose

Do not trust a single signal when safety depends on multiple sources.

## Types

`SensorConfidence`, `FusedObservation`, `SignalAgreement`, `SignalConflict`

## Examples

- GPS + IMU + wheel odometry
- Camera + LiDAR + radar
- Wearable + location + operator input

## On conflict

Lower readiness, trigger diagnosis, require fallback, escalate if safety-critical.

## CLI

```bash
spanda fusion check
```

## API

REST: `GET /v1/autonomy/fusion` · SDK: `FusionClient`

See [confidence-model.md](./confidence-model.md),
[responsibility-matrix.md](./responsibility-matrix.md).
