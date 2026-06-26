#!/usr/bin/env bash
# Optional macOS codesign + notarization for Tauri Control Center bundles.
# Requires Apple Developer credentials in CI secrets or local keychain.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUNDLE_DIR="${ROOT}/packages/control-center-desktop/src-tauri/target/release/bundle/macos"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "[sign-tauri-macos] skip: not macOS"
  exit 0
fi

if [[ ! -d "${BUNDLE_DIR}" ]]; then
  echo "[sign-tauri-macos] skip: no bundle at ${BUNDLE_DIR} (run TAURI_BUILD=1 first)"
  exit 0
fi

APP_PATH="$(find "${BUNDLE_DIR}" -maxdepth 1 -name '*.app' | head -n 1 || true)"
if [[ -z "${APP_PATH}" ]]; then
  echo "[sign-tauri-macos] skip: no .app in ${BUNDLE_DIR}"
  exit 0
fi

IDENTITY="${APPLE_SIGNING_IDENTITY:-}"
if [[ -z "${IDENTITY}" ]]; then
  echo "[sign-tauri-macos] skip: APPLE_SIGNING_IDENTITY unset"
  exit 0
fi

echo "[sign-tauri-macos] codesign ${APP_PATH}"
codesign --force --options runtime --sign "${IDENTITY}" "${APP_PATH}"
codesign --verify --deep --strict --verbose=2 "${APP_PATH}"

if [[ -n "${APPLE_NOTARIZE_PROFILE:-}" ]]; then
  echo "[sign-tauri-macos] notarize via notarytool profile ${APPLE_NOTARIZE_PROFILE}"
  ditto -c -k --keepParent "${APP_PATH}" /tmp/spanda-control-center.zip
  xcrun notarytool submit /tmp/spanda-control-center.zip \
    --keychain-profile "${APPLE_NOTARIZE_PROFILE}" \
    --wait
  xcrun stapler staple "${APP_PATH}"
fi

echo "[sign-tauri-macos] OK"
