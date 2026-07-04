# Release Readiness Report

Snapshot for the **release hardening** phase. Update when quality gates change.

**Phase policy:** [scope-control.md](./scope-control.md)  
**Blockers:** [release-blockers.md](./release-blockers.md)  
**Feature labels:** [feature-status.md](./feature-status.md)

## Build status

| Gate | Status | Notes |
|------|--------|-------|
| `cargo fmt --all -- --check` | Required in CI | `.github/workflows/ci.yml` rust job |
| `cargo clippy --workspace -- -D warnings` | Required in CI | Same |
| `cargo build -p spanda --release` | Required in CI | Used by smoke jobs |
| `cargo doc --workspace --no-deps` | Required in CI | Same |

## Test status

| Suite | Status | Notes |
|-------|--------|-------|
| `cargo test --workspace` | Required in CI | Includes security + property regressions |
| `npm test` (repo TypeScript) | Required in CI | typescript job |
| Python SDK (`sdk/python`) | Required in CI | pytest |
| TypeScript SDK (`sdk/typescript`) | Required in CI | npm test |
| README command smoke | Required in CI | `tests/readme_commands/run.sh` |
| Golden-output flagship commands | Required in CI | `tests/readme_commands/run.sh --golden` |
| Cross-interface consistency | Required in CI | `scripts/cross_interface_consistency.sh` |
| Security regressions | Required in CI | plugin / package / decision / recovery tests |
| Property-style parsers | Required in CI | parser, manifest, config, policy, capability |

## Docs status

| Item | Status |
|------|--------|
| README commands match runnable invocations | Hardened (entity `--config`, demos) |
| Feature stability labels | Audit ongoing — see honesty rules in feature-status |
| Release blockers tracked | [release-blockers.md](./release-blockers.md) |
| Scope control published | [scope-control.md](./scope-control.md) |

## Demo status

| Demo | Status |
|------|--------|
| `spanda demo rover` | Passes after monorepo registry fix |
| `spanda demo assurance` | Passes (golden) |
| `spanda demo self-healing` | Passes (golden) |
| `spanda demo continuity` | Passes (smoke) |

## SDK status

| SDK | Status |
|-----|--------|
| Rust `spanda-sdk` | Published stream; entity/readiness clients exercised in entity smoke |
| Python `spanda-sdk` | pytest + cross-interface probe |
| TypeScript `@davalgi-spanda/sdk` | unit tests + cross-interface probe |

## API status

| Surface | Status |
|---------|--------|
| REST `/v1/entities/*` | Covered by entity + cross-interface smoke |
| REST `/v1/programs/readiness` | Cross-interface probe |
| REST `/v1/recovery/*` | Partial — plan empty-plans issue (RB-004) |
| gRPC | Existing API tests; not re-audited in this pass |

## Feature status (honesty)

| Tier | Rule |
|------|------|
| **Stable** | Tested, non-mock default path, not demo-only |
| **Beta** | Usable with known limitations |
| **Experimental** | Works with caveats / optional live backends |
| **Preview** | Early API, may change |
| **Stubbed** | Syntax/API without full integration |
| **Mock-backed** | Default path uses mocks/simulators |
| **Planned** | Not implemented |
| **Deprecated** | Replaced |

Mock-backed, demo-only, docs-only, untested, and simulated-only features must **not** be labeled Stable.

## Security status

| Control | Coverage |
|---------|----------|
| Plugin trust / blocked install | `spanda-plugin` security regression |
| Plugin sandbox defaults | Same |
| Package signatures / tamper | `spanda-package` security regression |
| Decision replay / policy tamper / fake coordinator | `spanda-decision` security regression |
| Entity takeover permission | Same |
| Recovery privilege / secret leakage | `spanda-recovery` security regression |

## Known limitations

- Default AI providers are **mock-backed** unless live keys/env are set.
- Live IoT / MQTT / ROS2 paths are optional and environment-gated.
- `LOCAL_REGISTRY` stub is incomplete; monorepo uses `registry/index.json` + on-disk packages.
- Recovery orchestrator `plan` may return zero plans for some failures (RB-004).
- Organizational soak and third-party audit gates remain open (RB-007).

## Release blockers summary

- **Open P0:** none (after RB-001 / RB-010 / RB-012)
- **Open P1:** RB-004 (recovery plan empty), RB-006 (stability label honesty), RB-011 (v3 decision outer-field binding)
- **Open P2:** RB-005, RB-007, RB-008

## Recommendation

**Go with documented limitations** for an evaluation / beta release, provided:

1. CI green including README smoke, goldens, cross-interface, and security regressions
2. RB-004 either fixed or clearly documented in recovery docs and release notes
3. Feature-status rows for mock-default AI/IoT are not marketed as production Stable

**Do not** claim full production readiness until P1 items are closed and organizational security audit (RB-007) is accepted.
