# Attention Engine

**Functional domain:** [Attention Engine](./functional-domains.md#attention-engine)  
**Status: Beta** — health-mapped event prioritization for Control Center, telemetry, and API
surfaces.

> Canonical architecture:
> [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)
> Prior short guide: [attention-system.md](./attention-system.md)

## Purpose

Reduce event overload and **alert fatigue** by prioritizing signals that matter for safety,
missions, and operators. The Attention Engine ranks, suppresses, aggregates, and focuses events — it
does not replace alerting infrastructure; it governs **what surfaces first**.

## Responsibilities

| Responsibility | Mechanism |
|----------------|-----------|
| Prioritize critical events | `EventPriority::Critical` + severity boost |
| Boost mission events | Attention policy rules in `.sd` |
| Suppress routine noise | Habituation policy (`spanda-autonomy::habituation`) |
| Focus operator attention | Ranked `AttentionWindow` |

## Entity integration

| Field | Description |
|-------|-------------|
| `Entity.attention` | `EntityAttentionSnapshot` — `top_priority`, `queue_depth`, `focused_event`, `suppressed_count` |

Populated during `enrich_entity_autonomy()` from health/readiness severity mapping.

## Language

```spanda
@policy(kind: "attention")
MissionFocus {
    rule suppress_low_priority;
    rule boost_critical_health;
}
```

## Types (`spanda-autonomy`)

`AttentionPolicy`, `AttentionScore`, `EventPriority`, `SignalPriority`, `AttentionWindow`,
`SuppressionRule`

## Examples

- Critical safety event ranks above routine telemetry
- Repeated low-value warning suppressed after habituation threshold
- Degraded entity health boosted to Urgent priority
- Recovery completion deprioritized when critical alert active

## CLI

```bash
spanda alerts analyze
spanda alerts fatigue-report
```

## API

| Surface | Endpoint / RPC |
|---------|----------------|
| REST | `GET /v1/autonomy/attention` (uses `@policy` rules when Control Center started with `--program`) |
| gRPC | `GetAutonomyAttention` |
| SDK | `AttentionClient::queue()` |
| Entity | `GET /v1/entities/{id}/autonomy` → `attention` field |

## Control Center

**Attention Queue** section in the Cognitive & Resilience tab.

## Integrations

- **Recovery:** recovery events enter attention queue with mission-relative priority
- **Telemetry:** WebSocket stream feeds attention scoring (health-mapped)
- **Diagnosis:** high-attention events link to diagnose workflows

See [responsibility-matrix.md](./responsibility-matrix.md#attention-engine).
