#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${CARGO_TARGET_DIR:-$ROOT/target}/debug/spanda"
ROVER="$ROOT/examples/showcase/assurance/rover.sd"

cd "$ROOT"
cargo build -p spanda -q

echo "== security assurance rollup =="
OUTPUT="$("$BIN" security assurance "$ROVER" 2>&1 || true)"
echo "$OUTPUT"
echo "$OUTPUT" | grep -q "Security assurance:"
echo "$OUTPUT" | grep -q "attack_surface"
echo "$OUTPUT" | grep -q "tamper"

echo "== security assurance json =="
JSON="$("$BIN" security assurance "$ROVER" --json 2>&1 || true)"
echo "$JSON" | grep -q '"trust_score"'

echo "security assurance smoke ok"
