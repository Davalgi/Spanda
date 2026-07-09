# Workspace release v0.7.0 — prep checklist

**Status:** Version bumped on `main`; tag **not yet pushed**.

| Stream | Version | Tag |
|--------|---------|-----|
| **Workspace / CLI** | **0.7.0** | `v0.7.0` |
| **SDK** (unchanged) | **0.5.8** | `crates-sdk-v0.5.8`, etc. |
| **Desktop** (unchanged) | **0.6.3** | `desktop-v0.6.3` |
| **gRPC proto** | **1.0.15** (**174** RPCs) | pin via `GET /v1/version` |

---

## Shipped in 0.7.0 (highlights)

- **Autonomous Entity Mesh (Experimental)** — crate, CLI, REST/gRPC, SDK, Control Center tab
- **Entity Mesh field pilot** — dedicated soak clock ([entity-mesh-field-pilot.md](./entity-mesh-field-pilot.md))
- **Cognitive & Resilience Architecture** — eleven domains, REST/gRPC, domain SDK clients
- **Tiered CI** — Fast / Integration / Nightly
- **Control Center auth hardening** — hashed API keys, session JWTs, OIDC SSO
- **Wasm32 dependency graph** — optional HTTP features for wasm targets

Full notes: [CHANGELOG.md](../CHANGELOG.md#070---2026-07-08).

---

## Pre-tag verification

```bash
./scripts/ci-fast.sh
./scripts/entity_mesh_smoke.sh
./scripts/verify_sdk_publish_ready.sh
cargo test -p spanda-entity-mesh --quiet
```

---

## Tag and release (maintainers)

```bash
# After merge to main and CI green:
git tag v0.7.0
git push origin v0.7.0
```

Or merge a PR labeled `release:minor` to trigger **Actions → Bump version** (workspace stream only).

GitHub Release: attach built CLI artifacts per [CONTRIBUTING.md](../CONTRIBUTING.md#releases).

**Do not** bump SDK or desktop tags for this workspace release.

---

## Post-release doc sync

- [ ] [release-readiness.md](./release-readiness.md) — update snapshot date
- [ ] [ROADMAP.md](../ROADMAP.md) — mark v0.7.0 tagged
- [ ] [feature-status.md](./feature-status.md) — `as of v0.7.0` header

---

## Organizational gates (unchanged)

v0.7.0 remains **evaluation / beta**. v1.0 requires field soak, security audit, and Stable-tier
pillars per [organizational-gates.md](./organizational-gates.md).

Entity Mesh Stable promotion uses the **dedicated** pilot clock (started **2026-07-09**), not the
enterprise-ops soak alone.
