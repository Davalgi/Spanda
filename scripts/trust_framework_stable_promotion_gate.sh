#!/usr/bin/env bash
# Trust Framework (signature capability) Stable tier promotion gate.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# shellcheck source=scripts/lib/control_center_smoke_lib.sh
source "$ROOT/scripts/lib/control_center_smoke_lib.sh"

SOAK_FILE="${SPANDA_TRUST_FRAMEWORK_FIELD_SOAK_START_FILE:-$ROOT/.spanda/trust-framework-field-soak-start.txt}"
MIN_DAYS="${SPANDA_TRUST_FRAMEWORK_FIELD_SOAK_MIN_DAYS:-30}"
PROGRAM="$ROOT/examples/showcase/tamper_policy/rover.sd"

echo "== Trust framework stable promotion gate =="

if [[ "${SPANDA_TRUST_FRAMEWORK_SKIP_SOAK:-1}" != "1" ]]; then
  if [[ ! -f "$SOAK_FILE" ]]; then
    echo "missing soak file: $SOAK_FILE — run ./scripts/trust_framework_field_soak_init.sh" >&2
    exit 1
  fi
  START_DATE="$(tr -d '[:space:]' < "$SOAK_FILE")"
  if date -u -j -f "%Y-%m-%d" "$START_DATE" "+%s" >/dev/null 2>&1; then
    START_EPOCH="$(date -u -j -f "%Y-%m-%d" "$START_DATE" "+%s")"
  else
    START_EPOCH="$(date -u -d "$START_DATE" "+%s")"
  fi
  ELAPSED_DAYS=$(( ($(date -u "+%s") - START_EPOCH) / 86400 ))
  if (( ELAPSED_DAYS < MIN_DAYS )); then
    echo "Trust framework field soak incomplete: need $(( MIN_DAYS - ELAPSED_DAYS )) more day(s)" >&2
    exit 1
  fi
else
  echo "Skipping field soak (SPANDA_TRUST_FRAMEWORK_SKIP_SOAK=1)"
fi

AUDIT_FILE="${SPANDA_TRUST_FRAMEWORK_SECURITY_AUDIT_PREP_FILE:-$ROOT/.spanda/trust-framework-security-audit-prep.json}"
if [[ "${SPANDA_TRUST_FRAMEWORK_SKIP_AUDIT:-1}" != "1" ]]; then
  if [[ ! -f "$AUDIT_FILE" ]]; then
    echo "missing audit prep file: $AUDIT_FILE — run ./scripts/trust_framework_security_audit_prep.sh" >&2
    exit 1
  fi
  python3 -c 'import json,sys; json.load(open(sys.argv[1]))' "$AUDIT_FILE"
else
  echo "Skipping audit prep check (SPANDA_TRUST_FRAMEWORK_SKIP_AUDIT=1)"
fi

if [[ "${SPANDA_TRUST_FRAMEWORK_SKIP_SMOKE:-0}" != "1" ]]; then
  "$ROOT/scripts/trust_program_smoke.sh"
  "$ROOT/scripts/trust_showcase_smoke.sh"
else
  echo "Skipping smoke (SPANDA_TRUST_FRAMEWORK_SKIP_SMOKE=1)"
fi

cargo test -p spanda-trust --quiet

grep -q '^\*\*Status:\*\* Stable' "$ROOT/docs/trust-framework.md" || {
  echo "docs/trust-framework.md must declare Status: Stable" >&2
  exit 1
}

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')
BIND="127.0.0.1:${PORT}"
cleanup() { cc_smoke_stop_listener; }
cc_smoke_trap cleanup
CC_SMOKE_BIND="$BIND"
run_spanda control-center serve --bind "$BIND" --program "$PROGRAM" &
sleep 2

ENCODED="$(python3 -c 'import urllib.parse,sys; print(urllib.parse.quote(sys.argv[1]))' "$PROGRAM")"
body="$(curl -sf --max-time 15 "http://${BIND}/v1/trust/program?file=${ENCODED}")"
echo "$body" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert d.get("version")=="v1", d; assert "report" in d, d'

echo "Trust framework stable promotion gate passed."
