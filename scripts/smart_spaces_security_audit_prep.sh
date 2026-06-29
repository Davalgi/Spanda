#!/usr/bin/env bash
# Generate Smart Spaces security audit intake artifact (experimental scaffold).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="$ROOT/.spanda"
OUT_FILE="${SPANDA_SMART_SPACES_SECURITY_AUDIT_PREP_FILE:-$OUT_DIR/smart-spaces-security-audit-prep.json}"

mkdir -p "$OUT_DIR"
python3 - "$OUT_FILE" <<'PY'
import json
import sys
from datetime import datetime, timezone

out = sys.argv[1]
payload = {
    "blueprint": "smart_spaces",
    "tier": "experimental",
    "generated_at": datetime.now(timezone.utc).isoformat(),
    "scope": [
        "facility_readiness",
        "gateway_failover",
        "life_safety_emergency",
        "human_health_opt_in",
        "access_control",
    ],
    "config_paths": [
        "examples/solutions/smart-spaces/spanda.security.toml",
        "examples/solutions/smart-spaces/spanda.readiness.toml",
        "docs/smart-space-security.md",
    ],
    "notes": "Scaffold tier — audit prep artifact for promotion gate; not a certification claim.",
}
with open(out, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
print(f"Wrote {out}")
PY
