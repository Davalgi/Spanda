# Entity Model — Best Practices

Guidance for adopting the Unified Entity Model without breaking existing workflows.

## Prefer entity APIs for cross-kind questions

Use `spanda entity query`, `GET /v1/entities/query`, or Control Center **Entities** when the
question spans robots, devices, packages, and humans. Keep domain APIs (`/v1/devices`, `/v1/robots`)
for authoring and pool operations.

## Evaluate before deploy

Run the entity evaluation trio on critical paths:

```bash
spanda entity verify rover-001 --program mission.sd --dependencies
spanda entity readiness rover-001 --program mission.sd
spanda entity trust rover-001 --program mission.sd
```

## Extend via projection, not replacement

Add new industry objects as `EntityKind` variants and projection in `build_entity_registry`. Keep
TOML source types authoritative.

## Use relationships for impact analysis

Model `depends_on`, `assigned_to`, and `participates_in` edges so `entity graph`, traceability, and
verification dependency checks stay accurate.

## Mutation overlay vs TOML

Use `POST /v1/entities/register` for runtime overlay; `POST /v1/entities/sync` to persist durable
fragments. Do not edit generated registry JSON by hand.

## Related docs

- [entity-model.md](./entity-model.md)
- [entity-migration-guide.md](./entity-migration-guide.md)
- [entity-integration-report.md](./entity-integration-report.md)
