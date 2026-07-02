#!/usr/bin/env bash
# Start the 30-day LATER differentiation field soak clock for Stable promotion.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOAK_FILE="${SPANDA_LATER_FIELD_SOAK_START_FILE:-$ROOT/.spanda/later-field-soak-start.txt}"
mkdir -p "$(dirname "$SOAK_FILE")"

if [[ -f "$SOAK_FILE" ]]; then
  echo "LATER field soak already started: $(tr -d '[:space:]' < "$SOAK_FILE")" >&2
  exit 1
fi

date -u +%Y-%m-%d > "$SOAK_FILE"
echo "LATER differentiation field soak started: $(cat "$SOAK_FILE")"
echo "Wrote $SOAK_FILE"
echo "After 30 days run: ./scripts/later_differentiation_stable_promotion_gate.sh"
