#!/usr/bin/env python3
"""Receive one WebSocket telemetry hello frame (stdlib only)."""

import base64
import hashlib
import os
import socket
import struct
import sys


def recv_frame(sock: socket.socket) -> str:
    header = sock.recv(2)
    if len(header) < 2:
        raise RuntimeError("short websocket header")
    length = header[1] & 0x7F
    if length == 126:
        length = struct.unpack("!H", sock.recv(2))[0]
    elif length == 127:
        length = struct.unpack("!Q", sock.recv(8))[0]
    payload = sock.recv(length)
    return payload.decode("utf-8", errors="replace")


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
