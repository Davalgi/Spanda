#!/usr/bin/env bash
# Golden path for GPS/IMU/camera sensor pipelines (hub stubs + env-gated CMD bridges).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export SPANDA_LIVE_GPS=1
export SPANDA_GPS_CMD='echo 37.1,-122.2,15.0,180.0'
export SPANDA_LIVE_IMU=1
export SPANDA_IMU_CMD='echo 0.0,0.0,1.0,0,0,9.81'
export SPANDA_LIVE_CAMERA=1
export SPANDA_CAMERA_CMD='echo 640,480,0.25'

cargo test -p spanda-providers --test sensor_hub live_gps_cmd_overrides_hub_stub
cargo test -p spanda-providers --test sensor_hub live_imu_cmd_overrides_hub_stub
cargo test -p spanda-providers --test sensor_hub live_camera_cmd_overrides_hub_stub
cargo test -p spanda-providers --test sensor_hub live_fusion_sensor_readings_use_gps_imu_camera
cargo test -p spanda-runtime --test execution_runtime_tests

echo "✓ GPS/IMU/camera sensor pipeline golden path"
