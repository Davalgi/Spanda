# Release Blockers

Tracked defects and gaps that affect a credible Spanda release.
Update this file when blockers are found, fixed, or reclassified.

**Severity**

| Level | Meaning |
|-------|---------|
| **P0** | Blocks release |
| **P1** | Should fix before release |
| **P2** | Acceptable known issue (document) |
| **P3** | Backlog |

| ID | Issue | Area | Severity | Reproduction | Owner | Status |
|----|-------|------|----------|--------------|-------|--------|
| RB-001 | `spanda demo rover` failed when monorepo registry was shadowed by incomplete CLI bundle | Packages / demos | P0 | `spanda demo rover` without `SPANDA_REGISTRY_URL` | — | **Fixed** — prefer monorepo `registry/` and on-disk `packages/registry/` lookup |
| RB-002 | README `entity list` / `entity graph` omitted required `--config` | Docs / entity CLI | P1 | `spanda entity list` at repo root | — | **Fixed** — README documents warehouse fixture; CLI error includes fix |
| RB-003 | Bare `assure` / `diagnose` / `readiness` printed opaque "Missing file path" | CLI diagnostics | P2 | `spanda assure` with no args | — | **Fixed** — structured what/why/where/fix messages |
| RB-004 | `spanda recovery plan … --failure gps` returns `Plans (0)` while explain succeeds | Recovery orchestrator | P1 | `spanda recovery plan examples/showcase/self_healing/rover.sd --failure gps` | — | Open — plan path does not emit strategies for GPS failure |
| RB-005 | `LOCAL_REGISTRY` stub omits packages present under `packages/registry/` | Package manager | P2 | `spanda install` without monorepo index for packages only on disk | — | Mitigated by on-disk fallback; stub still warns |
| RB-006 | Many features labeled **Stable** while default AI/IoT paths are mock-backed | Feature status | P1 | Review `docs/feature-status.md` agent/AI and live transport rows | — | Open — audit in progress; mock defaults must not read as production-ready |
| RB-007 | Organizational gates (field soak, third-party security audit) incomplete | Enterprise ops / blueprints | P2 | Promotion gates skip soak/audit in CI | — | Documented; not code blockers |
| RB-008 | Recovery plan empty plans may mislead operators | Recovery / docs | P2 | Same as RB-004 | — | Open — document limitation until RB-004 fixed |
| RB-009 | Cross-interface consistency not historically enforced in CI | QA / SDKs | P1 | Manual CLI vs REST vs SDK drift | — | **Mitigated** — `scripts/cross_interface_consistency.sh` + CI job |
| RB-010 | README commands lacked automated smoke/golden coverage | QA | P0 | Broken README commands undetected | — | **Fixed** — `tests/readme_commands/` harness |
| RB-011 | v3 decision signature verifies embedded `signing_payload` only; outer `decision` fields can diverge | Decision security | P1 | Mutate `decision` on a signed v3 payload; `verify_v3_decision_signature` still returns Ok | — | Open — bind outer fields to envelope or reject mismatch |
| RB-012 | Parser `previous`/`advance` underflow panic on empty/start position | Parser | P0 | Property test seeds (`robot`, `{{{{`, empty-ish streams) | — | **Fixed** — use `saturating_sub(1)` |

## How to add a blocker

1. Assign the next `RB-NNN` id.
2. Include reproduction steps that fail on `main`.
3. Set severity honestly (mock-backed Stable claims are at least P1).
4. Link related tests or docs when fixed.

## Exit criteria for release

- No open **P0** items
- All **P1** items fixed or explicitly accepted with user-visible docs
- README command smoke + golden tests green in CI
- Security regression suite green
- [release-readiness.md](./release-readiness.md) recommendation is **Go** or **Go with documented limitations**
