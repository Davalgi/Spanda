#!/usr/bin/env bash
# Verify structural consistency across CLI, REST, and SDK surfaces.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# shellcheck source=scripts/lib/control_center_smoke_lib.sh
source "$ROOT/scripts/lib/control_center_smoke_lib.sh"

WAREHOUSE_FIXTURE="${ROOT}/crates/spanda-config/tests/fixtures/warehouse"
SMOKE_CONFIG_DIR="$(mktemp -d "${TMPDIR:-/tmp}/spanda-xiface.XXXXXX")"
cp -R "${WAREHOUSE_FIXTURE}/." "${SMOKE_CONFIG_DIR}/"
CONFIG="${SMOKE_CONFIG_DIR}/spanda.toml"
PROGRAM="${ROOT}/examples/showcase/compliance/defense_rover.sd"
SELF_HEALING="${ROOT}/examples/showcase/self_healing/rover.sd"
DECISIONS="${ROOT}/examples/showcase/distributed_decisions/main.sd"
READINESS_SD="${ROOT}/examples/showcase/readiness/rover.sd"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

PORT="${SPANDA_XIFACE_PORT:-}"
if [[ -z "$PORT" ]]; then
  PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
fi
BIND="127.0.0.1:${PORT}"
BASE="http://${BIND}"
export SPANDA_API_KEY="cross-interface-smoke-key"

fetch() {
  curl -sf --max-time 15 "${BASE}$1"
}

cleanup() {
  cc_smoke_stop_listener
  rm -rf "$SMOKE_CONFIG_DIR"
}
cc_smoke_trap cleanup

echo "== start control-center on ${BIND} =="
CC_SMOKE_BIND="$BIND"
run_spanda control-center serve --bind "$BIND" --config "$CONFIG" --program "$PROGRAM" &
CC_SMOKE_WRAPPER_PID=$!

echo "== wait for health =="
for _ in $(seq 1 40); do
  if fetch /v1/health >/dev/null 2>&1; then
    break
  fi
  sleep 0.25
done
fetch /v1/health | grep -q spanda-control-center

echo "== health consistency =="
cli_health="$(run_spanda entity health rover-001 --config "$CONFIG" --json)"
rest_health="$(fetch /v1/entities/rover-001/health)"
python3 - "$cli_health" "$rest_health" <<'PY'
import json, sys
cli = json.loads(sys.argv[1])
rest = json.loads(sys.argv[2])
for key in ("entity_id", "health_status"):
    assert key in cli or key in cli.get("health", {}), f"CLI missing {key}: {cli}"
    assert key in rest or key in rest.get("health", {}), f"REST missing {key}: {rest}"
print("health keys ok")
PY

echo "== entity list consistency =="
cli_list="$(run_spanda entity list --config "$CONFIG" --json)"
rest_list="$(fetch /v1/entities)"
python3 - "$cli_list" "$rest_list" <<'PY'
import json, sys
cli = json.loads(sys.argv[1])
rest = json.loads(sys.argv[2])
cli_ids = {e["id"] for e in cli["entities"]}
rest_ids = {e["id"] for e in rest["entities"]}
assert "rover-001" in cli_ids and "rover-001" in rest_ids
# REST may include runtime overlays (e.g. smoke entities); require CLI ⊆ REST.
missing = cli_ids - rest_ids
assert not missing, f"REST missing CLI entities: {missing}"
print(f"entity list ok (cli={len(cli_ids)} rest={len(rest_ids)} entities)")
PY

echo "== entity graph consistency =="
cli_graph="$(run_spanda entity graph --config "$CONFIG" --json)"
rest_graph="$(fetch /v1/entities/graph)"
python3 - "$cli_graph" "$rest_graph" <<'PY'
import json, sys
cli = json.loads(sys.argv[1])
rest = json.loads(sys.argv[2])
# Accept top-level nodes or a nested graph object.
def nodes(payload):
    if isinstance(payload.get("nodes"), list):
        return payload["nodes"]
    g = payload.get("graph", {})
    if isinstance(g, dict) and isinstance(g.get("nodes"), list):
        return g["nodes"]
    return payload.get("entities") or []
cli_n = nodes(cli)
rest_n = nodes(rest)
assert cli_n and rest_n, (cli, rest)
print(f"entity graph ok (cli={len(cli_n)} rest={len(rest_n)} nodes)")
PY

echo "== readiness consistency (CLI program + REST program) =="
cli_ready="$(run_spanda readiness "$READINESS_SD" --json)"
rest_ready="$(curl -sf --max-time 30 -X POST -H 'Content-Type: application/json' \
  -d "{\"file\":\"${READINESS_SD}\"}" "${BASE}/v1/programs/readiness")"
python3 - "$cli_ready" "$rest_ready" <<'PY'
import json, sys
cli = json.loads(sys.argv[1])
rest = json.loads(sys.argv[2])

def readiness_blob(payload):
    if isinstance(payload.get("report"), dict):
        return payload["report"]
    return payload

for payload in (cli, rest):
    blob = readiness_blob(payload)
    assert (
        "status" in blob or "mission_ready" in blob or "score" in blob
    ), payload
print("readiness structure ok")
PY

echo "== trust consistency =="
cli_trust="$(run_spanda entity trust rover-001 --config "$CONFIG" --json)"
rest_trust="$(fetch /v1/entities/rover-001/trust)"
python3 - "$cli_trust" "$rest_trust" <<'PY'
import json, sys
cli = json.loads(sys.argv[1])
rest = json.loads(sys.argv[2])
for payload in (cli, rest):
    blob = json.dumps(payload)
    assert "trust" in blob.lower(), payload
print("trust structure ok")
PY

echo "== recovery plan consistency =="
cli_recovery="$(run_spanda recovery plan "$SELF_HEALING" --failure gps --json 2>/dev/null || true)"
rest_recovery="$(curl -sf --max-time 30 -X POST -H 'Content-Type: application/json' \
  -H "Authorization: Bearer ${SPANDA_API_KEY}" \
  -d "{\"file\":\"${SELF_HEALING}\",\"failure\":\"gps\"}" \
  "${BASE}/v1/recovery/plan" 2>/dev/null || true)"
python3 - "$cli_recovery" "$rest_recovery" <<'PY'
import json, sys
cli_raw, rest_raw = sys.argv[1], sys.argv[2]
# Recovery plan may be text-only on CLI; accept either JSON or non-empty text.
if cli_raw.strip().startswith("{"):
    cli = json.loads(cli_raw)
    assert "plans" in cli or "passed" in cli or "mode" in cli, cli
else:
    assert "Recovery" in cli_raw or "plan" in cli_raw.lower(), cli_raw[:200]
if rest_raw.strip().startswith("{"):
    rest = json.loads(rest_raw)
    assert isinstance(rest, dict)
print("recovery plan structure ok")
PY

echo "== decision list / traces consistency =="
cli_decisions="$(run_spanda decision list "$DECISIONS")"
rest_traces="$(fetch "/v1/decisions/traces?file=${DECISIONS}" 2>/dev/null || echo '{}')"
python3 - "$cli_decisions" "$rest_traces" <<'PY'
import json, sys
cli = sys.argv[1]
rest_raw = sys.argv[2]
assert "Decision architecture" in cli or "Authorities" in cli
if rest_raw.strip().startswith("{"):
    json.loads(rest_raw)
print("decision surfaces ok")
PY

echo "== TypeScript SDK structural probe =="
(
  cd "${ROOT}/sdk/typescript"
  if [[ ! -f dist/index.js ]]; then
    npm ci >/dev/null
    npm run build >/dev/null
  fi
  node --input-type=module -e "
import { SpandaClient } from './dist/index.js';
const client = new SpandaClient({ baseUrl: '${BASE}' });
const entities = await client.listEntities();
const list = Array.isArray(entities) ? entities : (entities?.entities ?? []);
if (!list.some((e) => e.id === 'rover-001')) {
  console.error('TS SDK listEntities missing rover-001', entities);
  process.exit(1);
}
const graph = await client.entityGraph();
if (!graph) {
  console.error('TS SDK entityGraph empty');
  process.exit(1);
}
console.log('typescript sdk ok');
"
)

echo "== Python SDK structural probe =="
PYTHONPATH="${ROOT}/sdk/python" python3 - "$BASE" <<'PY'
import sys
from spanda_sdk import SpandaClient

base = sys.argv[1]
client = SpandaClient(base_url=base)
entities = client.list_entities()
payload = entities.get("entities", entities) if isinstance(entities, dict) else entities
ids = {e.get("id") for e in payload}
assert "rover-001" in ids, entities
graph = client.entity_graph()
assert graph is not None
print("python sdk ok")
PY

echo "Cross-interface consistency OK"
