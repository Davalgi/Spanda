#!/usr/bin/env bash
# Return 0 when git diff includes Control Center UI, API, CLI, or desktop shell paths.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT}"

BASE="${1:-origin/main}"
if ! git rev-parse --verify "${BASE}" >/dev/null 2>&1; then
  BASE="HEAD~1"
fi

mapfile -t CHANGED < <(git diff --name-only "${BASE}...HEAD" 2>/dev/null || git diff --name-only HEAD~1 HEAD)

for path in "${CHANGED[@]}"; do
  case "${path}" in
    packages/control-center-desktop/*|packages/web/*|crates/spanda-api/src/control_center*|crates/spanda-api/src/static/control-center-ui/*|crates/spanda-cli/src/control_center*|scripts/sync_control_center_embedded_ui.sh|scripts/control_center_*|scripts/verify_desktop_release_ready.sh)
      exit 0
      ;;
  esac
done

exit 1
