# Organizational Gates — v0.6.3 → v1.0

**v0.6.3** shipped as an **evaluation / beta** release with CI-backed quality gates and honest stability labels. Code blockers for that release are closed; **organizational gates** remain before **v1.0 production positioning**.

Tracked as [RB-007 / #51](https://github.com/Davalgi/Spanda/issues/51) in [release-blockers.md](./release-blockers.md).

---

## What v0.6.3 means today

| Claim | Status |
|-------|--------|
| Language, verify, sim, replay, recovery, continuity demos | **Supported for evaluation** — README smoke + golden tests in CI |
| Control Center, REST v1, SDKs | **Supported for evaluation** — cross-interface consistency in CI |
| Default AI / in-memory transport | **Mock-backed** — live paths require env and optional features |
| Enterprise ops pillars (E1–E4) | **Stable in code/CI** — organizational soak and external audit not yet complete |
| Full production readiness | **Not claimed** until gates below are accepted |

Public messaging: use **evaluation / beta**, link [known-limitations.md](./known-limitations.md), and do **not** imply IEC/ISO certification or managed-cloud SLAs.

---

## Gates required for v1.0

v1.0 is a **major** workspace bump (`1.0.0`) when code **and** organizational exit criteria are met. See [ROADMAP.md § v1.0](../ROADMAP.md#v10--production-positioning) and [versioning.md](./versioning.md).

### 1. Field soak (30 days)

Run representative workloads in staging or pilot environments — warehouse fixture, rover/self-healing showcase, Control Center with `--config` + `--program`.

```bash
# One-time: record soak start date
./scripts/enterprise_ops_field_soak_init.sh

# After 30+ days, verify elapsed time (gate script checks this)
./scripts/enterprise_ops_stable_promotion_gate.sh
```

Detail: [field-soak-gate.md](./field-soak-gate.md) · [enterprise-ops-stable-promotion.md](./enterprise-ops-stable-promotion.md)

**Acceptance:** no P0/P1 incidents attributable to platform core during soak; promotion gate passes without `SPANDA_ENTERPRISE_OPS_SKIP_SOAK=1`.

### 2. Third-party security audit

Generate the audit prep packet and engage an external reviewer.

```bash
./scripts/security_audit_prep.sh
# Artifact: .spanda/security-audit-prep.json (or SPANDA_SECURITY_AUDIT_PREP_FILE)
```

Detail: [security-audit-third-party.md](./security-audit-third-party.md)

**Acceptance:** signed audit report on file; no unresolved critical findings; promotion gate passes without `SPANDA_ENTERPRISE_OPS_SKIP_AUDIT=1`.

### 3. Code quality (already met for v0.6.3)

Maintain through v1.0 tag:

- No open **P0** / **P1** [release-blockers](./release-blockers.md)
- README command smoke + golden tests green
- Security regression suite green
- [release-readiness.md](./release-readiness.md) recommendation **Go** or **Go with documented limitations**

### 4. Stability honesty

- Mock-default AI remains **Mock-backed** in [feature-status.md](./feature-status.md) unless live path is default for a profile
- Experimental → Stable promotions require tests and non-mock default path per tier rules

---

## v1.0 exit checklist

Use this when preparing the `1.0.0` workspace tag:

| # | Item | Owner | Status |
|---|------|-------|--------|
| 1 | Field soak ≥ 30 days | Ops / pilot team | **In progress** — started 2026-06-29 (`.spanda/field-soak-start.txt`) |
| 2 | Third-party security audit sign-off | Security | Open |
| 3 | RB-007 closed or explicitly accepted in release notes | Maintainers | Open |
| 4 | Open P0/P1 release blockers | Engineering | **Met** (v0.6.3) |
| 5 | CI release-hardening suite green | CI | **Met** |
| 6 | [known-limitations.md](./known-limitations.md) matches shipped behavior | Docs | **Met** |
| 7 | Native codegen / device pool / RBAC promotion criteria (code) | Engineering | Partial — see [ROADMAP.md](../ROADMAP.md) v1.0 table |

When rows 1–3 and 7 code items are satisfied, bump workspace with `python3 scripts/bump_version.py major`, update [CHANGELOG.md](../CHANGELOG.md), and tag `v1.0.0`.

---

## Parallel work (does not replace gates)

While soak and audit run, engineering follows the **Next horizon** priorities in [ROADMAP.md](../ROADMAP.md#next-horizon-priorities-post-v063) under [scope-control.md](./scope-control.md):

- Differentiation signature capabilities (mission contracts, explainability, audit trail hardening)
- Control Center Stable promotion (wire playground-only panels)
- VS Code Marketplace publish (requires `VSCE_PAT`)
- Live vehicle I/O and swarm quorum hardening
- Experimental → Stable promotions with tests

Organizational gates and platform work proceed in parallel; **v1.0 messaging** waits on gates 1–2.

---

## Related

- [release-readiness.md](./release-readiness.md) — current recommendation
- [release-blockers.md](./release-blockers.md) — RB-007
- [scope-control.md](./scope-control.md) — current engineering phase policy
- [enterprise-ops-stable-promotion.md](./enterprise-ops-stable-promotion.md) — runbook
- [product-strategy.md](./product-strategy.md) — positioning
