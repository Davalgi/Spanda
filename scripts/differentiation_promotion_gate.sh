#!/usr/bin/env bash
# Differentiation pillars promotion gate (Experimental tier — all 15 areas).
# Runs full differentiation smoke, showcase parse checks, and topic-guide presence.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

echo "== Differentiation promotion gate (Experimental) =="

if [[ "${SPANDA_DIFFERENTIATION_SKIP_SMOKE:-0}" != "1" ]]; then
  echo "--- differentiation_smoke.sh ---"
  ./scripts/differentiation_smoke.sh
else
  echo "Skipping smoke (SPANDA_DIFFERENTIATION_SKIP_SMOKE=1)"
fi

echo "--- Showcase parse (15 pillars) ---"
SHOWCASES=(
  "examples/showcase/differentiation/warehouse.sd"
  "examples/showcase/differentiation/warehouse.sd"
  "examples/showcase/differentiation/decision_trail/main.sd"
  "examples/showcase/differentiation/warehouse.sd"
  "examples/showcase/differentiation/warehouse.sd"
  "examples/showcase/what_if/gps_failure.sd"
  "examples/showcase/risk/deployment_risk.sd"
  "examples/showcase/forecast/degradation.sd"
  "examples/showcase/trust_graph/rover.sd"
  "examples/showcase/scorecard/executive.sd"
  "examples/showcase/mission_twin/patrol.sd"
  "examples/showcase/certify/deployment_bundle/rover.sd"
  "examples/showcase/differentiation/decision_trail/main.sd"
  "examples/showcase/human_robot/approval_escalation.sd"
  "examples/showcase/governance/night_ops.sd"
)
for file in "${SHOWCASES[@]}"; do
  test -f "$file" || { echo "missing showcase: $file" >&2; exit 1; }
  run_spanda check "$file" >/dev/null
done

TRACE="examples/showcase/differentiation/decision_trail/main.trace"
test -f "$TRACE" || { echo "missing golden trace: $TRACE" >&2; exit 1; }
run_spanda replay "$TRACE" --at T+00:01 --inspect decisions --json >/dev/null

echo "--- Topic guides ---"
DOCS=(
  "docs/mission-contracts.md"
  "docs/explainability.md"
  "docs/decision-audit-trail.md"
  "docs/safety-coverage.md"
  "docs/recovery-coverage.md"
  "docs/what-if-analysis.md"
  "docs/mission-risk-analysis.md"
  "docs/readiness-forecast.md"
  "docs/trust-graph.md"
  "docs/scorecards.md"
  "docs/digital-mission-twin.md"
  "docs/certification-packs.md"
  "docs/mission-time-travel.md"
  "docs/human-robot-teaming.md"
  "docs/autonomous-governance.md"
)
for doc in "${DOCS[@]}"; do
  test -f "$doc" || { echo "missing doc: $doc" >&2; exit 1; }
  grep -q '^\*\*Status:\*\*' "$doc" || { echo "missing Status line: $doc" >&2; exit 1; }
done

grep -q 'Exit met' docs/differentiation-roadmap.md || {
  echo "differentiation-roadmap.md missing Phase exit markers" >&2
  exit 1
}

echo ""
echo "Differentiation promotion gate passed."
