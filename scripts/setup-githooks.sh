#!/usr/bin/env bash
# Point git at repo-managed hooks under .githooks/
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

mkdir -p .githooks
cp scripts/githooks/pre-push .githooks/pre-push
git config core.hooksPath .githooks
chmod +x .githooks/pre-push scripts/ci-fast.sh scripts/check_cross_surface.sh scripts/githooks/pre-push

echo "Git hooks path set to .githooks (pre-push: fmt + cross-surface check)"
echo "Full PR gate locally: ./scripts/ci-fast.sh"
