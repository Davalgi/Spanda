#!/usr/bin/env bash
# Official Solution Blueprint scaffolds — agriculture, environmental, maritime.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

check_verify() {
  local label="$1"
  local file="$2"
  local target="$3"
  echo "== ${label} check =="
  run_spanda check "$file"
  echo "== ${label} verify =="
  run_spanda verify "$file" --target "$target"
}

check_only() {
  local label="$1"
  local file="$2"
  echo "== ${label} check =="
  run_spanda check "$file"
}

echo "== Core blueprint scaffolds =="
check_verify "agriculture-field-patrol" \
  "$ROOT/examples/solutions/agriculture/field_patrol.sd" \
  FieldRobotV1

check_verify "environmental-sensor-mesh" \
  "$ROOT/examples/solutions/environmental-monitoring/sensor_mesh.sd" \
  SensorNodeV1

check_verify "maritime-harbor-patrol" \
  "$ROOT/examples/solutions/maritime/harbor_patrol.sd" \
  CoastalVesselV1

echo "== Extended blueprint missions =="
check_verify "agriculture-spray" \
  "$ROOT/examples/solutions/agriculture/spray_mission.sd" \
  FieldRobotV1

check_verify "agriculture-harvest-convoy" \
  "$ROOT/examples/solutions/agriculture/harvest_convoy.sd" \
  FieldRobotV1

check_verify "environmental-gateway" \
  "$ROOT/examples/solutions/environmental-monitoring/gateway_bridge.sd" \
  MeshGatewayV1

check_verify "maritime-convoy-escort" \
  "$ROOT/examples/solutions/maritime/convoy_escort.sd" \
  CoastalVesselV1

check_verify "maritime-docking" \
  "$ROOT/examples/solutions/maritime/docking_assist.sd" \
  CoastalVesselV1

echo "Solution blueprint smoke OK"
