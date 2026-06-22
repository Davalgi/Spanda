#!/usr/bin/env bash
# Golden path for live IoT bridge handlers (mock fallback without hardware libraries).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
BRIDGE="${ROOT}/scripts/spanda_python_bridge.py"

echo "== modbus bridge mock path =="
unset SPANDA_LIVE_MODBUS
RESULT="$(printf '%s\n' '{"fn":"modbus_read_register","args":["127.0.0.1","502",40001]}' | python3 "${BRIDGE}")"
echo "${RESULT}" | grep -q '"ok": true'

echo "== opcua bridge mock path =="
RESULT="$(printf '%s\n' '{"fn":"opcua_read_node","args":["opc.tcp://127.0.0.1:4840","ns=2;s=Temperature"]}' | python3 "${BRIDGE}")"
echo "${RESULT}" | grep -q '"ok": true'

echo "Live IoT golden path complete."
