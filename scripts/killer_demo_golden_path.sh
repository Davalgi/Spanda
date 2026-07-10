#!/usr/bin/env bash
# Golden path for the v0.5 beta killer demo (safety + verify + sim).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

SPANDA="${SPANDA_BIN:-$ROOT/target/release/spanda}"
UNSAFE="${ROOT}/examples/showcase/ai_safety_violation.sd"
UNSAFE_DRIVE="${ROOT}/examples/showcase/ai_safety_drive_bypass.sd"
SAFE="${ROOT}/examples/showcase/killer_demo.sd"

if [[ ! -x "${SPANDA}" ]]; then
  cargo build -p spanda --release
  SPANDA="${ROOT}/target/release/spanda"
fi

echo "== unsafe AI must fail check =="
if "${SPANDA}" check "${UNSAFE}" >/dev/null 2>&1; then
  echo "expected compile error for ${UNSAFE}" >&2
  exit 1
fi
echo "✓ ${UNSAFE} rejected (ActionProposal → execute gate)"

echo "== unsafe AI drive bypass must fail check =="
if "${SPANDA}" check "${UNSAFE_DRIVE}" >/dev/null 2>&1; then
  echo "expected compile error for ${UNSAFE_DRIVE}" >&2
  exit 1
fi
echo "✓ ${UNSAFE_DRIVE} rejected (ActionProposal → drive gate)"

echo "== safe killer demo check =="
"${SPANDA}" check "${SAFE}"

echo "== hardware verify =="
"${SPANDA}" verify "${SAFE}"
"${SPANDA}" verify "${SAFE}" --json | grep -q '"compatible":true'

echo "== simulation =="
"${SPANDA}" sim "${SAFE}"

echo "== verify with simulate_compatibility fault =="
"${SPANDA}" verify "${SAFE}" --simulate

echo "Killer demo golden path complete."
