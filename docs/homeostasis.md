# Homeostasis

**Status: Beta** — entity health/readiness/trust signals plus scheduler telemetry from recent `run`/`sim`.

## Purpose

Maintain safe operating range **before** failures occur.

## Monitored metrics

CPU, memory, battery, temperature, latency, network, storage, health, readiness, trust, sensor quality, scheduler ticks, runtime load, deadline misses, provider failures.

## Language

```spanda
homeostasis_policy PlatformStability {
    metric cpu_pct;
    metric memory_pct;
    metric battery_pct;
    metric scheduler_ticks;
}
```

## Types

`HomeostasisPolicy`, `StabilityRange`, `StabilityMetric`, `DriftSignal`, `CorrectionAction`, `StabilityReport`

## Examples

- Memory rising steadily → restart low-risk provider
- Temperature rising → reduce workload
- Battery degrading → re-plan mission
- Network unstable → switch transport
- Scheduler deadline misses rising → enter degraded mode

## CLI

```bash
spanda homeostasis check
spanda homeostasis report
```

## API

- REST: `GET /v1/autonomy/homeostasis`
- gRPC: `GetAutonomyHomeostasis`

See [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md) · [platform-homeostasis.md](./platform-homeostasis.md).
