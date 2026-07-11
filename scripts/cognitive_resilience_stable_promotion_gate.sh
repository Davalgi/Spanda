#!/usr/bin/env bash
# Cognitive & Resilience Stable promotion gate (implementation checks).
# Field soak remains an organizational gate — see docs/organizational-gates.md.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== Cognitive & Resilience stable promotion gate =="

echo "--- Cognitive resilience smoke ---"
./scripts/cognitive_resilience_smoke.sh

echo "--- Maintenance window CLI ---"
if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda --no-default-features -- "$@"; }
fi
run_spanda maintenance window list >/dev/null
run_spanda maintenance window set \
  --id gate-nightly \
  --start 2026-07-12T02:00:00Z \
  --end 2026-07-12T04:00:00Z \
  --activity ota >/dev/null
run_spanda maintenance window list --json | grep -q gate-nightly

echo "--- Cross-interface consistency (fusion/memory + SDK) ---"
./scripts/cross_interface_consistency.sh

echo "--- RBAC notes (autonomy mutations) ---"
cat <<'EOF'
RBAC audit (/v1/autonomy/*):
  - All historical autonomy routes are GET (read-only). Sensitive-read auth applies when
    SPANDA_API_REQUIRE_AUTH_READS=1 (prefix /v1/autonomy).
  - Mutation POST /v1/autonomy/maintenance/windows requires Operate (ensure_rbac).
  - No other /v1/autonomy/* mutations exist; Operate gate is enforced on the write path.
EOF

echo "--- Field soak (organizational; pending) ---"
echo "Field soak sign-off remains organizational per docs/organizational-gates.md."
echo "This gate does not fail on soak; track separately before production Stable claims."

echo "== Cognitive & Resilience stable promotion gate OK =="
