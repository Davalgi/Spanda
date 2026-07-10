# Attention System

**Status: Beta** — health-mapped event prioritization for Control Center and API surfaces.

## Purpose

Reduce event overload; prioritize important signals for telemetry, alerting, diagnosis, and Control
Center dashboards.

## Language

```spanda
@policy(kind: "attention")
MissionFocus {
    rule suppress_low_priority;
    rule boost_critical_health;
}
```

## Types

`AttentionPolicy`, `AttentionScore`, `EventPriority`, `SignalPriority`, `AttentionWindow`,
`SuppressionRule`

## Examples

- Critical safety event > routine telemetry
- Repeated low-value warning suppressed
- Mission-relevant events prioritized

## API

- REST: `GET /v1/autonomy/attention`
- gRPC: `GetAutonomyAttention`

See [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md) ·
[attention-engine.md](./attention-engine.md).
