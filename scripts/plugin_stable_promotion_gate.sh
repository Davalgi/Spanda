#!/usr/bin/env bash
# Plugin system Stable promotion gate (implementation checks).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== Plugin system stable promotion gate =="

echo "--- spanda-plugin crate tests ---"
cargo test -p spanda-plugin --quiet

echo "--- Plugin system smoke ---"
./scripts/plugin_system_smoke.sh

echo "--- Docs presence ---"
test -f docs/plugin-stable-promotion.md
test -f docs/plugin-security.md
test -f docs/plugins.md

echo "== Plugin system stable promotion gate OK =="
