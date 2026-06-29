#!/usr/bin/env bash
# Smart Spaces & Ambient Intelligence blueprint smoke — examples/solutions/smart-spaces/
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BLUEPRINT="$ROOT/examples/solutions/smart-spaces"

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

echo "== Smart Spaces blueprint =="
check_verify "smart-home-night-mode" \
  "$BLUEPRINT/smart-home/night_mode.sd" \
  SmartSpaceGatewayV1

check_verify "smart-office-occupancy" \
  "$BLUEPRINT/smart-office/occupancy_climate.sd" \
  SmartSpaceGatewayV1

check_verify "smart-building-floor" \
  "$BLUEPRINT/smart-building/floor_readiness.sd" \
  BuildingEdgeV1

check_verify "hospital-at-home" \
  "$BLUEPRINT/hospital-at-home/patient_monitoring.sd" \
  SmartSpaceGatewayV1

check_verify "energy-demand-response" \
  "$BLUEPRINT/energy-management/demand_response.sd" \
  SmartSpaceGatewayV1

check_verify "emergency-fire-response" \
  "$BLUEPRINT/emergency-response/fire_response.sd" \
  BuildingEdgeV1

echo "Smart Spaces blueprint smoke OK"
