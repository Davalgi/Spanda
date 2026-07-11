#!/usr/bin/env bash
# Plugin system smoke: install example CC plugin, list REST, fetch bundle.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# shellcheck source=scripts/lib/control_center_smoke_lib.sh
source "$ROOT/scripts/lib/control_center_smoke_lib.sh"

WAREHOUSE_FIXTURE="${ROOT}/crates/spanda-config/tests/fixtures/warehouse"
SMOKE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/spanda-plugin-smoke.XXXXXX")"
cp -R "${WAREHOUSE_FIXTURE}/." "${SMOKE_DIR}/"
CONFIG="${SMOKE_DIR}/spanda.toml"
PROGRAM="${ROOT}/examples/showcase/mission_twin/patrol.sd"
EXAMPLE="${ROOT}/examples/plugins/control-center-panel"
PLUGIN_NAME="spanda-plugin-control-center-panel"

export SPANDA_CONTROL_CENTER_STATE_DIR="${SMOKE_DIR}/cc-state"
mkdir -p "$SPANDA_CONTROL_CENTER_STATE_DIR"
export SPANDA_API_KEY="${SPANDA_API_KEY:-plugin-smoke-key}"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() {
    cargo run -q -p spanda --no-default-features --manifest-path "$ROOT/Cargo.toml" -- "$@"
  }
fi

PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
BIND="127.0.0.1:${PORT}"
BASE="http://${BIND}"

cleanup() {
  cc_smoke_stop_listener
  rm -rf "$SMOKE_DIR"
}
cc_smoke_trap cleanup
CC_SMOKE_BIND="$BIND"

echo "== Plugin system smoke =="

echo "--- CLI install / enable (project = smoke config dir) ---"
(
  cd "$SMOKE_DIR"
  run_spanda plugin install --path "$EXAMPLE"
  run_spanda plugin enable "$PLUGIN_NAME"
  run_spanda plugin list | grep -q "$PLUGIN_NAME"
)

echo "--- Control Center serve ---"
run_spanda control-center serve --bind "$BIND" --config "$CONFIG" --program "$PROGRAM" &
CC_SMOKE_WRAPPER_PID=$!
cc_smoke_wait_for_health

AUTH=(-H "Authorization: Bearer ${SPANDA_API_KEY}")

echo "--- GET /v1/plugins ---"
plugins_body="$(curl -sf "${AUTH[@]}" "$BASE/v1/plugins")"
echo "$plugins_body" | grep -q "$PLUGIN_NAME"
echo "$plugins_body" | grep -q 'control_center_panels'

echo "--- GET /v1/plugins/search ---"
curl -sf "${AUTH[@]}" "$BASE/v1/plugins/search?q=readiness" | grep -q 'spanda-plugin-readiness'

echo "--- GET /v1/plugins/control-center ---"
curl -sf "${AUTH[@]}" "$BASE/v1/plugins/control-center" | grep -q "$PLUGIN_NAME"

echo "--- GET bundle ---"
bundle_body="$(curl -sf "${AUTH[@]}" \
  "$BASE/v1/plugins/control-center/${PLUGIN_NAME}/bundle")"
echo "$bundle_body" | grep -q '"available":true'
echo "$bundle_body" | grep -q 'FleetOverviewPanel\|plugin-mount\|sandboxed'

echo "--- REST enable/disable round-trip ---"
curl -sf -X POST "${AUTH[@]}" "$BASE/v1/plugins/${PLUGIN_NAME}/disable" | grep -q '"ok":true'
curl -sf -X POST "${AUTH[@]}" "$BASE/v1/plugins/${PLUGIN_NAME}/enable" | grep -q '"ok":true'

echo "--- REST install (path) ---"
# Re-install is idempotent (replaces install dir).
curl -sf -X POST "${AUTH[@]}" \
  -H "Content-Type: application/json" \
  -d "{\"path\":\"${EXAMPLE}\"}" \
  "$BASE/v1/plugins/install" | grep -q "$PLUGIN_NAME"

echo "== Plugin system smoke OK =="
