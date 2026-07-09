#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH="${HOME}/.cargo/bin:${PATH}"

ensure_wasm_pack() {
  if command -v wasm-pack >/dev/null 2>&1; then
    return 0
  fi

  echo "wasm-pack not found; installing..."

  if curl --proto '=https' --tlsv1.2 -fsSf \
    https://rustwasm.github.io/wasm-pack/installer/init.sh | sh; then
    :
  elif command -v cargo >/dev/null 2>&1; then
    cargo install wasm-pack --locked
  else
    echo "Failed to install wasm-pack: install Rust from https://rustup.rs" >&2
    exit 1
  fi

  if ! command -v wasm-pack >/dev/null 2>&1; then
    echo "Failed to install wasm-pack. Try: cargo install wasm-pack" >&2
    exit 1
  fi
}

ensure_wasm_target() {
  if ! command -v rustup >/dev/null 2>&1; then
    return 0
  fi
  rustup target add wasm32-unknown-unknown >/dev/null
}

ensure_wasm_pack
ensure_wasm_target

wasm-pack build crates/spanda-wasm --target web --out-dir "$ROOT/packages/web/wasm" --release

echo "WASM built to packages/web/wasm/"
