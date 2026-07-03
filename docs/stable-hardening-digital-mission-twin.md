# Digital Mission Twin — Stable Hardening Checklist

**Promoted 2026-07-02** — LATER differentiation pillar #11.

**Related:** [digital-mission-twin.md](./digital-mission-twin.md)

| Gate | Status |
|------|--------|
| `spanda twin mission` | **Shipped** |
| Showcase | `examples/showcase/mission_twin/patrol.sd` |
| Smoke | `scripts/later_differentiation_smoke.sh` |

Cloud sync (`spanda-twin-cloud`) — **Stable** SaaS backend on Control Center; see [twin-cloud.md](./twin-cloud.md), [hosted-twin-cloud.md](./hosted-twin-cloud.md), [stable-hardening-twin-cloud-saas.md](./stable-hardening-twin-cloud-saas.md).

```bash
SPANDA_LATER_SKIP_SOAK=1 SPANDA_LATER_SKIP_SMOKE=1 ./scripts/later_differentiation_stable_promotion_gate.sh
```
