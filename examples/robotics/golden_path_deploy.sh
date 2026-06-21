#!/usr/bin/env bash
# Golden-path robotics workflow: certify, deploy, fleet, swarm, and adapter verify.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
if [[ -n "${SPANDA:-}" ]]; then
  :
elif [[ -x "${ROOT}/target/release/spanda" ]]; then
  SPANDA="${ROOT}/target/release/spanda"
else
  SPANDA="spanda"
fi
export SPANDA

CERTIFIED="${ROOT}/examples/robotics/certified_deployment.sd"
REMOTE="${ROOT}/examples/robotics/remote_ota_deployment.sd"
FLEET="${ROOT}/examples/robotics/fleet_peer_missions.sd"
SWARM="${ROOT}/examples/robotics/swarm_coordination.sd"
NAV2_PKG="${ROOT}/examples/packages/nav2_adapter_package"

echo "== check certified deployment =="
"${SPANDA}" check "${CERTIFIED}"

echo "== verify with strict certify =="
"${SPANDA}" verify "${CERTIFIED}" --all-targets --strict-certify

echo "== certification proof artifact =="
"${SPANDA}" certify prove "${CERTIFIED}" --strict --out /tmp/spanda-certified-proof.json

echo "== deploy plan with certification summary =="
"${SPANDA}" deploy plan "${CERTIFIED}" --version 1.0.0

echo "== dry-run rollout with --require-certify =="
"${SPANDA}" deploy rollout "${CERTIFIED}" --require-certify --dry-run --version 1.0.0

echo "== remote OTA example (plan only) =="
"${SPANDA}" deploy plan "${REMOTE}" --version 1.3.0

echo "== verify Nav2 adapter package =="
"${SPANDA}" verify-adapter --project "${NAV2_PKG}" --import navigation.nav2

echo "== fleet orchestration =="
"${SPANDA}" fleet orchestrate "${FLEET}"

echo "== swarm coordination (round_robin tick 1) =="
"${SPANDA}" swarm coordinate "${SWARM}"

echo "== swarm coordination (round_robin tick 2) =="
"${SPANDA}" swarm coordinate "${SWARM}"

echo "Robotics golden path complete."
