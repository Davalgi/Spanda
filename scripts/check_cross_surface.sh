#!/usr/bin/env bash
# Fail when a public-surface change touches one layer without required companion layers.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

BASE="${1:-}"
if [[ -z "$BASE" ]]; then
  if [[ -n "${GITHUB_BASE_REF:-}" ]]; then
    BASE="origin/${GITHUB_BASE_REF}"
  elif git rev-parse --verify origin/main >/dev/null 2>&1; then
    BASE="origin/main"
  else
    BASE="HEAD~1"
  fi
fi

CHANGED=()
while IFS= read -r line; do
  CHANGED+=("$line")
done < <(git diff --name-only "${BASE}...HEAD" 2>/dev/null || git diff --name-only HEAD~1 HEAD)

if [[ ${#CHANGED[@]} -eq 0 ]]; then
  echo "check_cross_surface: no changed files (base=${BASE}); ok"
  exit 0
fi

matches_any() {
  local pattern="$1"
  shift
  local path
  for path in "$@"; do
    if [[ "$path" == $pattern ]]; then
      return 0
    fi
  done
  return 1
}

PROTO_TOUCH=false
API_TOUCH=false
CLI_TOUCH=false
TS_MIRROR_TOUCH=false
SDK_TOUCH=false

for path in "${CHANGED[@]}"; do
  matches_any "crates/spanda-api/proto/*" "$path" && PROTO_TOUCH=true
  matches_any "crates/spanda-api/src/*" "$path" && API_TOUCH=true
  matches_any "crates/spanda-cli/src/*" "$path" && CLI_TOUCH=true
  matches_any "src/*" "$path" && TS_MIRROR_TOUCH=true
  if matches_any "sdk/python/*" "$path" \
    || matches_any "sdk/typescript/*" "$path" \
    || matches_any "crates/spanda-sdk/*" "$path"; then
    SDK_TOUCH=true
  fi
done

fail() {
  echo "check_cross_surface: $1" >&2
  echo "See docs/ci-architecture.md#cross-surface-change-protocol" >&2
  exit 1
}

if $PROTO_TOUCH && ! $API_TOUCH; then
  fail "proto changed without matching crates/spanda-api/src changes"
fi

if $API_TOUCH && ! $SDK_TOUCH; then
  fail "API changed without SDK updates (sdk/python, sdk/typescript, or crates/spanda-sdk)"
fi

if $API_TOUCH && ! $CLI_TOUCH; then
  echo "check_cross_surface: note — API changed without crates/spanda-cli/src edits (ok for read-only routes)"
fi

if $CLI_TOUCH && ! $TS_MIRROR_TOUCH; then
  echo "check_cross_surface: note — CLI changed without src/ mirror edits (ok when CLI-only)"
fi

echo "check_cross_surface: ok (${#CHANGED[@]} changed files, base=${BASE})"
