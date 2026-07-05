# Release Readiness Report

Snapshot for **v0.6.3** (evaluation / beta). Update when quality gates or phase policy change.

**Phase policy:** [scope-control.md](./scope-control.md) (Next horizon)  
**Organizational path to v1.0:** [organizational-gates.md](./organizational-gates.md)  
**Blockers:** [release-blockers.md](./release-blockers.md)  
**Feature labels:** [feature-status.md](./feature-status.md)

## Build status

| Gate | Status | Notes |
|------|--------|-------|
| **CI Fast** (`lint-rust`, `test-rust`, `test-typescript`, SDK tests, `cross-interface`) | Required on PRs | [.github/workflows/ci-fast.yml](../.github/workflows/ci-fast.yml) |
| **CI Integration** (smokes, golden paths, LSP, WASM) | Required on `main` after merge | [.github/workflows/ci-integration.yml](../.github/workflows/ci-integration.yml) |
| **CI Nightly** (promotion gates, ROS2, audit, desktop) | Signal only | [.github/workflows/ci-nightly.yml](../.github/workflows/ci-nightly.yml) |
| Local parity | `./scripts/ci-fast.sh` | [ci-architecture.md](./ci-architecture.md) |

## Build status (compile gates)

| Gate | Status | Notes |
|------|--------|-------|
| `cargo fmt --all -- --check` | CI Fast `lint-rust` | |
| `cargo clippy --workspace -- -D warnings` | CI Fast `lint-rust` | |
| `cargo build -p spanda --release` | CI Fast `build-spanda` (once per workflow) | Artifact reused in integration |
| `cargo doc --workspace --no-deps` | CI Integration `docs-build` | |

## Test status

| Suite | Status | Notes |
|-------|--------|-------|
| `cargo test --workspace` | CI Fast `test-rust` | |
| `npm test` (repo TypeScript) | CI Fast `test-typescript` | |
| Python SDK (`sdk/python`) | CI Fast `test-python-sdk` | pytest |
| TypeScript SDK (`sdk/typescript`) | CI Fast `test-ts-sdk` | npm test |
| Cross-interface consistency | CI Fast + Integration | `scripts/cross_interface_consistency.sh` |
| README command smoke + golden | CI Integration `core-smokes` | `tests/readme_commands/run.sh` |
| Cognitive & Resilience smoke | CI Integration | `scripts/cognitive_resilience_smoke.sh` |
| Security regressions | CI Integration `release-hardening` | plugin / package / decision / recovery tests |
| Property-style parsers | CI Integration `release-hardening` | parser, manifest, config, policy, capability |

## Docs status

| Item | Status |
|------|--------|
| README commands match runnable invocations | Hardened (entity `--config`, demos) |
| Feature stability labels | Audit ongoing — see honesty rules in feature-status |
| Release blockers tracked | [release-blockers.md](./release-blockers.md) |
| Scope control published | [scope-control.md](./scope-control.md) — Next horizon phase |
| Organizational gates documented | [organizational-gates.md](./organizational-gates.md) |
| Public v0.6.3 positioning | Evaluation / beta — README + [known-limitations.md](./known-limitations.md) |

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
| REST `/v1/recovery/*` | Covered by cross-interface smoke (CLI vs REST vs gRPC plan parity) |
| gRPC Control Center | Covered by `cross_interface_live` probe (health + recovery plan) |

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

- Default AI providers are **mock-backed** unless live keys/env are set (`SPANDA_LIVE_AI=0` forces mock). Runtime emits one-time `[spanda]` notices; see [known-limitations.md](./known-limitations.md).
- Live IoT / MQTT / ROS2 paths are optional and environment-gated; in-memory transport is the default.
- Monorepo uses `registry/index.json` + on-disk `packages/registry/`; compile-time `LOCAL_REGISTRY` stub is incomplete but no longer blocks install.
- Organizational soak and third-party audit gates remain open (RB-007 / [#51](https://github.com/Davalgi/Spanda/issues/51)).

## Release blockers summary

Tracked in [release-blockers.md](./release-blockers.md) and GitHub issues labeled [`release-blocker`](https://github.com/Davalgi/Spanda/issues?q=is%3Aissue+label%3Arelease-blocker).

- **Open P0:** none
- **Open P1:** none (RB-004, RB-006, RB-011 fixed or mitigated in release-hardening follow-up)
- **Open P2:** [#51](https://github.com/Davalgi/Spanda/issues/51) RB-007 (organizational field soak / third-party audit)

## Recommendation

**Go with documented limitations** — evaluation / beta release **shipped** as workspace **v0.6.3**.

1. Release-hardening suites are in CI (README smoke, goldens, cross-interface, security regressions)
2. Feature-status honesty audit is respected (mock-default AI is **Mock-backed**, not production Stable alone)
3. Organizational field soak and third-party security audit ([#51](https://github.com/Davalgi/Spanda/issues/51)) remain explicitly out of scope for this code release

**Do not** claim full production readiness until RB-007 organizational gates are accepted.
