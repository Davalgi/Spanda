#!/usr/bin/env bash
# Prepare Trust Framework security audit intake artifact.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${ROOT}/.spanda/trust-framework-security-audit-prep.json"
mkdir -p "$(dirname "$OUT")"

echo "== Trust framework security audit prep =="

SMOKE_OK=false
if ./scripts/trust_program_smoke.sh >/tmp/trust-framework-smoke-audit.log 2>&1; then
  SMOKE_OK=true
fi

GATE_OK=false
if SPANDA_TRUST_FRAMEWORK_SKIP_SOAK=1 SPANDA_TRUST_FRAMEWORK_SKIP_AUDIT=1 SPANDA_TRUST_FRAMEWORK_SKIP_SMOKE=1 \
  ./scripts/trust_framework_stable_promotion_gate.sh >/tmp/trust-framework-gate-audit.log 2>&1; then
  GATE_OK=true
fi

export ROOT SMOKE_OK GATE_OK
python3 - <<'PY' > "$OUT"
import json, os, time
report = {
    "generated_at_ms": int(time.time() * 1000),
    "scope": [
        "composite_program_trust_scoring",
        "package_and_mission_integrity",
        "control_center_trust_program_api",
        "grpc_get_trust_program",
    ],
    "checks": {
        "trust_program_smoke": os.environ.get("SMOKE_OK") == "true",
        "promotion_gate_smoke": os.environ.get("GATE_OK") == "true",
    },
    "reviewer_packet": [
        "docs/stable-hardening-trust-framework.md",
        "docs/trust-framework.md",
        "examples/showcase/tamper_policy/rover.sd",
        "crates/spanda-trust/src/composite.rs",
    ],
    "signed_off": False,
}
print(json.dumps(report, indent=2))
PY

echo "Wrote $OUT"
echo "See docs/stable-hardening-trust-framework.md"
