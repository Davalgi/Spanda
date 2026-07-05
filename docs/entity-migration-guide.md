# Entity Model — Migration Guide

The Unified Entity Model is **additive**. Existing APIs, CLIs, and TOML schemas continue to work.

## What changes for operators

| Before | After (recommended) |
|--------|---------------------|
| Ad-hoc device + verify checks | `spanda entity verify <id>` |
| Robot-only readiness views | `spanda entity readiness <id>` for any kind |
| Separate trust CLI per domain | `spanda entity trust <id>` |

## What does not change

- `spanda verify`, `spanda verify-fleet`, `spanda device *`
- `/v1/devices`, `/v1/robots`, `/v1/fleets`, `/v1/humans`
- `DeviceRegistry`, `HumanRegistry`, fleet TOML authoring

## SDK migration

Existing SDK list/get methods are unchanged. Add typed evaluation calls:

| SDK | New methods |
|-----|-------------|
| Rust | `entity_verify`, enriched `entity_readiness` / `entity_health` / `entity_trust` responses |
| TypeScript | `verifyEntity`; readiness/health/trust return `report` object |
| Python | `entity_verify`; same `report` enrichment |

## Control Center

The **Entities** tab queries `/v1/entities/*`. Legacy robot/device tabs remain; they project from
the same registry.

## Phased adoption

1. **Inventory** — `spanda entity list`, graph, relationships
2. **Verification** — `spanda entity verify` (Phase 2)
3. **Operations** — readiness, health, trust reports (Phases 3–5)
4. **Authoring** — mutations + sync when runtime overlay is needed

See [entity-integration-report.md](./entity-integration-report.md) for phase status.
