#!/usr/bin/env bash
# What-If Analysis Stable tier promotion gate (first NEXT differentiation pillar).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# shellcheck source=scripts/lib/control_center_smoke_lib.sh
source "$ROOT/scripts/lib/control_center_smoke_lib.sh"

SOAK_FILE="${SPANDA_WHATIF_FIELD_SOAK_START_FILE:-$ROOT/.spanda/what-if-field-soak-start.txt}"
MIN_DAYS="${SPANDA_WHATIF_FIELD_SOAK_MIN_DAYS:-30}"
PROGRAM="$ROOT/examples/showcase/what_if/gps_failure.sd"

echo "== What-if stable promotion gate =="

if [[ "${SPANDA_WHATIF_SKIP_SOAK:-1}" != "1" ]]; then
  echo "--- Field soak (min ${MIN_DAYS} days) ---"
  if [[ ! -f "$SOAK_FILE" ]]; then
    echo "missing soak start file: $SOAK_FILE" >&2
    echo "Create with: ./scripts/what_if_field_soak_init.sh" >&2
    exit 1
  fi
  START_DATE="$(tr -d '[:space:]' < "$SOAK_FILE")"
  if date -u -j -f "%Y-%m-%d" "$START_DATE" "+%s" >/dev/null 2>&1; then
    START_EPOCH="$(date -u -j -f "%Y-%m-%d" "$START_DATE" "+%s")"
  else
    START_EPOCH="$(date -u -d "$START_DATE" "+%s")"
  fi
  NOW_EPOCH="$(date -u "+%s")"
  ELAPSED_DAYS=$(( (NOW_EPOCH - START_EPOCH) / 86400 ))
  echo "What-if soak started: $START_DATE (${ELAPSED_DAYS} days elapsed)"
  if (( ELAPSED_DAYS < MIN_DAYS )); then
    echo "What-if field soak incomplete: need $(( MIN_DAYS - ELAPSED_DAYS )) more day(s)" >&2
    exit 1
  fi
else
  echo "Skipping field soak (SPANDA_WHATIF_SKIP_SOAK=1)"
fi

AUDIT_FILE="${SPANDA_WHATIF_SECURITY_AUDIT_PREP_FILE:-$ROOT/.spanda/what-if-security-audit-prep.json}"
if [[ "${SPANDA_WHATIF_SKIP_AUDIT:-1}" != "1" ]]; then
  echo "--- What-if security audit prep artifact ---"
  if [[ ! -f "$AUDIT_FILE" ]]; then
    echo "missing audit prep file: $AUDIT_FILE" >&2
    echo "Run: ./scripts/what_if_security_audit_prep.sh" >&2
    exit 1
  fi
  python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$AUDIT_FILE"
  echo "What-if audit prep artifact present"
else
  echo "Skipping audit prep check (SPANDA_WHATIF_SKIP_AUDIT=1)"
fi

echo "--- What-if smoke ---"
if [[ "${SPANDA_WHATIF_SKIP_SMOKE:-0}" != "1" ]]; then
  "$ROOT/scripts/what_if_smoke.sh"
else
  echo "Skipping smoke (SPANDA_WHATIF_SKIP_SMOKE=1)"
fi

echo "--- spanda-whatif + analytics API tests ---"
cargo test -p spanda-whatif --quiet
cargo test -p spanda-api --test differentiation_analytics_api_tests --quiet

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

PORT="${SPANDA_WHATIF_TEST_PORT:-}"
if [[ -z "$PORT" ]]; then
  PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
fi
BIND="127.0.0.1:${PORT}"

echo "--- Control Center analytics what-if probe on ${BIND} ---"

cleanup() {
  cc_smoke_stop_listener
}
cc_smoke_trap cleanup

CC_SMOKE_BIND="$BIND"
run_spanda control-center serve --bind "$BIND" --program "$PROGRAM" &
CC_SMOKE_WRAPPER_PID=$!
sleep 2

fetch() {
  local path="$1"
  local attempt=0
  while [[ $attempt -lt 30 ]]; do
    if curl -sf --max-time 15 "http://${BIND}${path}"; then
      return 0
    fi
    attempt=$((attempt + 1))
    sleep 0.2
  done
  echo "failed to fetch http://${BIND}${path}" >&2
  return 1
}

body="$(fetch "/v1/analytics/what-if?all=1")"
echo "$body" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert d.get("version")=="v1", d'

echo ""
echo "What-if stable promotion gate passed."
