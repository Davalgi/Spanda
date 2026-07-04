#!/usr/bin/env bash
# Smoke operational governance — examples, CLI, and expected pass/fail gates.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
GOV_ROOT="${ROOT}/examples/governance"

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  run_spanda() { "$SPANDA_BIN" "$@"; }
else
  cargo build -p spanda -q
  run_spanda() { cargo run -q -p spanda -- "$@"; }
fi

run_in_example() {
  local example="$1"
  shift
  (
    cd "${GOV_ROOT}/${example}"
    run_spanda "$@"
  )
}

expect_pass() {
  local example="$1"
  echo "== ${example}: governance validate (expect PASS) =="
  run_in_example "$example" governance validate
  echo "== ${example}: deployment verify (expect PASS) =="
  run_in_example "$example" deployment verify
  echo "== ${example}: compliance check (expect PASS) =="
  run_in_example "$example" compliance check
  echo "== ${example}: certification report =="
  run_in_example "$example" certification report >/dev/null
  echo "== ${example}: risk report =="
  run_in_example "$example" risk report >/dev/null
}

expect_fail() {
  local example="$1"
  local label="$2"
  echo "== ${example}: governance validate (expect FAIL — ${label}) =="
  if run_in_example "$example" governance validate; then
    echo "error: expected ${example} governance validate to fail (${label})" >&2
    exit 1
  fi
  echo "== ${example}: deployment verify (expect FAIL — ${label}) =="
  if run_in_example "$example" deployment verify; then
    echo "error: expected ${example} deployment verify to fail (${label})" >&2
    exit 1
  fi
}

echo "== deployment profiles list =="
run_spanda deployment profile >/dev/null
run_spanda deployment profile warehouse --json >/dev/null

echo "== governance framework =="
run_spanda governance framework >/dev/null
run_spanda governance framework --json >/dev/null

# Live-ready examples with complete accountability and operational certification.
expect_pass warehouse
expect_pass industrial-robot
expect_pass smart-building
expect_pass adas
expect_pass connected-healthcare

# Live maturity without operational certification must fail gates.
expect_fail hospital "pre_production requires validated/certified status"
expect_fail search-rescue "pilot requires validated/certified status"

echo "== operational governance smoke OK =="
