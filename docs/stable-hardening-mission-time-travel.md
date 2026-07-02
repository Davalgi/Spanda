# Mission Time Travel — Stable Hardening Checklist

**Promoted 2026-07-02** — LATER differentiation pillar #13.

**Related:** [mission-time-travel.md](./mission-time-travel.md)

| Gate | Status |
|------|--------|
| `spanda replay --at` / `--inspect` | **Shipped** |
| Golden trace fixture | `differentiation/decision_trail/main.trace` |
| Runtime tests | `crates/spanda-runtime/tests/time_travel_tests.rs` |
| Smoke | `scripts/later_differentiation_smoke.sh` |

```bash
SPANDA_LATER_SKIP_SOAK=1 SPANDA_LATER_SKIP_SMOKE=1 ./scripts/later_differentiation_stable_promotion_gate.sh
```

Field soak: `./scripts/later_field_soak_init.sh`
