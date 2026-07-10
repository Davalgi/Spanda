# Platform Homeostasis

**Functional domain:** [Homeostasis Engine](./functional-domains.md#homeostasis-engine)  
**Status: Beta** — entity health/readiness/trust signals plus scheduler telemetry from recent
`run`/`sim`.

> Canonical architecture:
> [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)
> Prior short guide: [homeostasis.md](./homeostasis.md) (retained for CLI/API quick reference)

## Purpose

Maintain **stable operating conditions before failures occur**. Homeostasis monitors platform and
entity signals, detects drift, and recommends or triggers corrective actions through existing
Health, Telemetry, Recovery, and Readiness services.

Spanda does **not** simulate biological regulation — homeostasis is an engineering stability loop.

## Monitored signals

| Signal category | Sources |
|-----------------|---------|
| Compute | CPU, memory, scheduler ticks, deadline misses, runtime load |
| Power | Battery percentage, degradation |
| Environment | Temperature, storage |
| Network | Latency, connectivity, transport stability |
| Operational | Health, readiness, trust, sensor quality |
| Runtime | Provider failures, emergency stops |

## Entity integration

| Field | Description |
|-------|-------------|
| `Entity.health_status` | Primary health input |
| `Entity.readiness_status` | Mission go/no-go input |
| `Entity.trust_status` | Stability trust dimension |
| `Entity.homeostasis` | `EntityHomeostasisSnapshot` — `stable`, `drift_signals`, `last_report_at` |

## Language

```spanda
@policy(kind: "homeostasis")
PlatformStability {
    metric cpu_pct;
    metric memory_pct;
    metric battery_pct;
    metric scheduler_ticks;
}
```

## Types (`spanda-autonomy`)

`HomeostasisPolicy`, `StabilityRange`, `StabilityMetric`, `DriftSignal`, `CorrectionAction`,
`StabilityReport`

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

| Surface | Endpoint / RPC |
|---------|----------------|
| REST | `GET /v1/autonomy/homeostasis` (uses `@policy` metrics when Control Center started with `--program`) |
| gRPC | `GetAutonomyHomeostasis` |
| SDK | `HomeostasisClient::summary()` (Rust), `homeostasis().summary()` (TS/Python) |
| Entity | `GET /v1/entities/{id}/autonomy` → `homeostasis` field |

## Control Center

**Homeostasis** section in the Cognitive & Resilience tab — stability reports per entity.

## Integrations

- **Reflex & Safety:** critical drift may trigger reflex evaluation
- **Recovery:** correction actions route to recovery recommend/execute
- **Readiness:** unstable entities reduce composite readiness score

See [responsibility-matrix.md](./responsibility-matrix.md#homeostasis-engine).
