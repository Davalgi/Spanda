#!/usr/bin/env python3
"""Minimal OTLP HTTP collector stub for smoke tests (accepts POST /v1/traces)."""

from http.server import BaseHTTPRequestHandler, HTTPServer
import sys


class Handler(BaseHTTPRequestHandler):
    def do_POST(self) -> None:
        length = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(length)
        if b"resourceSpans" not in body:
            self.send_response(400)
            self.end_headers()
            return
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"{}")

    def log_message(self, format: str, *args: object) -> None:
        return


def main() -> None:
    port = int(sys.argv[1])
    HTTPServer(("127.0.0.1", port), Handler).serve_forever()


if __name__ == "__main__":
    main()
