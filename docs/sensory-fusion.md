# Sensory Fusion

**Status: Experimental** — rule-based validators; no live sensor fusion pipeline yet.

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

See [confidence-model.md](./confidence-model.md).
