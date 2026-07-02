#!/usr/bin/env bash
# Smoke readiness forecasting (NEXT differentiation).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FILE="${ROOT}/examples/showcase/forecast/degradation.sd"
HISTORY="${ROOT}/.spanda/readiness-forecast-smoke.json"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

rm -f "$HISTORY"

echo "== readiness forecast heuristic =="
run_spanda readiness forecast "$FILE" --json >/dev/null

echo "== readiness record + forecast with history =="
run_spanda readiness "$FILE" --record --history "$HISTORY" >/dev/null || true
run_spanda readiness "$FILE" --record --history "$HISTORY" >/dev/null || true
run_spanda readiness forecast "$FILE" --history "$HISTORY" --all --json >/dev/null || true

echo "Readiness forecast smoke OK"
