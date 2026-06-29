#!/usr/bin/env bash
# Record Smart Spaces field soak start date for stable promotion.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOAK_DIR="$ROOT/.spanda"
SOAK_FILE="${SPANDA_SMART_SPACES_FIELD_SOAK_START_FILE:-$SOAK_DIR/smart-spaces-field-soak-start.txt}"

mkdir -p "$SOAK_DIR"
DATE="$(date -u +%Y-%m-%d)"
echo "$DATE" > "$SOAK_FILE"
echo "Smart Spaces field soak clock started: $DATE"
echo "File: $SOAK_FILE"
