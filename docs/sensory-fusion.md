# Sensory Fusion

**Functional domain:** [Sensory Fusion](./functional-domains.md#sensory-fusion)  
**Status: Experimental** — rule-based validators; no live sensor fusion pipeline yet.

> Canonical architecture: [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)

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

See [confidence-model.md](./confidence-model.md), [responsibility-matrix.md](./responsibility-matrix.md).
