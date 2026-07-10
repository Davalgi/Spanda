# Repository organization recommendations

**Purpose:** Improve discoverability across `docs/`, `examples/`, `packages/`, `crates/`,
`website/`, and `.github/` **without deleting content**. This audit supports the [product
roadmap](../ROADMAP.md).

**Status:** Recommendations only — physical moves are optional follow-up PRs.

---

## Current state

| Directory | Role | Discoverability gap |
|-----------|------|---------------------|
| `docs/` | 270+ markdown guides, mostly flat | Hard to find pillar vs blueprint vs deep-dive |
| `examples/` | `basics/`, `showcase/`, `solutions/`, `end_to_end/`, `features/` | No pillar tags in README |
| `packages/registry/` | 91 official packages | Duplicate mirror in `registry/` and `crates/spanda-cli/bundled-registry/` |
| `crates/` | 80+ workspace crates | Good `crates/README.md`; missing pillar column |
| `website/` | Static landing + solutions | Minimal nav; no roadmap page |
| `.github/` | CI workflows | Smoke scripts not indexed by pillar |

---

## Recommended grouping

### `docs/` — pillar-oriented index

**Keep existing files.** Add navigation layers:

```
docs/
  README.md                    # Full index (existing)
  pillars/                     # NEW — README per pillar linking existing guides
    language/README.md
    compiler-runtime/README.md
    verification/README.md
    device-fleet/README.md
    security/README.md
    operations/README.md
    developer/README.md
    packages/README.md
  solutions/                   # Existing — expand per-industry .md
  overview/                    # Existing — link to ROADMAP.md pillars
  roadmap.md                   # Redirect → ../ROADMAP.md
  *-roadmap.md                 # Deep dives (unchanged)
```

**Action:** Create `docs/pillars/*/README.md` as **link hubs** — no file moves required initially.

### `examples/` — tag by pillar and blueprint

| Folder | Pillar | Blueprint |
|--------|--------|-----------|
| `basics/` | Language, Developer | Research & Education |
| `showcase/` | Verification, Device & Fleet | — |
| `solutions/adas/` | Verification, Device & Fleet | ADAS |
| `solutions/spatial-computing/` | Device & Fleet, Operations | SAR, Healthcare, Spatial HRI |
| `end_to_end/warehouse_delivery/` | Device & Fleet | Warehouse |
| `end_to_end/pick_and_place_cell/` | Verification | Smart Factory |
| `security/` | Security | Defense |
| `showcase/compliance/` | Security, Verification | Critical Infrastructure |

**Action:** Extend [examples/README.md](../examples/README.md) with pillar/blueprint columns.

### `packages/` — single registry source of truth

| Path | Recommendation |
|------|----------------|
| `packages/registry/` | **Authoritative** package sources |
| `registry/` | Document as publish mirror / CDN index |
| `crates/spanda-cli/bundled-registry/` | Document as CLI bundle snapshot |

**Action:** Add `packages/registry/README.md` explaining the three paths and update flow.

### `crates/` — pillar map

Extend [crates/README.md](../crates/README.md) with a pillar column:

| Pillar | Representative crates |
|--------|-------------------------|
| Language | `spanda-lexer`, `spanda-parser`, `spanda-ast`, `spanda-typecheck` |
| Compiler & Runtime | `spanda-interpreter`, `spanda-driver`, `spanda-llvm`, `spanda-wasm` |
| Verification | `spanda-hardware`, `spanda-readiness`, `spanda-assurance`, `spanda-contract`, `spanda-explain` |
| Device & Fleet | `spanda-config`, `spanda-fleet`, `spanda-ota`, `spanda-connectivity` |
| Security | `spanda-security`, `spanda-tamper`, `spanda-compliance`, `spanda-threat` |
| Operations | `spanda-api`, `spanda-ops`, `spanda-telemetry-store` |
| Developer | `spanda-cli`, `spanda-dap`, `spanda-docs` |
| Packages | `spanda-package`, `spanda-providers` |

### `website/` — product site sections

| Page | Content |
|------|---------|
| `index.html` | Hero + Platform Pillars grid |
| `platform.html` | Product family diagram |
| `solutions.html` | All 14 blueprints (existing ADAS expanded) |
| `roadmap.html` | Timeline + link to ROADMAP.md |
| `architecture.html` | Link to docs/architecture.md |
| `control-center.html` | Control Center overview |
| `docs.html` | Documentation hub |
| `examples.html` | Example ladder |
| `community.html` | Contributing, GitHub, conduct |

**Deploy:** unchanged — Cloudflare Pages publish directory `website/`.

### `.github/` — gate index

Create `scripts/gates/README.md`:

| Script | Pillar / Blueprint |
|--------|-------------------|
| `differentiation_smoke.sh` | Verification |
| `enterprise_ops_smoke.sh` | Operations |
| `adas_smoke.sh` | ADAS blueprint |
| `field_soak_gate.sh` | Operations (Stable promotion) |

---

## Duplication to document (not delete)

| Duplication | Resolution |
|-------------|------------|
| `docs/architecture.md` vs `docs/spanda-architecture.md` | Cross-link; consider merging in future PR |
| `docs/overview/platform-components.md` vs `docs/platform-overview.md` | Overview = navigation; platform-overview = depth |
| Multiple `*-roadmap.md` files | Keep as pillar deep-dives; `ROADMAP.md` is the index |
| Bundled examples in `crates/spanda-cli/bundled-examples/` | Document sync with `examples/` in CONTRIBUTING |

---

## Migration phases (optional)

| Phase | Effort | Action |
|-------|--------|--------|
| **A** (done) | Low | `ROADMAP.md`, README nav, website nav, redirect `docs/roadmap.md` |
| **B** (done) | Low | `docs/pillars/*/README.md` link hubs |
| **C** (done) | Medium | `examples/README.md` pillar tags; `scripts/gates/README.md`; `packages/registry/README.md` |
| **D** (partial) | Medium | `docs/solutions/{agriculture,maritime,environmental-monitoring}.md` — remaining planned blueprints optional |
| **E** | High | Physical `docs/` subdirectory moves with redirect stubs |

**Principle:** Prefer link hubs and indexes over mass file moves in a single PR.

---

## Success criteria

After organization:

1. A new visitor reads **README → ROADMAP.md** and understands the product family
2. Any feature is findable via **pillar** or **blueprint** within two clicks
3. Core vs package vs provider ownership is explicit
4. No content deleted — only classified and cross-linked

See also: [roadmap-migration.md](./roadmap-migration.md) · [ROADMAP.md](../ROADMAP.md)
