#!/usr/bin/env bash
# Automated Smart Spaces security config self-audit (scaffold; not third-party certification).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SS="$ROOT/examples/solutions/smart-spaces"
OUT="${SPANDA_SMART_SPACES_SECURITY_SELF_AUDIT_FILE:-$ROOT/.spanda/smart-spaces-security-self-audit.json}"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "missing required file: $path" >&2
    exit 1
  fi
}

require_file "$SS/spanda.security.toml"
require_file "$SS/spanda.readiness.toml"
require_file "$SS/spanda.devices.toml"
require_file "$ROOT/docs/smart-space-security.md"

mkdir -p "$(dirname "$OUT")"
SIGNED="${SPANDA_SMART_SPACES_AUDIT_SIGNED_OFF:-0}"
python3 - "$OUT" "$SIGNED" <<'PY'
import json
import sys
from datetime import datetime, timezone

out, signed = sys.argv[1], sys.argv[2] == "1"
payload = {
    "blueprint": "smart_spaces",
    "audit_type": "automated_self_audit",
    "generated_at": datetime.now(timezone.utc).isoformat(),
    "checks": [
        {"id": "security_toml", "passed": True},
        {"id": "readiness_toml", "passed": True},
        {"id": "devices_toml", "passed": True},
        {"id": "security_doc", "passed": True},
    ],
    "signed_off": signed,
    "notes": "Automated config presence check; third-party life-safety review still required for Stable tier.",
}
with open(out, "w", encoding="utf-8") as fh:
    json.dump(payload, fh, indent=2)
    fh.write("\n")
print(f"Wrote {out} (signed_off={signed})")
PY
