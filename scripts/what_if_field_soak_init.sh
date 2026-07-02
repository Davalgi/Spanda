#!/usr/bin/env bash
# Start the 30-day what-if field soak clock for Stable promotion.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOAK_FILE="${SPANDA_WHATIF_FIELD_SOAK_START_FILE:-$ROOT/.spanda/what-if-field-soak-start.txt}"
mkdir -p "$(dirname "$SOAK_FILE")"

if [[ -f "$SOAK_FILE" ]]; then
  echo "What-if field soak already started: $(tr -d '[:space:]' < "$SOAK_FILE")" >&2
  echo "File: $SOAK_FILE" >&2
  exit 1
fi

date -u +%Y-%m-%d > "$SOAK_FILE"
echo "What-if field soak started: $(cat "$SOAK_FILE")"
echo "Wrote $SOAK_FILE"
echo "After 30 days run: ./scripts/what_if_stable_promotion_gate.sh"
echo "See docs/stable-hardening-what-if.md"
