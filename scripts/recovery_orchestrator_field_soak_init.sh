#!/usr/bin/env bash
# Initialize Recovery Orchestrator field soak timer.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SOAK_DIR="${SPANDA_CONTROL_CENTER_STATE_DIR:-$ROOT/.spanda}"
SOAK_FILE="${SPANDA_RECOVERY_FIELD_SOAK_START_FILE:-$SOAK_DIR/recovery-field-soak-start.txt}"
mkdir -p "$(dirname "$SOAK_FILE")"
date -u +%Y-%m-%d >"$SOAK_FILE"
echo "Recovery field soak started: $(cat "$SOAK_FILE")"
echo "File: $SOAK_FILE"
