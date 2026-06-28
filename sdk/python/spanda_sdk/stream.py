"""WebSocket telemetry stream for Control Center events."""

from __future__ import annotations

import asyncio
import json
import os
from typing import Any, AsyncIterator, Callable, Optional


class TelemetryStream:
    """Async WebSocket client for `WS /v1/stream/telemetry`."""

    def __init__(self, base_url: Optional[str] = None) -> None:
        http = base_url or os.environ.get(
            "SPANDA_CONTROL_CENTER_URL", "http://127.0.0.1:8080"
        )
        self.ws_url = http.replace("https://", "wss://").replace("http://", "ws://")
        if not self.ws_url.endswith("/v1/stream/telemetry"):
            self.ws_url = f"{self.ws_url.rstrip('/')}/v1/stream/telemetry"

    async def events(self) -> AsyncIterator[dict[str, Any]]:
        """Yield parsed JSON events from the telemetry stream."""
        try:
            import websockets
        except ImportError as exc:
            raise RuntimeError(
                "Install stream extras: pip install 'spanda-sdk[stream]'"
            ) from exc

        async with websockets.connect(self.ws_url) as ws:
            async for message in ws:
                yield json.loads(message)

    async def listen(self, handler: Callable[[dict[str, Any]], None]) -> None:
        """Invoke handler for each telemetry event."""
        async for event in self.events():
            handler(event)


def run_stream(handler: Callable[[dict[str, Any]], None], base_url: Optional[str] = None) -> None:
    """Blocking helper to run an event handler loop."""
    asyncio.run(TelemetryStream(base_url).listen(handler))
