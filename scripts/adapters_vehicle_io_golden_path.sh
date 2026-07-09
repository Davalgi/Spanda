#!/usr/bin/env bash
# Golden path for ADAS live vehicle I/O bridges (LIN, UDS, V2X) via env-gated CMD hooks.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export SPANDA_LIVE_LIN=1
export SPANDA_LIN_CMD='echo 33.0'
export SPANDA_LIVE_UDS=1
export SPANDA_UDS_CMD='echo P0300'
export SPANDA_LIVE_V2X=1
export SPANDA_V2X_CMD='echo cooperative_alert'

cargo test -p spanda-providers --test automotive_hub live_lin_cmd_overrides_hub_stub
cargo test -p spanda-providers --test automotive_hub live_uds_cmd_overrides_hub_stub
cargo test -p spanda-providers --test automotive_hub live_v2x_cmd_overrides_hub_stub

echo "✓ ADAS live vehicle I/O golden path"
