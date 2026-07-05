# CI architecture

Spanda CI is split into **three tiers** so pull requests stay fast while `main` and nightly runs retain full platform coverage.

| Workflow | File | When | Blocks merge? |
|----------|------|------|---------------|
| **CI Fast** | [.github/workflows/ci-fast.yml](../.github/workflows/ci-fast.yml) | Every PR and push to `main` | **Yes** (required checks) |
| **CI Integration** | [.github/workflows/ci-integration.yml](../.github/workflows/ci-integration.yml) | After CI Fast succeeds on `main` push | **Yes for `main` health** (not individual PRs) |
| **CI Nightly** | [.github/workflows/ci-nightly.yml](../.github/workflows/ci-nightly.yml) | Daily cron + manual dispatch | **No** (signal only) |

**Auto release** (`.github/workflows/auto-release.yml`) waits for **CI Integration** success on `main`, not nightly.

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

**Path filters:** docs-only PRs skip Rust/TS/Python jobs and run `docs-validate` instead. Any change under `crates/`, `src/`, `sdk/`, workflows, etc. runs the full fast gate.

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

Builds `spanda` **once**, uploads an artifact, and reuses it across smoke and golden-path jobs (no per-job release compiles).

Includes: readme smoke + golden output, core smokes, distributed decisions, cognitive resilience, release hardening security/property tests, key golden paths (robotics, telemetry, twin cloud, registry, ci-verify, killer demo), solution blueprint smokes, entity model smoke, LSP, WASM, VS Code extension packaging, and docs/mdBook build.

---

## CI Nightly (~60+ min)

Scheduled at **06:00 UTC** and available via **Actions → CI Nightly → Run workflow**.

Includes: `cargo audit`, promotion gates (Stable hardening scripts with soak/audit skipped in CI), ROS 2, MQTT, LLVM, embedded cross-compile, live AI/IoT, Python native, desktop/Tauri builds, and remaining tier-3 golden paths.

Nightly failures do **not** block PR merge or auto-release. Triage them like production monitoring alerts.

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

Under **Settings → Branches → Branch protection for `main`**, require these **CI Fast** checks on pull requests:

- `lint-rust`
- `test-rust`
- `test-typescript`
- `test-python-sdk`
- `test-ts-sdk`
- `cross-surface-check`
- `cross-interface`

Enable **Require status checks to pass before merging** and **Require branches to be up to date before merging**.

Optional (recommended): enable **merge queue** so integration runs against the exact merge commit. See [GitHub merge queue](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue).

Do **not** require nightly or integration jobs on PRs — integration runs after merge to `main`.

Monitor **`main` health** via CI Integration workflow status. Do not push new feature work while CI Integration is red.

---

## Main hygiene

1. Run `./scripts/ci-fast.sh` locally before opening a PR.
2. Do not merge PRs with failing CI Fast checks.
3. If `main` is red after merge, fix fmt/compile/cross-surface before stacking more commits.
4. Auto-release only fires after **CI Integration** succeeds — do not tag manually while integration is failing.

---

## Contributor pre-flight

```bash
npm install && npm run build:rust
./scripts/ci-fast.sh
```

Gate script index: [scripts/gates/README.md](../scripts/gates/README.md).

Related: [ci-verify.md](./ci-verify.md) (customer CI for `spanda verify`), [troubleshooting.md](./troubleshooting.md#ci-and-release-builds).
