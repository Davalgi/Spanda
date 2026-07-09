#!/usr/bin/env bash
# Start the 30-day Autonomous Entity Mesh field pilot soak clock for Stable promotion.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOAK_FILE="${SPANDA_ENTITY_MESH_FIELD_SOAK_START_FILE:-$ROOT/.spanda/entity-mesh-field-soak-start.txt}"
mkdir -p "$(dirname "$SOAK_FILE")"

if [[ -f "$SOAK_FILE" ]]; then
  echo "Entity Mesh field soak already started: $(tr -d '[:space:]' < "$SOAK_FILE")" >&2
  exit 1
fi

date -u +%Y-%m-%d > "$SOAK_FILE"
echo "Entity Mesh field soak started: $(cat "$SOAK_FILE")"
echo "Wrote $SOAK_FILE"
echo "Pilot guide: docs/entity-mesh-field-pilot.md"
