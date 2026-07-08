#!/usr/bin/env bash
# Autonomous Entity Mesh smoke — CLI + REST + SDK endpoints.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
# shellcheck source=scripts/lib/control_center_smoke_lib.sh
source "$ROOT/scripts/lib/control_center_smoke_lib.sh"
WAREHOUSE_FIXTURE="${ROOT}/crates/spanda-config/tests/fixtures/warehouse"
SMOKE_CONFIG_DIR="$(mktemp -d "${TMPDIR:-/tmp}/spanda-mesh-smoke.XXXXXX")"
cp -R "${WAREHOUSE_FIXTURE}/." "${SMOKE_CONFIG_DIR}/"
CONFIG="${SMOKE_CONFIG_DIR}/spanda.toml"
PROGRAM="${ROOT}/examples/showcase/compliance/defense_rover.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

PORT="${SPANDA_MESH_SMOKE_PORT:-}"
if [[ -z "$PORT" ]]; then
  PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
fi
BIND="127.0.0.1:${PORT}"

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
  echo "failed to fetch ${path}" >&2
  return 1
}

post_json() {
  local path="$1"
  local body="$2"
  curl -sf --max-time 30 -X POST -H "Content-Type: application/json" -d "$body" \
    "http://${BIND}${path}"
}

cleanup() {
  cc_smoke_stop_listener
  rm -rf "$SMOKE_CONFIG_DIR"
}
cc_smoke_trap cleanup

echo "== CLI mesh discover =="
run_spanda mesh discover --config "$CONFIG" | grep -q 'Discovered'

echo "== CLI mesh health =="
run_spanda mesh health --config "$CONFIG" | grep -q 'Mesh Health'

echo "== start control-center on ${BIND} =="
CC_SMOKE_BIND="$BIND"
run_spanda control-center serve --bind "$BIND" --config "$CONFIG" --program "$PROGRAM" &
CC_SMOKE_WRAPPER_PID=$!

echo "== wait for /v1/health =="
fetch /v1/health | grep -q spanda-control-center

echo "== GET /v1/mesh/topology =="
fetch /v1/mesh/topology | grep -q '"topology"'

echo "== GET /v1/mesh/nodes =="
fetch /v1/mesh/nodes | grep -q '"nodes"'

echo "== GET /v1/mesh/health =="
fetch /v1/mesh/health | grep -q '"health"'

echo "== GET /v1/mesh/graph =="
fetch /v1/mesh/graph | grep -q '"graph"'

echo "== POST /v1/mesh/discover =="
post_json /v1/mesh/discover '{}' | grep -q '"discovery"'

echo "== POST /v1/mesh/find-capability =="
post_json /v1/mesh/find-capability '{"capability":"calibrate"}' | grep -q '"matches"'

echo "== TypeScript SDK mesh =="
if command -v npm >/dev/null 2>&1 && [[ -f "${ROOT}/sdk/typescript/package.json" ]]; then
  (
    cd "${ROOT}/sdk/typescript"
    npm run build --silent 2>/dev/null || npm run build
    SPANDA_CONTROL_CENTER_URL="http://${BIND}" \
    node --input-type=module -e "
import { SpandaClient } from './dist/index.js';
const c = new SpandaClient();
const health = await c.meshHealth();
if (!health.health) throw new Error('mesh health missing');
const nodes = await c.meshNodes();
if (!nodes.nodes) throw new Error('mesh nodes missing');
console.log('ts-sdk mesh smoke ok');
"
  )
fi

echo "== Python SDK mesh =="
if command -v python3 >/dev/null 2>&1 && [[ -f "${ROOT}/sdk/python/pyproject.toml" ]]; then
  PYTHONPATH="${ROOT}/sdk/python${PYTHONPATH:+:${PYTHONPATH}}" \
  SPANDA_CONTROL_CENTER_URL="http://${BIND}" \
  python3 - <<'PY'
from spanda_sdk import SpandaClient

client = SpandaClient()
health = client.mesh_health()
if "health" not in health:
    raise SystemExit("mesh health missing")
nodes = client.mesh_nodes()
if "nodes" not in nodes:
    raise SystemExit("mesh nodes missing")
print("py-sdk mesh smoke ok")
PY
fi

echo "== Rust SDK mesh =="
SPANDA_CONTROL_CENTER_URL="http://${BIND}" \
  cargo run -q -p spanda-sdk --example entity_mesh

echo "Entity mesh smoke OK"
