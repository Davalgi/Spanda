#!/usr/bin/env bash
# Prepare what-if analysis security audit intake artifact.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${ROOT}/.spanda/what-if-security-audit-prep.json"
mkdir -p "$(dirname "$OUT")"

echo "== What-if security audit prep =="

SMOKE_OK=false
if ./scripts/what_if_smoke.sh >/tmp/what-if-smoke-audit.log 2>&1; then
  SMOKE_OK=true
fi

GATE_OK=false
if SPANDA_WHATIF_SKIP_SOAK=1 SPANDA_WHATIF_SKIP_AUDIT=1 SPANDA_WHATIF_SKIP_SMOKE=1 \
  ./scripts/what_if_stable_promotion_gate.sh >/tmp/what-if-gate-audit.log 2>&1; then
  GATE_OK=true
fi

export ROOT SMOKE_OK GATE_OK
python3 - <<'PY' > "$OUT"
import json, os, time
report = {
    "generated_at_ms": int(time.time() * 1000),
    "scope": [
        "what_if_scenario_injection",
        "recovery_planner_composition",
        "control_center_analytics_what_if",
        "grpc_get_analytics_what_if",
    ],
    "checks": {
        "what_if_smoke": os.environ.get("SMOKE_OK") == "true",
        "promotion_gate_smoke": os.environ.get("GATE_OK") == "true",
    },
    "reviewer_packet": [
        "docs/stable-hardening-what-if.md",
        "docs/what-if-analysis.md",
        "examples/showcase/what_if/gps_failure.sd",
        "crates/spanda-whatif/tests/gps_failure.rs",
    ],
    "signed_off": False,
}
print(json.dumps(report, indent=2))
PY

echo "Wrote $OUT"
echo "See docs/stable-hardening-what-if.md"
