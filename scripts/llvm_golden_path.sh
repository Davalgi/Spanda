#!/usr/bin/env bash
# Golden path for LLVM native codegen (requires clang and spanda-cli with llvm feature).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

SPANDA="${SPANDA_BIN:-$ROOT/target/release/spanda}"
OUT="${TMPDIR:-/tmp}/spanda-llvm-golden"

if ! command -v clang >/dev/null 2>&1; then
  echo "clang not found; skip LLVM golden path" >&2
  exit 0
fi

cargo build -p spanda --release --features llvm

for SOURCE in examples/hello_world.sd examples/showcase/autonomous_rover/rover.sd; do
  if [[ ! -f "$SOURCE" ]]; then
    echo "skip missing $SOURCE" >&2
    continue
  fi
  OUT_BIN="${OUT}-$(basename "${SOURCE%.sd}")"
  "$SPANDA" check "$SOURCE"
  "$SPANDA" llvm-ir "$SOURCE" --out "${OUT_BIN}.ll"
  "$SPANDA" compile-native "$SOURCE" --out "$OUT_BIN"
  test -x "$OUT_BIN"
  echo "✓ LLVM golden path: $SOURCE -> $OUT_BIN"
done
