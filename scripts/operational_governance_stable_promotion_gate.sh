#!/usr/bin/env bash
# Operational Governance Stable tier promotion gate.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

SOAK_FILE="${SPANDA_FIELD_SOAK_START_FILE:-$ROOT/.spanda/field-soak-start.txt}"
MIN_DAYS="${SPANDA_FIELD_SOAK_MIN_DAYS:-30}"

echo "== Operational governance stable promotion gate =="

if [[ "${SPANDA_GOVERNANCE_SKIP_SOAK:-0}" != "1" ]]; then
  echo "--- Field soak (min ${MIN_DAYS} days) ---"
  if [[ ! -f "$SOAK_FILE" ]]; then
    echo "missing soak start file: $SOAK_FILE" >&2
    echo "Create with: ./scripts/enterprise_ops_field_soak_init.sh" >&2
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
  echo "Skipping field soak (SPANDA_GOVERNANCE_SKIP_SOAK=1)"
fi

echo "--- Operational governance smoke ---"
chmod +x "$ROOT/scripts/operational_governance_smoke.sh"
"$ROOT/scripts/operational_governance_smoke.sh"

echo "--- Unit tests ---"
cargo test -p spanda-governance -q

echo "== Operational governance promotion gate OK =="
