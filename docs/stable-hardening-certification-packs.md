# Certification Packs — Stable Hardening Checklist

**Promoted 2026-07-02** — LATER differentiation pillar #12.

**Related:** [certification-packs.md](./certification-packs.md)

| Gate | Status |
|------|--------|
| `spanda certify pack --bundle` | **Shipped** |
| Evidence composition | verify, readiness, safety, recovery, trust |
| Showcase | `examples/showcase/certify/deployment_bundle/rover.sd` |

Bundle may exit non-zero when evidence thresholds fail; gate validates JSON + manifest files.

```bash
SPANDA_LATER_SKIP_SOAK=1 SPANDA_LATER_SKIP_SMOKE=1 ./scripts/later_differentiation_stable_promotion_gate.sh
```
