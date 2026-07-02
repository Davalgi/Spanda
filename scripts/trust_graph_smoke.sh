#!/usr/bin/env bash
# Smoke trust-weighted dependency graph (NEXT differentiation).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FILE="${ROOT}/examples/showcase/trust_graph/rover.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== trust-graph crate tests =="
cargo test -p spanda-graph --quiet

echo "== trust-graph text =="
run_spanda trust-graph "$FILE" >/dev/null

echo "== trust-graph json =="
run_spanda trust-graph "$FILE" --json >/dev/null

echo "== trust-graph mermaid =="
run_spanda trust-graph "$FILE" --format mermaid >/dev/null

echo "Trust graph smoke OK"
