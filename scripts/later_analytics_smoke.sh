#!/usr/bin/env bash
# Smoke LATER differentiation Control Center REST analytics endpoints.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# shellcheck source=scripts/lib/control_center_smoke_lib.sh
source "$ROOT/scripts/lib/control_center_smoke_lib.sh"

PROGRAM="$ROOT/examples/showcase/mission_twin/patrol.sd"
GOV_PROGRAM="$ROOT/examples/showcase/policy/warehouse.sd"
TRACE_PROGRAM="$ROOT/examples/showcase/differentiation/decision_trail/main.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

fetch() {
  local path="$1"
  curl -sf --max-time 15 "http://${CC_SMOKE_BIND}${path}"
}

assert_v1() {
  python3 -c 'import json,sys; d=json.load(sys.stdin); assert d.get("version")=="v1", d'
}

start_cc() {
  local bind="$1"
  local program="$2"
  CC_SMOKE_BIND="$bind"
  run_spanda control-center serve --bind "$bind" --program "$program" &
  cc_smoke_wait_for_health
}

cleanup() { cc_smoke_stop_listener; }
cc_smoke_trap cleanup

PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
echo "== LATER analytics REST probes on 127.0.0.1:${PORT} =="
start_cc "127.0.0.1:${PORT}" "$PROGRAM"

for path in \
  "/v1/analytics/mission-twin" \
  "/v1/analytics/certification-pack" \
  "/v1/analytics/human-teaming"
do
  fetch "$path" | assert_v1
done

cc_smoke_stop_listener
sleep 1

PORT2=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
echo "== LATER governance REST probe on 127.0.0.1:${PORT2} =="
start_cc "127.0.0.1:${PORT2}" "$GOV_PROGRAM"
fetch "/v1/analytics/governance?policy=WarehousePolicy" | assert_v1
cc_smoke_stop_listener
sleep 1

PORT3=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
echo "== LATER time-travel REST probe on 127.0.0.1:${PORT3} =="
start_cc "127.0.0.1:${PORT3}" "$TRACE_PROGRAM"
fetch "/v1/analytics/time-travel?at=T%2B00%3A01&inspect=decisions" | python3 -c \
  'import json,sys; d=json.load(sys.stdin); assert d.get("version")=="v1" and "time_travel" in d, d'

echo "Later analytics smoke OK"
