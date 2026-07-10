# Versioning policy

Spanda follows [Semantic Versioning 2.0.0](https://semver.org/). Each **release stream** has its own
version line — bump **only the stream whose area changed**.

**Canonical roadmap milestones:** [ROADMAP.md](../ROADMAP.md#release-milestones)

## Independent release streams

| Stream | When to bump | Manifests | Release tag(s) |
|--------|--------------|-----------|----------------|
| **Workspace / CLI** | Language, compiler, runtime, CLI, core platform milestones | `Cargo.toml` `[workspace.package].version`, root `package.json`, `editor/vscode/`, `packages/lsp`, `packages/native`, `packages/web` | `vX.Y.Z` |
| **Official SDKs** | New or changed REST/gRPC client methods; SDK API fixes (Rust + Python + TS bump **together**) | `crates/spanda-sdk/Cargo.toml`, `sdk/python/pyproject.toml`, `sdk/typescript/package.json`, `packages/sdk-python/pyproject.toml` | `crates-sdk-vX.Y.Z`, `sdk-python-vX.Y.Z`, `npm-sdk-vX.Y.Z` |
| **Control Center desktop** | Tauri shell, embedded UI packaging, desktop-only fixes | `packages/control-center-desktop/package.json`, `src-tauri/Cargo.toml`, `tauri.conf.json` | `desktop-vX.Y.Z` |
| **gRPC proto** | Additive or breaking RPC changes | `crates/spanda-api/proto/spanda/v1/control_center.proto` | *(no tag — pin via `GET /v1/version`)* |

**Rule:** Do **not** bump SDK or desktop when only the workspace changes, and vice versa. Streams
may diverge (for example workspace `0.7.0`, SDK `0.5.9`, desktop `0.6.3`).

## Semver component guide (per stream)

| Component | Increment when |
|-----------|----------------|
| **Patch** | Bug fixes, regressions, small non-breaking polish **within that stream's current release line** |
| **Minor** | Substantial additive features for that stream, or a **roadmap milestone** for workspace |
| **Major** | Breaking public contracts for that stream; workspace **v1.0** positioning |

### Workspace-specific milestones

| Milestone / phase | Bump |
|-------------------|------|
| Bug fix in CLI / language / runtime | **patch** (workspace) |
| Architecture hardening phase (no user-visible theme) | **patch** or defer |
| **Roadmap release milestone** (v0.5, v1.0, …) | **minor** (workspace) |
| Breaking language syntax or default CLI behavior | **major** (workspace) |

### SDK-specific

| Change | Bump |
|--------|------|
| New REST route wrappers / gRPC client methods (all three SDKs) | **minor** or **patch** |
| SDK-only bug fix | **patch** |
| Breaking public SDK API | **major** |

### Desktop-specific

| Change | Bump |
|--------|------|
| Tauri shell, updater, packaging, desktop UI wiring | **patch** or **minor** |
| Breaking desktop install/upgrade path | **major** |

**Automatic desktop bump:** When a merged PR has a `release:*` label **and** changes Control Center
paths (see `scripts/control_center_paths_changed.sh`), **Auto release** bumps the desktop stream
with the same semver component and pushes `desktop-v*`.

**Version display:** Control Center UI shows `vX.Y.Z` in the sidebar; `spanda control-center
--version` and `spanda control-center status` report the UI semver; `GET /v1/version` and `GET
/v1/instance` include `control_center_ui_version`. Full reference:
[control-center-versioning.md](./control-center-versioning.md).

## Bump commands

```bash
# Workspace (CLI / platform) — updates CHANGELOG [Unreleased] → dated section
python3 scripts/bump_version.py minor --dry-run
python3 scripts/bump_version.py patch

# Official SDKs only (Rust + Python + TypeScript together)
python3 scripts/bump_version.py patch --stream sdk --dry-run
python3 scripts/bump_version.py minor --stream sdk

# Control Center desktop only
python3 scripts/bump_version.py patch --stream desktop --dry-run
python3 scripts/bump_version.py minor --stream desktop
```

## Tag and push (only the stream you bumped)

```bash
# Workspace release
git tag v0.5.1 && git push origin v0.5.1

# SDK release (push all three tags)
git tag crates-sdk-v0.5.1 sdk-python-v0.5.1 npm-sdk-v0.5.1
git push origin crates-sdk-v0.5.1 sdk-python-v0.5.1 npm-sdk-v0.5.1

# Desktop release
git tag desktop-v0.5.1 && git push origin desktop-v0.5.1
```

| Tag | Triggers |
|-----|----------|
| `vX.Y.Z` | cargo-dist **Release** (CLI installers) |
| `crates-sdk-vX.Y.Z` | [publish-sdk-rust.yml](../.github/workflows/publish-sdk-rust.yml) |
| `sdk-python-vX.Y.Z` | [publish-sdk-python.yml](../.github/workflows/publish-sdk-python.yml) |
| `npm-sdk-vX.Y.Z` | [publish-sdk-typescript.yml](../.github/workflows/publish-sdk-typescript.yml) |
| `desktop-vX.Y.Z` | [desktop-release.yml](../.github/workflows/desktop-release.yml) |

Pre-release checks:

```bash
./scripts/verify_sdk_publish_ready.sh      # before SDK tags
./scripts/verify_desktop_release_ready.sh  # before desktop tag
```

## Current versions (2026-07-10)

| Stream | Version | Last tag |
|--------|---------|----------|
| Workspace / CLI | **0.7.0** | `v0.7.0` |
| Official SDKs (published) | **0.5.9** | `crates-sdk-v0.5.9`, `sdk-python-v0.5.9`, `npm-sdk-v0.5.9` |
| Official SDKs (workspace line) | **0.5.9** | *(aligned with published)* |
| Control Center desktop | **0.6.3** | `desktop-v0.6.3` |
| Official registry packages | **91** | `packages/registry/` → `registry/index.json` |

**Sources of truth (do not invent counts/versions in prose):**

| Fact | Canonical source |
|------|------------------|
| Workspace version | `[workspace.package].version` in root `Cargo.toml` |
| SDK versions | `crates/spanda-sdk/Cargo.toml`, `sdk/python/pyproject.toml`, `sdk/typescript/package.json` (keep aligned) |
| Official package count | `ls -d packages/registry/*/ \| wc -l` (must match `registry/index.json` length after `build-registry.sh`) |

When docs mention SDK version or package count, update them from these sources in the same change.

## Checklist

- [ ] Identify which stream(s) actually changed
- [ ] Bump **only** those streams (`--stream workspace|sdk|desktop`)
- [ ] Add `CHANGELOG.md` notes for workspace releases; optional SDK/desktop notes under `[Unreleased]`
- [ ] Push **only** the tag(s) for the stream you released
- [ ] Do not bump a stream for unrelated work in another area

## Related docs

- [CONTRIBUTING.md](../CONTRIBUTING.md#releases) — workspace and desktop auto release via PR labels
- [control-center-versioning.md](./control-center-versioning.md) — Control Center UI/CLI/API version
  display and auto bump
- [sdk-publishing.md](./sdk-publishing.md) — SDK registry secrets
- [desktop-release-runbook.md](./desktop-release-runbook.md) — Tauri release
