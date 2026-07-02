# Autonomous Governance

**Status:** Stable · **Horizon:** LATER (v0.7) · **Priority:** P2

Evaluate operational policy blocks at verify time and via CLI before deployment.

## CLI

```bash
spanda governance examples/showcase/governance/night_ops.sd
spanda governance examples/showcase/governance/night_ops.sd --policy NightOps --json
spanda governance examples/showcase/policy/warehouse.sd --policy WarehousePolicy
```

Lists `policy { }` declarations in the program and reports violations (max speed, kill switch, readiness minimum, capabilities, operation hours).

## Policy blocks

```spanda
policy NightOps {
    max_speed = 1.0 m/s;
    requires_kill_switch;
    min_readiness_score = 80;
}
```

Runtime enforcement uses `spanda verify --policy`, `readiness --policy`, and `deploy gate --operational-policy` (see [policy-engine.md](./policy-engine.md)).

## Showcase

| Example | Purpose |
|---------|---------|
| `examples/showcase/governance/night_ops.sd` | Night operations speed + readiness gate |
| `examples/showcase/policy/warehouse.sd` | Warehouse operational policy (smoke default) |

Part of `spanda demo governance` and `scripts/later_differentiation_smoke.sh`.

See [differentiation-roadmap.md](./differentiation-roadmap.md) · [policy-engine.md](./policy-engine.md) · [deploy-gates.md](./deploy-gates.md).
