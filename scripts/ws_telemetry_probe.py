#!/usr/bin/env python3
"""Receive one WebSocket telemetry hello frame (stdlib only)."""

import base64
import hashlib
import os
import socket
import struct
import sys
import time


def recv_frame(sock: socket.socket, timeout_s: float = 15.0) -> str:
    deadline = time.monotonic() + timeout_s
    while time.monotonic() < deadline:
        sock.settimeout(max(0.1, deadline - time.monotonic()))
        try:
            header = sock.recv(2)
        except TimeoutError:
            continue
        if len(header) < 2:
            continue
        length = header[1] & 0x7F
        if length == 126:
            length = struct.unpack("!H", _recv_exact(sock, 2))[0]
        elif length == 127:
            length = struct.unpack("!Q", _recv_exact(sock, 8))[0]
        payload = _recv_exact(sock, length)
        return payload.decode("utf-8", errors="replace")
    raise TimeoutError("timed out waiting for websocket frame")


def _recv_exact(sock: socket.socket, size: int) -> bytes:
    chunks: list[bytes] = []
    remaining = size
    while remaining > 0:
        chunk = sock.recv(remaining)
        if not chunk:
            raise RuntimeError("short websocket read")
        chunks.append(chunk)
        remaining -= len(chunk)
    return b"".join(chunks)


def main() -> None:
    url = sys.argv[1]
    assert url.startswith("ws://"), url
    rest = url[len("ws://") :]
    host_port, _, path = rest.partition("/")
    host, _, port_text = host_port.partition(":")
    port = int(port_text or "80")
    path = "/" + path

    key = base64.b64encode(os.urandom(16)).decode("ascii")
    request = (
        f"GET {path} HTTP/1.1\r\n"
        f"Host: {host}:{port}\r\n"
        "Upgrade: websocket\r\n"
        "Connection: Upgrade\r\n"
        f"Sec-WebSocket-Key: {key}\r\n"
        "Sec-WebSocket-Version: 13\r\n\r\n"
    )
    sock = socket.create_connection((host, port), timeout=5)
    sock.sendall(request.encode("ascii"))
    response = sock.recv(4096).decode("ascii", errors="replace")
    if "101" not in response:
        raise RuntimeError(f"upgrade failed: {response[:200]}")
    accept_key = key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
    digest = base64.b64encode(hashlib.sha1(accept_key.encode("ascii")).digest()).decode("ascii")
    if digest not in response:
        raise RuntimeError("invalid Sec-WebSocket-Accept")

    frame = recv_frame(sock)
    if '"type":"hello"' not in frame.replace(" ", ""):
        raise RuntimeError(f"unexpected frame: {frame}")
    print(frame)


if __name__ == "__main__":
    main()
