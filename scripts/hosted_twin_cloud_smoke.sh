#!/usr/bin/env bash
# Hosted Twin Cloud smoke — tenant scoping + twin push/list against Control Center.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# shellcheck source=scripts/lib/control_center_smoke_lib.sh
source "$ROOT/scripts/lib/control_center_smoke_lib.sh"

PROGRAM="$ROOT/examples/showcase/mission_twin/patrol.sd"
STATE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/spanda-hosted-twin.XXXXXX")"
export SPANDA_CONTROL_CENTER_STATE_DIR="$STATE_DIR"
export SPANDA_TENANT_ID="${SPANDA_TENANT_ID:-hosted-tenant-a}"
export SPANDA_API_KEY="${SPANDA_API_KEY:-hosted-twin-smoke-key}"
export SPANDA_TWIN_CLOUD_API_KEY="$SPANDA_API_KEY"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
BIND="127.0.0.1:${PORT}"
BASE="http://${BIND}"
cleanup() {
  cc_smoke_stop_listener
  rm -rf "$STATE_DIR"
}
cc_smoke_trap cleanup
CC_SMOKE_BIND="$BIND"

echo "== Hosted Twin Cloud tenant smoke =="
run_spanda control-center serve --bind "$BIND" --program "$PROGRAM" &
CC_SMOKE_WRAPPER_PID=$!
cc_smoke_wait_for_health

tenant_body="$(curl -sf "$BASE/v1/tenant")"
echo "$tenant_body" | grep -q '"hosted-tenant-a"'

export SPANDA_TWIN_CLOUD_URL="$BASE"
run_spanda twin cloud push "$PROGRAM" --json | grep -q '"twin_id"'

mismatch_status="$(curl -s -o /dev/null -w '%{http_code}' -X POST "$BASE/v1/twins/sync" \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer wrong-key' \
  -d '{}')"
[[ "$mismatch_status" == "401" || "$mismatch_status" == "403" ]]

curl -sf -H "Authorization: Bearer ${SPANDA_API_KEY}" "$BASE/v1/twins" | grep -q '"patrol"'

echo "Hosted Twin Cloud smoke OK"
