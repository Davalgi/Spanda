# Release Blockers

Tracked defects and gaps that affect a credible Spanda release.
Update this file when blockers are found, fixed, or reclassified.

**GitHub issues:** each blocker has a linked issue labeled `release-blocker` plus severity
(`P0`–`P3`) and `release-hardening`.

Filter open blockers:

```text
https://github.com/Davalgi/Spanda/issues?q=is%3Aissue+is%3Aopen+label%3Arelease-blocker
```

**Severity**

| Level | Meaning |
|-------|---------|
| **P0** | Blocks release |
| **P1** | Should fix before release |
| **P2** | Acceptable known issue (document) |
| **P3** | Backlog |

| ID | GitHub | Issue | Area | Severity | Reproduction | Owner | Status |
|----|--------|-------|------|----------|--------------|-------|--------|
| RB-001 | [#45](https://github.com/Davalgi/Spanda/issues/45) | `spanda demo rover` failed when monorepo registry was shadowed by incomplete CLI bundle | Packages / demos | P0 | `spanda demo rover` without `SPANDA_REGISTRY_URL` | — | **Fixed** — prefer monorepo `registry/` and on-disk `packages/registry/` lookup |
| RB-002 | [#46](https://github.com/Davalgi/Spanda/issues/46) | README `entity list` / `entity graph` omitted required `--config` | Docs / entity CLI | P1 | `spanda entity list` at repo root | — | **Fixed** — README documents warehouse fixture; CLI error includes fix |
| RB-003 | [#47](https://github.com/Davalgi/Spanda/issues/47) | Bare `assure` / `diagnose` / `readiness` printed opaque "Missing file path" | CLI diagnostics | P2 | `spanda assure` with no args | — | **Fixed** — structured what/why/where/fix messages |
| RB-004 | [#48](https://github.com/Davalgi/Spanda/issues/48) | `spanda recovery plan … --failure gps` returns `Plans (0)` while explain succeeds | Recovery orchestrator | P1 | `spanda recovery plan examples/showcase/self_healing/rover.sd --failure gps` | — | **Fixed** — failure-only plans target program robots; empty plans no longer pass |
| RB-005 | [#49](https://github.com/Davalgi/Spanda/issues/49) | `LOCAL_REGISTRY` stub omits packages present under `packages/registry/` | Package manager | P2 | `spanda install` without monorepo index for packages only on disk | — | **Fixed** — on-disk packages resolve and no longer warn as missing from stub |
| RB-006 | [#50](https://github.com/Davalgi/Spanda/issues/50) | Many features labeled **Stable** while default AI/IoT paths are mock-backed | Feature status | P1 | Review `docs/feature-status.md` agent/AI and live transport rows | — | **Mitigated** — honesty audit section; AI agents Mock-backed; organizational soak still open via RB-007 |
| RB-007 | [#51](https://github.com/Davalgi/Spanda/issues/51) | Organizational gates (field soak, third-party security audit) incomplete | Enterprise ops / blueprints | P2 | Promotion gates skip soak/audit in CI | — | **In progress** — field soak started 2026-06-29; audit prep script available |
| RB-008 | [#52](https://github.com/Davalgi/Spanda/issues/52) | Recovery plan empty plans may mislead operators | Recovery / docs | P2 | Same as RB-004 | — | **Fixed** — empty plans report `Passed: false` with what/why/where/fix text |
| RB-009 | [#53](https://github.com/Davalgi/Spanda/issues/53) | Cross-interface consistency not historically enforced in CI | QA / SDKs | P1 | Manual CLI vs REST vs SDK drift | — | **Mitigated** — `scripts/cross_interface_consistency.sh` + CI Fast/Integration |
| RB-010 | [#54](https://github.com/Davalgi/Spanda/issues/54) | README commands lacked automated smoke/golden coverage | QA | P0 | Broken README commands undetected | — | **Fixed** — `tests/readme_commands/` harness |
| RB-011 | [#55](https://github.com/Davalgi/Spanda/issues/55) | v3 decision signature verifies embedded `signing_payload` only; outer `decision` fields can diverge | Decision security | P1 | Mutate `decision` on a signed v3 payload; `verify_v3_decision_signature` still returns Ok | — | **Fixed** — outer fields must match signing payload before signature verify |
| RB-012 | [#56](https://github.com/Davalgi/Spanda/issues/56) | Parser `previous`/`advance` underflow panic on empty/start position | Parser | P0 | Property test seeds (`robot`, `&#123;&#123;&#123;&#123;`, empty-ish streams) | — | **Fixed** — use `saturating_sub(1)` |

## Open issues (priority)

| Priority | Issues |
|----------|--------|
| **P2** | [#51](https://github.com/Davalgi/Spanda/issues/51) (RB-007) — organizational field soak / third-party audit | Field soak **in progress** (started 2026-06-29) |

## How to add a blocker

1. Assign the next `RB-NNN` id.
2. Create a GitHub issue titled `[RB-NNN] …` with labels `release-blocker`, severity (`P0`–`P3`),
   and `release-hardening`.
3. Include reproduction steps that fail on `main`.
4. Set severity honestly (mock-backed Stable claims are at least P1).
5. Add a row here with the issue link.
6. Link related tests or docs when fixed, and close the GitHub issue.

## Path to v1.0

Code release criteria for v0.6.3 are **met**. v1.0 requires organizational gates — see
[organizational-gates.md](./organizational-gates.md):

| Gate | Status |
|------|--------|
| Field soak (30 days) | Open — `enterprise_ops_field_soak_init.sh` |
| Third-party security audit | Open — `security_audit_prep.sh` + external sign-off |
| RB-007 / [#51](https://github.com/Davalgi/Spanda/issues/51) | Open — P2, documented |

## Exit criteria for release

- No open **P0** items
- All **P1** items fixed or explicitly accepted with user-visible docs
- README command smoke + golden tests green in CI Integration (`core-smokes`)
- Security regression suite green in CI Integration (`release-hardening`)
- CI Fast required checks green on PRs — [ci-architecture.md](./ci-architecture.md)
- [release-readiness.md](./release-readiness.md) recommendation is **Go** or **Go with documented
  limitations**
- v1.0 additionally requires [organizational-gates.md](./organizational-gates.md) checklist (soak +
  audit)
