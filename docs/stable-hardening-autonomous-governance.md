# Autonomous Governance — Stable Hardening Checklist

**Promoted 2026-07-02** — LATER differentiation pillar #15.

**Related:** [autonomous-governance.md](./autonomous-governance.md) ·
[policy-engine.md](./policy-engine.md)

| Gate | Status |
|------|--------|
| `spanda governance` | **Shipped** |
| Showcases | `governance/night_ops.sd`, `policy/warehouse.sd` |
| Runtime policy | `spanda verify --policy`, `deploy gate --operational-policy` |

```bash
SPANDA_LATER_SKIP_SOAK=1 SPANDA_LATER_SKIP_SMOKE=1 ./scripts/later_differentiation_stable_promotion_gate.sh
```
