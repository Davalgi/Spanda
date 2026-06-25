#!/usr/bin/env bash
# Smoke readiness history recording and trend analysis.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FILE="${ROOT}/examples/showcase/readiness/rover.sd"
HISTORY="${ROOT}/.spanda/readiness-history-smoke.json"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

rm -f "$HISTORY"

echo "== readiness record x2 =="
run_spanda readiness "$FILE" --record --history "$HISTORY" || true
run_spanda readiness "$FILE" --record --history "$HISTORY" || true

echo "== readiness trends json =="
run_spanda readiness trends "$FILE" --history "$HISTORY" --forecast 7d --json >/dev/null

echo "Readiness trends smoke OK"
