#!/usr/bin/env bash
# Helpers for smoke scripts that background `spanda control-center serve`.
# Kills the listener on CC_SMOKE_BIND (not only the cargo wrapper PID) on EXIT/INT/TERM.

cc_smoke_stop_listener() {
  # Stop the TCP listener recorded in CC_SMOKE_BIND.
  local bind="${CC_SMOKE_BIND:-}"
  if [[ -z "$bind" ]]; then
    return 0
  fi
  local port="${bind##*:}"
  local pids=""
  if command -v lsof >/dev/null 2>&1; then
    pids="$(lsof -nP -iTCP:"$port" -sTCP:LISTEN -t 2>/dev/null || true)"
  fi
  if [[ -n "$pids" ]]; then
    # shellcheck disable=SC2086
    kill $pids 2>/dev/null || true
    sleep 0.2
    # shellcheck disable=SC2086
    kill -9 $pids 2>/dev/null || true
  fi
  if [[ -n "${CC_SMOKE_WRAPPER_PID:-}" ]]; then
    kill "$CC_SMOKE_WRAPPER_PID" 2>/dev/null || true
  fi
}

cc_smoke_trap() {
  # Register cleanup on normal exit and interruption.
  local handler="${1:-cc_smoke_stop_listener}"
  trap "$handler" EXIT INT TERM
}
