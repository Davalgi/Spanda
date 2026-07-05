# What-If Analysis — Stable Hardening Checklist

What-If Analysis is the first **NEXT** differentiation pillar promoted to **Stable** tier.
**Promoted 2026-07-02** after `what_if_stable_promotion_gate.sh`.

**Related:** [what-if-analysis.md](./what-if-analysis.md) · [feature-status.md](./feature-status.md)
· [differentiation-roadmap.md](./differentiation-roadmap.md)

---

## Promotion criteria

| Gate | Requirement | Status |
|------|-------------|--------|
| Crate tests | `cargo test -p spanda-whatif` | **Shipped** |
| CLI smoke | `scripts/what_if_smoke.sh` | **Shipped** |
| Demo | `spanda demo what-if` | **Shipped** |
| Showcase | `examples/showcase/what_if/gps_failure.sd` | **Shipped** |
| Control Center REST | `GET /v1/analytics/what-if` | **Shipped** |
| Control Center gRPC | `GetAnalyticsWhatIf` (proto **1.0.14**) | **Shipped** |
| SDK wrappers | Rust / Python / TypeScript `analyticsWhatIf` | **Shipped** |
| Stable gate script | `scripts/what_if_stable_promotion_gate.sh` | **Shipped** |
| Field soak | 30-day operational pilot | **Pending** — `.spanda/what-if-field-soak-start.txt` |
| Security audit | Third-party review of scenario injection paths | **Pending** — `./scripts/what_if_security_audit_prep.sh` |

---

## Running the promotion gate

```bash
# Start 30-day pilot clock (UTC) — one-time
./scripts/what_if_field_soak_init.sh

# Generate audit intake artifact
./scripts/what_if_security_audit_prep.sh

# Full gate (after soak + audit, or CI with skips):
chmod +x scripts/what_if_stable_promotion_gate.sh
./scripts/what_if_stable_promotion_gate.sh

# CI / local dev without soak or audit:
SPANDA_WHATIF_SKIP_SOAK=1 SPANDA_WHATIF_SKIP_AUDIT=1 ./scripts/what_if_stable_promotion_gate.sh

# CI after what-if-smoke job (skip duplicate smoke):
SPANDA_WHATIF_SKIP_SOAK=1 SPANDA_WHATIF_SKIP_AUDIT=1 SPANDA_WHATIF_SKIP_SMOKE=1 \
  ./scripts/what_if_stable_promotion_gate.sh
```

The gate runs:

1. Field soak check (unless `SPANDA_WHATIF_SKIP_SOAK=1`)
2. Security audit prep artifact (unless `SPANDA_WHATIF_SKIP_AUDIT=1`)
3. `scripts/what_if_smoke.sh` (unless `SPANDA_WHATIF_SKIP_SMOKE=1`)
4. `spanda-whatif` + differentiation analytics API tests
5. Live Control Center probe: `GET /v1/analytics/what-if?all=1`

---

## Promotion status (2026-07-02)

**Promoted to Stable** in `docs/feature-status.md`, [ROADMAP.md](../ROADMAP.md), and
[differentiation-roadmap.md](./differentiation-roadmap.md).

Remaining NEXT pillars (risk, forecast, trust graph, scorecards) promoted **Stable** — see sibling
stable-hardening guides.

### Ongoing organizational gates

| Gate | Status |
|------|--------|
| 30-day what-if field soak | **Pending** — `./scripts/what_if_field_soak_init.sh` |
| Third-party security audit sign-off | **Pending** — `./scripts/what_if_security_audit_prep.sh` |
