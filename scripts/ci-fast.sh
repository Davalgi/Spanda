#!/usr/bin/env bash
# Local mirror of the CI Fast workflow (.github/workflows/ci-fast.yml).
# Run before opening a PR or pushing to main.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== CI Fast: rustfmt =="
cargo fmt --all -- --check

echo "== CI Fast: clippy =="
cargo clippy --workspace -- -D warnings

echo "== CI Fast: spanda-core (no default features) =="
cargo build -p spanda-core --no-default-features

echo "== CI Fast: registry index =="
cargo run -q -p spanda-package --bin registry-index-maintain -- --verify

echo "== CI Fast: architecture manifest =="
python3 scripts/validate_architecture.py --check-manifest-sync

echo "== CI Fast: blueprints =="
python3 scripts/validate_blueprints.py

echo "== CI Fast: cross-surface coupling =="
./scripts/check_cross_surface.sh

echo "== CI Fast: rust tests =="
cargo test --workspace

echo "== CI Fast: python sdk =="
if ! python3 -c "import pytest" 2>/dev/null; then
  pip install pytest
fi
PYTHONPATH=sdk/python python3 -m pytest sdk/python

echo "== CI Fast: typescript root =="
npm test
npm run build

echo "== CI Fast: typescript sdk =="
npm ci --prefix sdk/typescript
npm test --prefix sdk/typescript

echo "== CI Fast: build spanda release =="
cargo build -p spanda --release

echo "== CI Fast: cross-interface consistency =="
SPANDA_BIN="${ROOT}/target/release/spanda" ./scripts/cross_interface_consistency.sh

echo "CI Fast passed."
