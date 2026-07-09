# Workspace release v0.7.0 — prep checklist

**Status:** **Tagged and released** — `v0.7.0` pushed **2026-07-09** ([GitHub Release](https://github.com/Davalgi/Spanda/releases/tag/v0.7.0)).

| Stream | Version | Tag |
|--------|---------|-----|
| **Workspace / CLI** | **0.7.0** | `v0.7.0` |
| **SDK** | **0.5.9** | `crates-sdk-v0.5.9`, `sdk-python-v0.5.9`, `npm-sdk-v0.5.9` |
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

Post-tag on `main` (documented under `[Unreleased]`): Python gRPC client, live transport discovery
sources, richer Entity Mesh Control Center UX — shipped in SDK **0.5.9** and workspace commits after
the tag.

Full notes: [CHANGELOG.md](../CHANGELOG.md#070---2026-07-08).

---

## Pre-tag verification (completed 2026-07-09)

```bash
./scripts/ci-fast.sh          # Rust gates pass; local pip env may skip Python SDK step
./scripts/entity_mesh_smoke.sh
./scripts/verify_sdk_publish_ready.sh
cargo test -p spanda-entity-mesh --quiet
```

---

## Tag and release (maintainers)

Completed:

```bash
git tag -a v0.7.0 -m "Spanda v0.7.0 — Entity Mesh, Cognitive & Resilience, tiered CI (evaluation/beta)."
git push origin main v0.7.0
```

**Release** workflow (`cargo-dist`) builds installers and publishes the GitHub Release on tag push.

**Do not** bump SDK or desktop tags for workspace-only releases.

---

## Post-release doc sync

- [x] [release-readiness.md](./release-readiness.md) — snapshot updated
- [x] [ROADMAP.md](../ROADMAP.md) — v0.7.0 tagged
- [x] [feature-status.md](./feature-status.md) — SDK **0.5.9** pins

---

## Organizational gates (unchanged)

v0.7.0 remains **evaluation / beta**. v1.0 requires field soak, security audit, and Stable-tier
pillars per [organizational-gates.md](./organizational-gates.md).

Entity Mesh Stable promotion uses the **dedicated** pilot clock (started **2026-07-09**), not the
enterprise-ops soak alone.
