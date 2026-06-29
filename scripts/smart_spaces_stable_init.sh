#!/usr/bin/env bash
# One-shot Smart Spaces stable promotion prep: field soak clock + audit artifacts.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

chmod +x scripts/smart_spaces_field_soak_init.sh
chmod +x scripts/smart_spaces_security_audit_prep.sh
chmod +x scripts/smart_spaces_security_self_audit.sh

./scripts/smart_spaces_field_soak_init.sh
./scripts/smart_spaces_security_audit_prep.sh
./scripts/smart_spaces_security_self_audit.sh

echo "Smart Spaces stable init complete."
echo "After 30-day pilot: SPANDA_SMART_SPACES_SKIP_SOAK=0 SPANDA_SMART_SPACES_SKIP_AUDIT=0 ./scripts/smart_spaces_promotion_gate.sh"
