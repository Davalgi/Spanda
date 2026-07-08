# Roadmap & Codebase Gap Audit (June 2026)

**Audit date:** 2026-06-24 (refreshed)  
**Release line:** v0.4.0 (tagged 2026-06-22)  
**Scope:** All roadmap documents vs. implementation, registry, CI, and TypeScript mirror.

Canonical status for ongoing release decisions remains [`feature-status.md`](./feature-status.md).
This file captures point-in-time audit findings and closure tracking.

**Superseded counts (2026-07-08):** **89** registry packages; SDK **0.5.5** published (workspace line
**0.5.6**); workspace/desktop **v0.6.3**.

---

## Executive summary

The **codebase is ahead of the v0.4.0 tag** on `main`: mission continuity runtime, telemetry
OTLP/fleet aggregation, and web Operations/Telemetry panels shipped after the tag. **v0.5 P0 golden
paths are 4/5 complete** (only VS Code Marketplace publish blocked on `VSCE_PAT`). **Differentiation
NOW** items have topic guides and architecture specs but **no implementation crates yet** — that is
the primary engineering focus for v0.5 after Marketplace.

**Registry:** 38 packages indexed consistently (`packages/registry/`, `registry/packages/`,
`registry/index.json`).

---

## Closure status

### Closed since 2026-06-22 audit

| Finding | Status |
|---------|--------|
| Registry index lag (20 → 29 packages) | **Closed** — now **38** packages, all indexed |
| `spanda demo rover` missing bundled `autonomous_rover` | **Closed** |
| No `phase30_gaps.rs` | **Closed** |
| README / feature-status version narrative drift | **Closed** — v0.4.0 aligned |
| Phase 25 “In progress” in lean-core roadmap | **Closed** — Complete ✓ (Marketplace partial) |
| Native deploy tier mismatch (Stable vs Experimental) | **Closed** |
| Phase 27+ missing from `spanda-language.md` | **Closed** |
| Continuity runtime experimental → stable | **Closed** — promoted on `main` |
| Telemetry OTLP push/serve/fleet-push | **Closed** — shipped; roadmap updated |
| Web Operations + Telemetry panels | **Closed** — `packages/web` |
| Differentiation roadmap + topic guides | **Closed** — docs only; code **Planned** |
| v0.5 milestone missing from `roadmap.md` | **Closed** — section added 2026-06-24 |

### Still open

| Finding | Priority | Notes |
|---------|----------|-------|
| VS Code Marketplace publish | **P0** | CI + `release.yml` ready; needs maintainer `VSCE_PAT` |
| Differentiation NOW implementation | **P0** (post-Marketplace) | No `spanda-contract`, `spanda-explain`, `spanda-decision` crates |
| `spanda demo differentiation` + smoke script | **P0** | Exit criteria defined; not implemented |
| `spanda-reference.md` Phase 27+ keywords | P1 | Regeneration deferred |
| TypeScript CLI `--verification-json` | P2 | LSP uses native CLI |
| TS integration tests for Phase 27+ syntax | P2 | Rust `phase*_gaps.rs` is authoritative |
| Per-protocol IoT standalone `.sd` examples | P2 | Only `modbus_dispatch.sd` today |
| `agent_can_deny.sd` in golden/CI smoke | P2 | Runtime denial not in smoke |
| Hardware adapter trait codegen | Future | Correctly deferred |
| Twin cloud SaaS product | Future | Golden-path upload script only |
| LLVM as primary runtime | Future | Interpreter remains LTS |

---

## Version roadmap vs codebase

### v0.4.0 tag (2026-06-22)

| Claimed | Code reality | Gap |
|---------|--------------|-----|
| `spanda deploy --target native` | `compile_native` in CLI | **Experimental** — clang required; interpreter primary |
| LLVM golden paths | `scripts/llvm_golden_path.sh`, CI | **Implemented** (experimental) |
| `spanda ros2 check` | `ros2_cli.rs` | **Implemented** |
| Distributed fleet | HTTP agents + mesh | **Experimental** |
| 38 registry packages | index + tarballs | **Aligned** |

### Post-v0.4.0 on `main` (toward v0.5)

| Area | Shipped | Roadmap tier |
|------|---------|--------------|
| Continuity runtime (modes, checkpoints, auto-trigger, swarm `--failed`) | ✓ | **Stable** |
| Fleet mesh continuity relay | ✓ | **Stable** |
| Telemetry OTLP push/serve, fleet-push, sessions | ✓ | **Stable** |
| Web Operations panel + WASM telemetry | ✓ | **Experimental** |
| Differentiation topic guides (15 areas) | ✓ docs | **Planned** code |

### v0.5 beta P0 (tier-3-priority-plan)

| # | Item | Status |
|---|------|--------|
| 1 | VS Code Marketplace | **Partial** — only blocker |
| 2 | Killer demo | **Complete** |
| 3 | Live AI path | **Complete** |
| 4 | ROS2 golden path | **Complete** |
| 5 | Hosted registry | **Complete** (38 packages) |

### v0.5 beta P1 (adoption enablers)

All **Complete** per [tier-3-priority-plan.md](./tier-3-priority-plan.md): CI verify guide, PyO3
golden path, LSP deploy hints, showcase trim, adoption quickstart.

### Differentiation NOW (v0.5+ engineering)

| Item | Docs | Crate / CLI | Status |
|------|------|-------------|--------|
| Mission Contracts | [mission-contracts.md](./mission-contracts.md) | `spanda-contract` | **Planned** |
| Explainability | [explainability.md](./explainability.md) | `spanda-explain` | **Planned** |
| Decision Audit Trail | [decision-audit-trail.md](./decision-audit-trail.md) | `spanda-decision` | **Planned** |
| Safety Coverage | [safety-coverage.md](./safety-coverage.md) | extends `spanda-readiness` | **Planned** |
| Recovery Coverage | [recovery-coverage.md](./recovery-coverage.md) | extends `spanda-assurance` | **Planned** |

### v1.0 / Future (unchanged)

| Item | Code | Gap |
|------|------|-----|
| LLVM as primary runtime | Interpreter default | **PLANNED** |
| Self-hosting compiler (full) | `examples/self_host/` bootstrap | **BOOTSTRAP ONLY** |
| Production verify on 5+ profiles | Partial matrix | **PLANNED** |
| Platform maturity Phase A (`spanda graph`, gates, package trust) | Topic guides only | **PLANNED** |

---

## Lean-core Phases 1–35

Phases **27–35** remain backed by code and `phase*_gaps.rs` tests. Phases **25–26** (v0.5 P0/P1
golden paths) are **complete** except Marketplace publish.

---

## Registry & packages

| Location | Count |
|----------|-------|
| `packages/registry/` scaffolds | **38** |
| `registry/packages/` tarballs | **38** |
| `registry/index.json` entries | **38** |

Rebuild: `./scripts/build-registry.sh`

---

## Documentation drift (resolved this audit)

| Issue | Status |
|-------|--------|
| `roadmap.md` missing v0.5 milestone | **Closed** |
| Differentiation NOW table implied crates exist | **Closed** — Docs/Code columns added |
| Telemetry OTLP/fleet not in roadmap Simulation row | **Closed** |
| Web playground capabilities understated | **Closed** |
| Audit doc stale at 29 packages | **Closed** |
| `feature-status` telemetry row missing OTLP/fleet | **Closed** |

---

## Recommended next actions

1. **Set `VSCE_PAT`** and publish VS Code extension — unblocks v0.5 beta tag.
2. **Implement differentiation NOW** in order: Mission Contracts → Explainability → Decision Audit
   Trail → Safety/Recovery Coverage.
3. **Add** `scripts/differentiation_smoke.sh` and `spanda demo differentiation` when first CLI lands.
4. **Regenerate** `spanda-reference.md` when differentiation syntax stabilizes.

---

## Related

- [`roadmap.md`](./roadmap.md) — version plan (updated 2026-06-24)
- [`feature-status.md`](./feature-status.md) — Stable / Experimental / Planned matrix
- [`differentiation-roadmap.md`](./differentiation-roadmap.md) — signature capabilities detail
- [`tier-3-priority-plan.md`](./tier-3-priority-plan.md) — P0–P4 ordering
- [`lean-core-roadmap.md`](./lean-core-roadmap.md) — Phases 1–35 detail
