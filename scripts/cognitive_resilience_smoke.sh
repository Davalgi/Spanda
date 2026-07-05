#!/usr/bin/env bash
# Smoke Cognitive & Resilience Architecture — delegates to autonomy crate/CLI checks.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
exec "${ROOT}/scripts/bio_inspired_autonomy_smoke.sh"
