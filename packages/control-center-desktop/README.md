# Spanda Control Center (desktop)

Tauri v2 desktop shell for the Spanda Control Center. The UI reuses `ControlCenterPanel` from `@davalgi-spanda/web`; the API backend is expected to run separately via `spanda control-center serve` (or any compatible `spanda-api` deployment).

## Security note

The desktop `src-tauri/Cargo.lock` may report [RUSTSEC-2024-0429](https://rustsec.org/advisories/RUSTSEC-2024-0429.html) (`glib` &lt; 0.20) on Linux builds. This repo patches `glib`/`glib-sys`/`glib-macros` from the gtk-rs `0.18` git branch (VariantStrIter backport) via `[patch.crates-io]` in `Cargo.toml`. Upstream gtk-rs 0.20+ adoption in Tauri is tracked for v3; the Control Center web/API path does not depend on `glib`.

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 20+
- Platform Tauri dependencies: [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

## Quick start

1. Start the Control Center API (from repo root):

```bash
# Installed spanda
spanda control-center serve --bind 127.0.0.1:8080

# Release build (cargo build -p spanda --release)
./target/release/spanda control-center serve --bind 127.0.0.1:8080

# Active development
cargo run -p spanda -- control-center serve --bind 127.0.0.1:8080
```

Copy-paste examples with `--config` and `--program`: [docs/control-center.md](../../docs/control-center.md#run-with-config-and-program).

2. Install workspace dependencies (once):

```bash
npm install
```

3. Run the desktop app in dev mode:

```bash
npm run dev --workspace=@spanda/control-center-desktop
```

Optional: point the UI at a different API URL:

```bash
VITE_CONTROL_CENTER_URL=http://127.0.0.1:9090 npm run dev --workspace=@spanda/control-center-desktop
```

## Build

Generate platform icons from the bundled PNG (first time only):

```bash
npm exec tauri icon --manifest-path packages/control-center-desktop/src-tauri/Cargo.toml packages/control-center-desktop/src-tauri/icons/icon.png
```

Production bundle:

```bash
npm run build --workspace=@spanda/control-center-desktop
```

## Smoke check

```bash
./scripts/control_center_desktop_smoke.sh
```

This runs `cargo check` on the Tauri crate (no GUI required).

## Architecture

| Layer | Package / crate |
|-------|-----------------|
| React UI | `packages/web` (`ControlCenterPanel`) |
| Desktop shell | `packages/control-center-desktop` (Vite + Tauri) |
| API | `spanda-api` via `spanda control-center serve` |

The desktop app does not embed the Rust API server; operators typically run the API locally or against a fleet endpoint.

Full start and rebuild checklist (all three layers): [docs/control-center.md — Local dev: start & rebuild](../../docs/control-center.md#local-dev-start--rebuild).

## Version

| Surface | Command / location |
|---------|-------------------|
| UI sidebar | `vX.Y.Z` under **Control Center** (Tauri uses desktop shell semver) |
| CLI | `spanda control-center --version` |
| API | `GET /v1/version` → `control_center_ui_version` |

Desktop shell semver lives in three synced manifests (`package.json`, `src-tauri/Cargo.toml`, `tauri.conf.json`). **Current release: 0.6.3** (`desktop-v0.6.3`).

Bump and release: [docs/control-center-versioning.md](../../docs/control-center-versioning.md) · [docs/desktop-release-runbook.md](../../docs/desktop-release-runbook.md).

```bash
./scripts/verify_desktop_release_ready.sh
python3 scripts/bump_version.py patch --stream desktop --dry-run
```

## Auto-update

The Tauri shell includes `tauri-plugin-updater`. In development builds, `active` defaults to `false`. For production releases:

1. Generate signing keys: `npm run tauri signer generate -- -w ~/.tauri/spanda-updater.key`
2. Set `TAURI_UPDATER_PUBKEY` at build time (injected via `src-tauri/build.rs`)
3. Set `SPANDA_DESKTOP_UPDATER_ACTIVE=1` (or `TAURI_UPDATER_ACTIVE=true`) when building with `TAURI_BUILD=1`
4. Publish signed artifacts from `.github/workflows/desktop-release.yml` (tag `desktop-v*`)

See [docs/desktop-release-runbook.md](../../docs/desktop-release-runbook.md).

## Status

**Stable** — dev workflow, CI signing scaffold, env-gated updater wiring, sidebar/CLI/API version display, **automatic desktop bump** on labeled Control Center PRs, and **production release tags** (`desktop-v*`). **Current release: `desktop-v0.6.3`**. Optional Apple codesign/notarization when `APPLE_SIGNING_IDENTITY` and `APPLE_NOTARIZE_PROFILE` secrets are configured. See [docs/control-center-versioning.md](../../docs/control-center-versioning.md) · [docs/stable-hardening-enterprise-ops.md](../../docs/stable-hardening-enterprise-ops.md) · [docs/desktop-release-runbook.md](../../docs/desktop-release-runbook.md).
