# Homeostasis

**Status: Preview**

## Purpose

Maintain safe operating range **before** failures occur.

## Monitored metrics

CPU, memory, battery, temperature, latency, network, storage, health, readiness, trust, sensor quality.

## Types

`HomeostasisPolicy`, `StabilityRange`, `StabilityMetric`, `DriftSignal`, `CorrectionAction`, `StabilityReport`

## Examples

- Memory rising steadily → restart low-risk provider
- Temperature rising → reduce workload
- Battery degrading → re-plan mission
- Network unstable → switch transport

## CLI

```bash
spanda homeostasis check
spanda homeostasis report
```

## API

`GET /v1/autonomy/homeostasis`
