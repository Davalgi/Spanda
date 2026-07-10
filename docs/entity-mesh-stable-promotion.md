# Autonomous Entity Mesh — Stable Promotion

**Status:** **Stable** (implementation promotion **2026-07-09**; organizational field pilot in progress)

Operational checklist to promote **Autonomous Entity Mesh** from **Experimental** to **Stable**
in `docs/feature-status.md`.

---

## Prerequisites

| Step | Status |
|------|--------|
| `spanda-entity-mesh` crate + security regression tests | ✅ Shipped on `main` |
| CLI `spanda mesh *` | ✅ |
| REST `/v1/mesh/*` + gRPC mesh RPCs (proto **1.0.15+**) | ✅ |
| SDK mesh helpers (Rust + TypeScript + Python) + Rust gRPC client | ✅ |
| Control Center **Entity Mesh** tab with topology graph | ✅ |
| `entity_mesh_smoke.sh` | ✅ CI Integration `entity-mesh-smoke` |
| Examples under `examples/showcase/entity_mesh/` | ✅ |
| Docs ([entity-mesh.md](./entity-mesh.md) + topic guides) | ✅ |

---

## Remaining for Stable

| Step | Status |
|------|--------|
| 30-day Entity Mesh field pilot (`entity_mesh_field_soak_init.sh`) | ⏳ Started **2026-07-09** — [entity-mesh-field-pilot.md](./entity-mesh-field-pilot.md) |
| External security audit sign-off | ⏳ |
| SDK publish with full mesh REST + gRPC surface | ✅ **0.5.9** published — see [SDK publish](#sdk-publish) |
| `docs/feature-status.md` → **Stable** | ✅ Implementation tier (organizational soak open) |

---

## Automated gate

```bash
chmod +x scripts/entity_mesh_stable_promotion_gate.sh

# CI / local implementation checks only:
SPANDA_ENTITY_MESH_SKIP_SOAK=1 SPANDA_ENTITY_MESH_SKIP_AUDIT=1 \
  ./scripts/entity_mesh_stable_promotion_gate.sh

# Full gate (requires elapsed field soak + audit prep artifact):
./scripts/entity_mesh_stable_promotion_gate.sh
```

The gate runs:

1. Field soak clock (optional skip in CI)
2. Security audit prep artifact (optional skip in CI)
3. `cargo test -p spanda-entity-mesh`
4. `scripts/entity_mesh_smoke.sh` (CLI, REST, SDK, gRPC)

CI Nightly job: `entity-mesh-promotion-gate` (soak/audit skipped).

---

## Compatibility rules (must hold at Stable)

- Mesh is **additive** — does not replace transport providers or fleet HTTP mesh relay
- All mesh messages use **secure messaging**
- Takeover/delegation via **Recovery Orchestrator** only
- Coordinator = **communication role only**

See [entity-mesh.md](./entity-mesh.md#compatibility-rules).

---

## SDK publish

| Package | Version | Tag | Registry |
|---------|---------|-----|----------|
| `spanda-sdk` (Rust) | `0.5.9` | `crates-sdk-v0.5.9` | [crates.io](https://crates.io/crates/spanda-sdk) |
| `spanda-sdk` (Python) | `0.5.9` | `sdk-python-v0.5.9` | [PyPI](https://pypi.org/project/spanda-sdk/) |
| `@davalgi-spanda/sdk` (npm) | `0.5.9` | `npm-sdk-v0.5.9` | [npm](https://www.npmjs.com/package/@davalgi-spanda/sdk) |

**0.5.9:** Python gRPC client; **0.5.8:** TypeScript `GrpcClient` with mesh gRPC RPCs. **0.5.7:** REST
`meshGraph`, `meshDiscover`, `meshMergeReport`, `meshSimulatePartition`. Rust
`GrpcClient` mesh RPCs ship with the `grpc` feature on **0.5.6+**.

```bash
./scripts/verify_sdk_publish_ready.sh
./scripts/publish_sdk_release.sh   # tags + push → GitHub Actions publish
```

See [sdk-publishing.md](./sdk-publishing.md).
