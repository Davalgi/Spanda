#!/usr/bin/env bash
# Smoke LATER differentiation pillars (time travel, mission twin, certify pack, team, governance).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
TWIN="${ROOT}/examples/showcase/mission_twin/patrol.sd"
TEAM="${ROOT}/examples/showcase/human_robot/approval_escalation.sd"
CERT="${ROOT}/examples/showcase/certify/deployment_bundle/rover.sd"
GOV="${ROOT}/examples/showcase/policy/warehouse.sd"
TRAIL="${ROOT}/examples/showcase/differentiation/decision_trail/main.sd"
TRACE="${ROOT}/examples/showcase/differentiation/decision_trail/main.trace"
BUNDLE="${ROOT}/.spanda/cert-pack-smoke"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== runtime time travel tests =="
cargo test -p spanda-runtime --test time_travel_tests --quiet

echo "== mission twin =="
run_spanda twin mission "$TWIN" --json >/dev/null

echo "== team verify =="
run_spanda team verify "$TEAM" --json >/dev/null

echo "== certify pack =="
rm -rf "$BUNDLE"
set +e
run_spanda certify pack "$CERT" --bundle "$BUNDLE" --json > /tmp/spanda-cert-pack-smoke.json 2>&1
set -e
grep -q '"evidence"' /tmp/spanda-cert-pack-smoke.json
test -f "$BUNDLE/certification-pack.json"

echo "== governance =="
run_spanda governance "$GOV" --policy WarehousePolicy --json >/dev/null

echo "== replay time travel =="
run_spanda check "$TRAIL" >/dev/null
rm -f "$TRACE"
export SPANDA_DECISION_TRACE=1
run_spanda sim "$TRAIL" --record --inject-health-faults >/dev/null
if [[ -f "$TRACE" ]]; then
  run_spanda replay "$TRACE" --at T+00:01 --inspect decisions --json >/dev/null
fi

echo "Later differentiation smoke OK"
