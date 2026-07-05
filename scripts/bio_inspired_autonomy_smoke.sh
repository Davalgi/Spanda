#!/usr/bin/env bash
# Smoke Cognitive & Resilience autonomy: crate tests, CLI reports, cross-domain integration.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  run_spanda() { cargo run -q -p spanda --no-default-features -- "$@"; }
fi

echo "== spanda-autonomy crate tests =="
cargo test -p spanda-autonomy --quiet

echo "== cognitive & resilience cross-domain integration =="
cargo test -p spanda-autonomy --test cognitive_resilience_integration --quiet

echo "== reflex CLI =="
run_spanda reflex list >/dev/null
run_spanda reflex simulate emergency >/dev/null
run_spanda reflex trace obstacle >/dev/null

echo "== homeostasis CLI =="
run_spanda homeostasis check >/dev/null
run_spanda homeostasis report >/dev/null

echo "== immunity CLI =="
run_spanda immunity scan >/dev/null

echo "== fusion / confidence CLI =="
run_spanda fusion check >/dev/null
run_spanda confidence report >/dev/null

echo "== alerts CLI =="
run_spanda alerts analyze >/dev/null
run_spanda alerts fatigue-report >/dev/null

echo "== recovery confidence CLI =="
run_spanda recovery confidence >/dev/null

echo "== entity autonomy profile (warehouse fixture) =="
run_spanda entity list --config crates/spanda-config/tests/fixtures/warehouse/spanda.toml >/dev/null

echo "== cognitive & resilience autonomy smoke OK =="
