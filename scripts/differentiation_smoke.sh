#!/usr/bin/env bash
# Smoke differentiation NOW commands (mission contracts, explain, coverage, audit).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FILE="${ROOT}/examples/showcase/differentiation/warehouse.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== spanda-contract tests =="
cargo test -p spanda-contract --quiet

echo "== contract verify =="
run_spanda contract verify "$FILE" >/dev/null

echo "== safety-coverage =="
run_spanda safety-coverage "$FILE" >/dev/null

echo "== recovery-coverage =="
run_spanda recovery-coverage "$FILE" >/dev/null

echo "== explain =="
run_spanda explain "$FILE" >/dev/null
run_spanda explain readiness --file "$FILE" >/dev/null
run_spanda explain verify --file "$FILE" >/dev/null
run_spanda explain safety --file "$FILE" >/dev/null

echo "== record trace + audit decisions =="
TRACE="${ROOT}/examples/showcase/differentiation/warehouse.trace"
rm -f "$TRACE"
run_spanda sim "$FILE" --record >/dev/null || true
if [[ -f "$TRACE" ]]; then
  run_spanda audit decisions "$TRACE" >/dev/null || true
  run_spanda explain "$TRACE" >/dev/null || true
fi

echo "== demo differentiation =="
export SPANDA_ROOT="${ROOT}"
run_spanda demo differentiation

echo "Differentiation smoke OK"
