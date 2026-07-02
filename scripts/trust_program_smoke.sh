#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  BIN="$SPANDA_BIN"
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  BIN="${CARGO_TARGET_DIR:-$ROOT/target}/debug/spanda"
  run_spanda() { "$BIN" "$@"; }
fi
ROVER="$ROOT/examples/showcase/tamper_policy/rover.sd"

cd "$ROOT"
if [[ ! -x "$BIN" ]]; then
  cargo build -p spanda -q
fi

echo "== composite program trust =="
OUTPUT="$(run_spanda trust "$ROVER" 2>&1 || true)"
echo "$OUTPUT"
echo "$OUTPUT" | grep -q "Composite trust:"
echo "$OUTPUT" | grep -q "package_trust"
echo "$OUTPUT" | grep -q "device_integrity"

echo "== composite program trust json =="
JSON="$(run_spanda trust "$ROVER" --json 2>&1 || true)"
echo "$JSON" | grep -q '"score"'
echo "$JSON" | grep -q '"integrity_status"'

echo "trust program smoke ok"
