#!/usr/bin/env bash
# Build the React Control Center SPA and sync into spanda-api embedded static assets.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WEB_DIR="${ROOT}/packages/web"
TARGET_DIR="${ROOT}/crates/spanda-api/src/static/control-center-ui"

echo "[control-center-ui] npm run build:control-center"
npm run build:control-center --workspace=@davalgi-spanda/web

echo "[control-center-ui] sync ${WEB_DIR}/dist-control-center -> ${TARGET_DIR}"
rm -rf "${TARGET_DIR}"
mkdir -p "${TARGET_DIR}"
cp -R "${WEB_DIR}/dist-control-center/." "${TARGET_DIR}/"

echo "[control-center-ui] synced $(find "${TARGET_DIR}" -type f | wc -l | tr -d ' ') files"
