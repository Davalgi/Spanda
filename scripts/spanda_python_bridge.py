#!/usr/bin/env python3
"""Spanda Python FFI bridge (subprocess protocol v1).

Reads JSON from stdin:
  {"fn": "py_add", "args": [1, 2]}

Writes JSON to stdout:
  {"ok": true, "result": 3}
  {"ok": false, "error": "..."}

Register handlers below or extend this module for project-specific bindings.
"""

from __future__ import annotations

import json
import sys
from typing import Any, Callable

Handler = Callable[..., Any]


def _openai_complete(prompt: str) -> str:
    import os

    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:
        return f"mock-completion:{prompt[:48]}"
    try:
        import urllib.request

        body = json.dumps(
            {
                "model": "gpt-4o-mini",
                "messages": [{"role": "user", "content": prompt}],
            }
        ).encode()
        req = urllib.request.Request(
            "https://api.openai.com/v1/chat/completions",
            data=body,
            headers={
                "Authorization": f"Bearer {api_key}",
                "Content-Type": "application/json",
            },
            method="POST",
        )
        with urllib.request.urlopen(req, timeout=30) as resp:  # noqa: S310
            data = json.loads(resp.read().decode())
        return data["choices"][0]["message"]["content"]
    except Exception as exc:  # noqa: BLE001
        return f"openai-error:{exc}"


HANDLERS: dict[str, Handler] = {
    "py_add": lambda a, b: int(a) + int(b),
    "py_echo": lambda x: x,
    "py_version": lambda: 1,
    "ros2_publish": lambda topic, data: {"topic": topic, "published": True, "bytes": len(str(data))},
    "mqtt_publish": lambda topic, payload: {"topic": topic, "published": True, "bytes": len(str(payload))},
    "openai_complete": _openai_complete,
}


def main() -> int:
    try:
        req = json.load(sys.stdin)
    except json.JSONDecodeError as exc:
        print(json.dumps({"ok": False, "error": f"Invalid JSON request: {exc}"}))
        return 0

    fn = req.get("fn")
    args = req.get("args", [])
    if not isinstance(fn, str):
        print(json.dumps({"ok": False, "error": "Missing fn string in request"}))
        return 0
    if not isinstance(args, list):
        print(json.dumps({"ok": False, "error": "args must be a JSON array"}))
        return 0

    handler = HANDLERS.get(fn)
    if handler is None:
        print(json.dumps({"ok": False, "error": f"Unknown python extern '{fn}'"}))
        return 0

    try:
        result = handler(*args)
    except Exception as exc:  # noqa: BLE001 — bridge boundary
        print(json.dumps({"ok": False, "error": str(exc)}))
        return 0

    print(json.dumps({"ok": True, "result": result}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
