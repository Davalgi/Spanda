# CI architecture

Spanda CI is split into **three tiers** so pull requests stay fast while `main` and nightly runs
retain full platform coverage.

| Workflow | File | When | Blocks merge? |
|----------|------|------|---------------|
| **CI Fast** | [.github/workflows/ci-fast.yml](../.github/workflows/ci-fast.yml) | Every PR and push to `main` | **Yes** (required checks) |
| **CI Integration** | [.github/workflows/ci-integration.yml](../.github/workflows/ci-integration.yml) | After CI Fast succeeds on `main` push | **Yes for `main` health** (not individual PRs) |
| **CI Nightly** | [.github/workflows/ci-nightly.yml](../.github/workflows/ci-nightly.yml) | Daily cron + manual dispatch | **No** (signal only) |

**Auto release** (`.github/workflows/auto-release.yml`) waits for **CI Integration** success on
`main`, not nightly.

When the merged PR has a `release:major|minor|patch` label:

1. Bumps **workspace** → tag `vX.Y.Z` → cargo-dist Release.
2. If Control Center paths changed (`scripts/control_center_paths_changed.sh`), bumps **desktop** →
   tag `desktop-vX.Y.Z` → [desktop-release.yml](../.github/workflows/desktop-release.yml).

Manual ad-hoc bumps: **Actions → Bump version** (stream **workspace** or **desktop**). See
[control-center-versioning.md](./control-center-versioning.md).

---

## CI Fast (PR gate ~12–15 min)

Runs on every pull request and every push to `main`.

| Job | Purpose |
|-----|---------|
| `lint-rust` | `cargo fmt`, `clippy -D warnings`, architecture + blueprint validation, registry index verify |
| `test-rust` | `cargo test --workspace` |
| `test-typescript` | `npm test`, `npm run build` |
| `test-python-sdk` | `pytest` in `sdk/python` |
| `test-ts-sdk` | `npm test` in `sdk/typescript` |
| `cross-surface-check` | `scripts/check_cross_surface.sh` |
| `build-spanda` | One release build; uploads `spanda-bin` artifact |
| `cross-interface` | `scripts/cross_interface_consistency.sh` using artifact |
| `docs-validate` | Documentation audit when **only** docs/markdown changed |

**Path filters:** docs-only PRs skip Rust/TS/Python jobs and run `docs-validate` instead. Any change
under `crates/`, `src/`, `sdk/`, workflows, etc. runs the full fast gate.

**Local parity:**

```bash
./scripts/ci-fast.sh
```

Optional pre-push hook (fmt + cross-surface only):

```bash
./scripts/setup-githooks.sh
```

---

## CI Integration (`main` ~25 min)

Triggered by `workflow_run` after **CI Fast** completes successfully on a `main` push.

Builds `spanda` **once**, uploads an artifact, and reuses it across smoke and golden-path jobs (no
per-job release compiles).

Includes: readme smoke + golden output, core smokes, distributed decisions, cognitive resilience,
release hardening security/property tests, key golden paths (robotics, telemetry, twin cloud,
registry, ci-verify, killer demo), solution blueprint smokes, entity model smoke, LSP, WASM, VS Code
extension packaging, and docs/mdBook build.

---

## CI Nightly (~60+ min)

Scheduled at **06:00 UTC** and available via **Actions → CI Nightly → Run workflow**.

Includes: `cargo audit`, promotion gates (Stable hardening scripts with soak/audit skipped in CI),
ROS 2, MQTT, LLVM, embedded cross-compile, live AI/IoT, Python native, desktop/Tauri builds, and
remaining tier-3 golden paths.

Nightly failures do **not** block PR merge or auto-release. Triage them like production monitoring
alerts.

---

## Cross-surface change protocol

Public API changes must land as **one atomic PR** across these layers:

| Step | Paths |
|------|--------|
| 1. Proto | `crates/spanda-api/proto/` |
| 2. Rust API + CLI | `crates/spanda-api/src/`, `crates/spanda-cli/src/` |
| 3. TypeScript mirror | `src/` (including AST types such as `Program`) |
| 4. SDKs | `crates/spanda-sdk/`, `sdk/typescript/`, `sdk/python/` |
| 5. Cross-interface probe | `scripts/cross_interface_consistency.sh`, `crates/spanda-api/tests/cross_interface_live.rs` |
| 6. Docs | `CHANGELOG.md`, relevant guides |

`scripts/check_cross_surface.sh` enforces minimum coupling (proto → API → SDK) on every CI Fast run.

---

## Branch protection (GitHub settings)

Under **Settings → Branches → Branch protection for `main`**, require these **CI Fast** checks on
pull requests:

- `lint-rust`
- `test-rust`
- `test-typescript`
- `test-python-sdk`
- `test-ts-sdk`
- `cross-surface-check`
- `cross-interface`

Enable **Require status checks to pass before merging** and **Require branches to be up to date
before merging**.

Optional (recommended): enable **merge queue** so integration runs against the exact merge commit.
See [GitHub merge
queue](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue).

Do **not** require nightly or integration jobs on PRs — integration runs after merge to `main`.

Monitor **`main` health** via CI Integration workflow status. Do not push new feature work while CI
Integration is red.

---

## Main hygiene

1. Run `./scripts/ci-fast.sh` locally before opening a PR.
2. Do not merge PRs with failing CI Fast checks.
3. If `main` is red after merge, fix fmt/compile/cross-surface before stacking more commits.
4. Auto-release only fires after **CI Integration** succeeds — do not tag manually while integration
   is failing.

---

## Contributor pre-flight

```bash
npm install && npm run build:rust
./scripts/ci-fast.sh
```

Gate script index: [scripts/gates/README.md](../scripts/gates/README.md).

Related: [ci-verify.md](./ci-verify.md) (customer CI for `spanda verify`),
[troubleshooting.md](./troubleshooting.md#ci-and-release-builds),
[tier-3-golden-paths.md](./tier-3-golden-paths.md) (golden path job index).

---

## Job tier map

Use this table when updating docs or stable-hardening guides. Workflow file names are stable; job
names match GitHub Actions UI.

### CI Fast (`.github/workflows/ci-fast.yml`)

| Job | Script / check |
|-----|----------------|
| `lint-rust` | fmt, clippy, architecture, blueprints, registry index |
| `test-rust` | `cargo test --workspace` |
| `test-typescript` | `npm test`, `npm run build` |
| `test-python-sdk` | `pytest sdk/python` |
| `test-ts-sdk` | `npm test` in `sdk/typescript` |
| `cross-surface-check` | `scripts/check_cross_surface.sh` |
| `build-spanda` | `cargo build -p spanda --release` → artifact |
| `cross-interface` | `scripts/cross_interface_consistency.sh` |
| `docs-validate` | `validate_documentation.py` (docs-only PRs) |

### CI Integration (`.github/workflows/ci-integration.yml`)

| Job | Script / check |
|-----|----------------|
| `core-smokes` | `readiness_smoke`, `sdk_smoke`, `check_all_examples`, readme smoke + golden |
| `docs-build` | `cargo doc`, mdBook, `generate_spanda_reference.py` |
| `distributed-decisions` | `distributed_decisions_smoke.sh` |
| `bio-inspired-autonomy` (cognitive resilience) | `cognitive_resilience_smoke.sh` |
| `release-hardening` | security + property regressions, cross-interface |
| `robotics-golden-path` | `examples/robotics/golden_path_deploy.sh` |
| `telemetry-golden-path` | `telemetry_store_golden_path.sh` |
| `twin-cloud-golden-path` | `twin_cloud_unified_path.sh`, `hosted_twin_cloud_smoke.sh` |
| `registry-golden-path` | `registry_golden_path.sh` |
| `ci-verify-golden-path` | `ci_verify_golden_path.sh` |
| `killer-demo-golden-path` | `killer_demo_golden_path.sh` |
| `showcase-smoke` | `showcase_smoke.sh` |
| `adas-smoke` | `adas_smoke.sh` |
| `agriculture-smoke` | `solution_blueprints_smoke.sh` |
| `smart-spaces-smoke` | `smart_spaces_smoke.sh` |
| `enterprise-ops-smoke` | `enterprise_ops_smoke.sh` |
| `operational-governance-smoke` | `operational_governance_smoke.sh` (smoke only) |
| `entity-model-smoke` | `entity_model_smoke.sh` |
| `differentiation-smoke` | `differentiation_smoke.sh` |
| `lsp` | `@spanda/lsp` build + tests |
| `wasm` | wasm32 checks + `npm run web:build` |
| `vscode-extension` | VSIX package via `editor/vscode` |

Path-filtered extension checks also run via
[.github/workflows/vscode-extension-ci.yml](../.github/workflows/vscode-extension-ci.yml) when
`editor/vscode/**` or `packages/lsp/**` change.

### CI Nightly (`.github/workflows/ci-nightly.yml`)

| Job | Script / check |
|-----|----------------|
| `security-audit` | `cargo audit` |
| `mqtt-golden-path` | `mqtt_golden_path.sh` |
| `twin-cloud-stable-promotion-gate` | `twin_cloud_stable_promotion_gate.sh` |
| `llvm-golden-path` | `llvm_golden_path.sh` |
| `llvm-embedded-golden-path` | `llvm_embedded_golden_path.sh` |
| `cpp-native-golden-path` | `cpp_native_golden_path.sh` |
| `ledger-golden-path` | `ledger_golden_path.sh` |
| `self-host-lexer-golden-path` | `self_host_lexer_golden_path.sh` |
| `world-model-golden-path` | `world_model_golden_path.sh` |
| `live-ai-golden-path` | `live_ai_golden_path.sh` |
| `live-iot-golden-path` | `live_iot_golden_path.sh` |
| `python-native-golden-path` | `python_native_golden_path.sh` |
| `ros2-golden-path` | `ros2_golden_path.sh` |
| `ros2-rclrs-native` | `spanda-ros2-rclrs-native` + `transport_rclrs` |
| `recovery-orchestrator-stable-promotion-gate` | `recovery_orchestrator_stable_promotion_gate.sh` |
| `smart-spaces-promotion-gate` | `smart_spaces_promotion_gate.sh` |
| `adas-promotion-gate` | `adas_stable_promotion_gate.sh` |
| `enterprise-ops-promotion-gate` | `enterprise_ops_stable_promotion_gate.sh` |
| `entity-model-promotion-gate` | `entity_model_stable_promotion_gate.sh` |
| `differentiation-promotion-gate` | `differentiation_promotion_gate.sh` |
| `what-if-stable-promotion-gate` | `what_if_stable_promotion_gate.sh` |
| `next-differentiation-stable-gates` | risk, forecast, trust graph, scorecard gates |
| `later-differentiation-stable-gates` | `later_differentiation_stable_promotion_gate.sh` |
| `trust-framework-stable-gate` | `trust_framework_stable_promotion_gate.sh` |
| `operational-governance-promotion-gate` | `operational_governance_stable_promotion_gate.sh` |
| `control-center-desktop` | Tauri Linux build |
| `control-center-desktop-bundle` | macOS bundle + optional codesign |
