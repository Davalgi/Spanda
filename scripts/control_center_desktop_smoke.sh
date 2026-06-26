#!/usr/bin/env bash
# Smoke check for Control Center Tauri desktop (compile-only, no GUI).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
echo "[control-center-desktop] cargo check (src-tauri)"
cargo check --manifest-path packages/control-center-desktop/src-tauri/Cargo.toml
echo "[control-center-desktop] OK"
