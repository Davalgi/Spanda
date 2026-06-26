"""WebSocket telemetry stream client for Control Center."""

from __future__ import annotations

import json
import os
from typing import Any, Iterator, Optional


def _ws_url(base_url: str, path: str = "/v1/stream/telemetry") -> str:
    if base_url.startswith("https://"):
        return "wss://" + base_url[len("https://") :].rstrip("/") + path
    if base_url.startswith("http://"):
        return "ws://" + base_url[len("http://") :].rstrip("/") + path
    return "ws://" + base_url.rstrip("/") + path


class TelemetryStream:
    """Async-friendly iterator over Control Center telemetry WebSocket events."""

    def __init__(self, base_url: Optional[str] = None) -> None:
        self.base_url = (
            base_url
            or os.environ.get("SPANDA_CONTROL_CENTER_URL", "http://127.0.0.1:8080")
        ).rstrip("/")

    def iter_events(self, max_messages: int = 100) -> Iterator[dict[str, Any]]:
        try:
            import websockets.sync.client as ws_client
        except ImportError as error:
            raise RuntimeError(
                "WebSocket streaming requires optional dependency: pip install 'spanda-sdk[stream]'"
            ) from error

        url = _ws_url(self.base_url)
        with ws_client.connect(url, open_timeout=10) as websocket:
            count = 0
            while count < max_messages:
                raw = websocket.recv(timeout=10)
                if isinstance(raw, bytes):
                    raw = raw.decode("utf-8")
                payload = json.loads(raw)
                yield payload
                count += 1

    def wait_for_hello(self) -> dict[str, Any]:
        for event in self.iter_events(max_messages=1):
            if event.get("type") == "hello":
                return event
        raise RuntimeError("telemetry stream did not send hello")
