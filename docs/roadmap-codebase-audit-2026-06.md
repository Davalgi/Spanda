# Roadmap & Codebase Gap Audit

**Last audit:** 2026-07-09  
**Release line:** **v0.7.0** (evaluation/beta, tagged 2026-07-09)  
**Scope:** Roadmap documents vs. implementation, registry, CI, and TypeScript mirror.

Canonical status for ongoing release decisions: [`feature-status.md`](./feature-status.md).  
This file tracks audit findings and closure history (supersedes the June 2026 snapshot).

**Current counts:** **91** registry packages; SDK **0.5.9** published; workspace **v0.7.0**; desktop
**v0.6.3**; gRPC proto **1.0.15** (**174** RPCs).

---

## Executive summary (2026-07-09)

The **v0.7.0** evaluation/beta line ships with CI-backed quality gates and honest stability labels.
**Differentiation NOW**, enterprise ops (E1–E4), solution blueprints (ADAS, Smart Spaces, HRI), and
platform maturity Phases A–D are **Stable** in code. **Autonomous Entity Mesh** promoted to **Stable**
(implementation) on 2026-07-09; organizational field pilot remains in progress.

**Open engineering (Next horizon):** VS Code Marketplace (`VSCE_PAT`), LLVM native codegen
hardening, live vehicle I/O bridges (LIN/UDS/V2X env hooks shipped), cognitive live fusion pipeline
(`SPANDA_LIVE_FUSION_SENSORS=1`).

**Open organizational (v1.0):** [#51](https://github.com/Davalgi/Spanda/issues/51) — 30-day field
soak + third-party security audit.

---

## Closure status

### Closed since June 2026 audit

| Finding | Status |
|---------|--------|
| Differentiation NOW crates (`spanda-contract`, `spanda-explain`, `spanda-decision`, coverage CLIs) | **Closed** — **Stable** |
| `spanda demo differentiation` + `differentiation_smoke.sh` | **Closed** |
| Platform maturity Phases A–D | **Closed** — **Stable** |
| Entity Mesh implementation (CLI, REST, gRPC, SDK, Control Center, smoke) | **Closed** — **Stable** tier |
| Registry package count (38 → 91) | **Closed** — aligned |
| OIDC admin test isolation (`admin_oauth_tests`) | **Closed** — temp state dir per test |
| ADAS live vehicle I/O (LIN/UDS/V2X `SPANDA_*_CMD` bridges) | **Closed** — experimental env path |
| Cognitive live fusion supplier (`SPANDA_LIVE_FUSION_SENSORS=1`) | **Closed** — Beta path |

### Still open

| Finding | Priority | Notes |
|---------|----------|-------|
| VS Code Marketplace publish | **P2** | CI ready; needs maintainer `VSCE_PAT` |
| Organizational gates RB-007 / #51 | **P2** | Field soak + third-party audit |
| `spanda-reference.md` Phase 27+ keywords | P1 | Regeneration deferred |
| LLVM as primary runtime | **Future** | Interpreter LTS; `compile-native` experimental |
| Self-hosting compiler (full) | **Future** | Bootstrap only |
| Hosted twin cloud SaaS product | **Future** | OSS `/v1/twins/*` Stable |
| Hardware adapter trait codegen | **Future** | Correctly deferred |
| Per-blueprint organizational soaks | P2 | Scripts exist; clocks separate from enterprise ops |

---

## Version roadmap vs codebase (v0.7.0)

| Area | Tier | Notes |
|------|------|-------|
| Language core, verify, sim, replay | **Stable** | CI Fast + Integration |
| Differentiation NOW + analytics pillars | **Stable** | Golden smokes |
| Enterprise ops E1–E4 | **Stable** code; organizational soak open | |
| Solution blueprints (ADAS, Smart Spaces, HRI) | **Stable** code; per-blueprint soak open | |
| Autonomous Entity Mesh | **Stable** code; Entity Mesh field pilot open | |
| Native deploy / LLVM | **Experimental** | `llvm_golden_path.sh` + rover example |
| AI / default transport | **Mock-backed** | Live paths env-gated |
| Cognitive & Resilience | **Beta** | Live fusion supplier optional |

---

## Recommended next actions

1. Complete organizational gates ([#51](https://github.com/Davalgi/Spanda/issues/51)) for v1.0 messaging.
2. Publish VS Code extension when `VSCE_PAT` is available.
3. Expand LLVM golden paths for additional HAL profiles; keep interpreter as LTS default until v1.0 table met.
4. Regenerate `spanda-reference.md` when syntax stabilizes.

---

## Related

- [`ROADMAP.md`](../ROADMAP.md) — product roadmap
- [`feature-status.md`](./feature-status.md) — stability matrix
- [`release-blockers.md`](./release-blockers.md) — RB-007
- [`organizational-gates.md`](./organizational-gates.md) — v1.0 checklist
