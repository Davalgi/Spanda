#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOAK_FILE="${SPANDA_TRUST_FRAMEWORK_FIELD_SOAK_START_FILE:-$ROOT/.spanda/trust-framework-field-soak-start.txt}"
mkdir -p "$(dirname "$SOAK_FILE")"
if [[ -f "$SOAK_FILE" ]]; then
  echo "Trust framework soak already started: $(tr -d '[:space:]' < "$SOAK_FILE")" >&2
  exit 1
fi
date -u +%Y-%m-%d > "$SOAK_FILE"
echo "Trust framework field soak started: $(cat "$SOAK_FILE")"
echo "Wrote $SOAK_FILE"
