# Fleet management

Multi-robot fleet with health requirements, safety zones, and fleet communication.

```bash
spanda check examples/showcase/fleet_management/fleet.sd
spanda verify examples/showcase/fleet_management/fleet.sd --health
```

Note: `spanda fleet run` for multi-robot programs is experimental; health verification is the stable evaluator path.

One command: `spanda demo fleet`

Docs: [fleet-health.md](../../docs/fleet-health.md), [concurrency.md](../../docs/concurrency.md)
