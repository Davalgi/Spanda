#!/usr/bin/env bash
# Recovery Orchestrator Stable tier promotion gate.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# shellcheck source=scripts/lib/control_center_smoke_lib.sh
source "$ROOT/scripts/lib/control_center_smoke_lib.sh"

PROGRAM="$ROOT/examples/showcase/self_healing/rover.sd"

echo "== Recovery Orchestrator stable promotion gate =="

if [[ "${SPANDA_RECOVERY_SKIP_SOAK:-1}" != "1" ]]; then
  SOAK_FILE="${SPANDA_RECOVERY_FIELD_SOAK_START_FILE:-$ROOT/.spanda/recovery-field-soak-start.txt}"
  MIN_DAYS="${SPANDA_RECOVERY_FIELD_SOAK_MIN_DAYS:-14}"
  if [[ ! -f "$SOAK_FILE" ]]; then
    echo "missing soak file: $SOAK_FILE (run scripts/recovery_orchestrator_field_soak_init.sh)" >&2
    exit 1
  fi
  echo "Recovery field soak file present: $SOAK_FILE (min ${MIN_DAYS} days — timer not enforced in smoke)"
fi

echo "--- Recovery orchestrator smoke ---"
if [[ "${SPANDA_RECOVERY_SKIP_SMOKE:-0}" != "1" ]]; then
  "$ROOT/scripts/recovery_orchestrator_smoke.sh"
else
  echo "Skipping smoke (SPANDA_RECOVERY_SKIP_SMOKE=1)"
fi

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

PORT="${SPANDA_RECOVERY_TEST_PORT:-}"
if [[ -z "$PORT" ]]; then
  PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
fi
BIND="127.0.0.1:${PORT}"

echo "--- Control Center /v1/recovery/* probe on ${BIND} ---"

cleanup() {
  cc_smoke_stop_listener
}
cc_smoke_trap cleanup

CC_SMOKE_BIND="$BIND"
run_spanda control-center serve --bind "$BIND" --program "$PROGRAM" &
CC_SMOKE_WRAPPER_PID=$!
cc_smoke_wait_for_health

fetch() {
  local path="$1"
  curl -sf --max-time 15 "http://${BIND}${path}"
}

body="$(fetch "/v1/recovery/playbooks")"
echo "$body" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert d.get("version")=="v1", d; assert "playbooks" in d'

body="$(fetch "/v1/recovery/history")"
echo "$body" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert "history" in d'

body="$(curl -sf --max-time 15 -X POST "http://${BIND}/v1/recovery/plan" \
  -H 'Content-Type: application/json' \
  -d '{"failure":"gps"}')"
echo "$body" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert "report" in d'

body="$(fetch "/v1/recovery/predictive")"
echo "$body" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert "indicators" in d'

body="$(fetch "/v1/recovery/recoverable-entities")"
echo "$body" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert "entities" in d'

body="$(curl -sf --max-time 15 -X POST "http://${BIND}/v1/recovery/recommend" \
  -H 'Content-Type: application/json' \
  -d '{"failure":"gps_loss"}')"
echo "$body" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert "recommendation" in d'

echo ""
echo "Recovery Orchestrator stable promotion gate passed."
