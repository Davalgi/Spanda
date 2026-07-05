# Operational Governance — Stable Hardening Checklist

**Promoted 2026-07-04** — platform operational governance framework (`spanda-governance`).

**Related:** [governance.md](./governance.md) · [governance-migration.md](./governance-migration.md)
· [compliance-framework.md](./compliance-framework.md)

> Spanda provides governance abstractions and validation mechanisms — not legal or regulatory advice.

| Gate | Status |
|------|--------|
| `spanda-governance` crate | **Shipped** |
| Entity extensions (`EntityGovernanceMeta`) | **Shipped** |
| Config fragment + auto-load `spanda.governance.toml` | **Shipped** |
| CLI (`compliance check`, `governance validate\|report`, `certification *`, `deployment profile\|verify`, `risk report`) | **Shipped** |
| REST + gRPC (proto **1.0.12**) | **Shipped** |
| SDK clients (Rust/Python/TypeScript) | **Shipped** |
| Control Center Governance tab (owners, policies, audit) | **Shipped** |
| Runtime influence (readiness, trust, gates, decisions, recovery) | **Shipped** |
| Live decision enforcement (`lookup_entity_for_governance`) | **Shipped** |
| Examples (`examples/governance/*`) | **Shipped** |
| Standards profile packages (`spanda-standards-*`) | **Shipped** (scaffolds) |
| `scripts/operational_governance_smoke.sh` | **Shipped** |
| Field soak (30 days) | **Pending** — shared enterprise soak file; CI uses `SPANDA_GOVERNANCE_SKIP_SOAK=1` |

## Smoke

```bash
./scripts/operational_governance_smoke.sh
```

Pass examples: `warehouse`, `industrial-robot`, `smart-building`, `adas`, `connected-healthcare`.

Expected fail (live maturity without operational certification): `hospital`, `search-rescue`.

## Promotion gate

```bash
SPANDA_GOVERNANCE_SKIP_SOAK=1 ./scripts/operational_governance_stable_promotion_gate.sh
```

## Known limits

- Standards packages define requirement *categories* only; no embedded regulatory text.
- Live enforcement requires project config or entity overlay visible to the process
  (`SPANDA_PROJECT_ROOT` / cwd).
- Policy signing uses content-hash material, not HSM-backed keys (optional future).
