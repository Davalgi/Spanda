#!/usr/bin/env bash
# Autonomous Entity Mesh Experimental → Stable promotion gate.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

SOAK_FILE="${SPANDA_ENTITY_MESH_FIELD_SOAK_START_FILE:-$ROOT/.spanda/entity-mesh-field-soak-start.txt}"
MIN_DAYS="${SPANDA_FIELD_SOAK_MIN_DAYS:-30}"
AUDIT_FILE="${SPANDA_SECURITY_AUDIT_PREP_FILE:-$ROOT/.spanda/security-audit-prep.json}"

echo "== Entity mesh stable promotion gate =="

if [[ "${SPANDA_ENTITY_MESH_SKIP_SOAK:-0}" != "1" ]]; then
  echo "--- Entity Mesh field pilot soak (min ${MIN_DAYS} days) ---"
  if [[ ! -f "$SOAK_FILE" ]]; then
    echo "missing soak start file: $SOAK_FILE" >&2
    echo "Create with: ./scripts/entity_mesh_field_soak_init.sh" >&2
    echo "Pilot guide: docs/entity-mesh-field-pilot.md" >&2
    exit 1
  fi
  START_DATE="$(tr -d '[:space:]' < "$SOAK_FILE")"
  if date -u -j -f "%Y-%m-%d" "$START_DATE" "+%s" >/dev/null 2>&1; then
    START_EPOCH="$(date -u -j -f "%Y-%m-%d" "$START_DATE" "+%s")"
  else
    START_EPOCH="$(date -u -d "$START_DATE" "+%s")"
  fi
  NOW_EPOCH="$(date -u "+%s")"
  ELAPSED_DAYS=$(( (NOW_EPOCH - START_EPOCH) / 86400 ))
  echo "Soak started: $START_DATE (${ELAPSED_DAYS} days elapsed)"
  if (( ELAPSED_DAYS < MIN_DAYS )); then
    echo "Field soak incomplete: need $(( MIN_DAYS - ELAPSED_DAYS )) more day(s)" >&2
    exit 1
  fi
else
  echo "Skipping field soak (SPANDA_ENTITY_MESH_SKIP_SOAK=1)"
fi

if [[ "${SPANDA_ENTITY_MESH_SKIP_AUDIT:-0}" != "1" ]]; then
  echo "--- Security audit prep artifact ---"
  if [[ ! -f "$AUDIT_FILE" ]]; then
    echo "missing audit prep file: $AUDIT_FILE" >&2
    echo "Run: ./scripts/security_audit_prep.sh" >&2
    exit 1
  fi
  python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$AUDIT_FILE"
  echo "Security audit prep present (external reviewer sign-off still required)"
else
  echo "Skipping audit prep check (SPANDA_ENTITY_MESH_SKIP_AUDIT=1)"
fi

echo "--- spanda-entity-mesh crate tests ---"
cargo test -p spanda-entity-mesh --quiet

echo "--- Entity mesh smoke (CLI + REST + SDK + gRPC) ---"
"$ROOT/scripts/entity_mesh_smoke.sh"

echo ""
echo "Entity mesh stable promotion gate passed."
echo "Update docs/feature-status.md Autonomous Entity Mesh row to Stable after audit sign-off."
echo "See docs/entity-mesh-stable-promotion.md"
