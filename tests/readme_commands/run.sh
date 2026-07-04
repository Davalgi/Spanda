#!/usr/bin/env bash
# README command smoke tests and optional golden-output comparison.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

MODE="smoke"
if [[ "${1:-}" == "--golden" ]]; then
  MODE="golden"
fi

if [[ -n "${SPANDA_BIN:-}" && -x "${SPANDA_BIN}" ]]; then
  SPANDA="${SPANDA_BIN}"
elif [[ -x "${ROOT}/target/release/spanda" ]]; then
  SPANDA="${ROOT}/target/release/spanda"
elif [[ -x "${ROOT}/target/debug/spanda" ]]; then
  SPANDA="${ROOT}/target/debug/spanda"
else
  cargo build -p spanda
  SPANDA="${ROOT}/target/debug/spanda"
fi

export SPANDA ROOT MODE
export SPANDA_UPDATE_GOLDENS="${SPANDA_UPDATE_GOLDENS:-}"
exec python3 "${ROOT}/tests/readme_commands/run.py"
