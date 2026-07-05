# Attention System

**Status: Preview**

## Purpose

Reduce event overload; prioritize important signals for telemetry, alerting, diagnosis, and Control Center dashboards.

## Types

`AttentionPolicy`, `AttentionScore`, `EventPriority`, `SignalPriority`, `AttentionWindow`, `SuppressionRule`

## Examples

- Critical safety event > routine telemetry
- Repeated low-value warning suppressed
- Mission-relevant events prioritized

## API

`GET /v1/autonomy/attention`

See [bio-inspired-architecture.md](./bio-inspired-architecture.md).
