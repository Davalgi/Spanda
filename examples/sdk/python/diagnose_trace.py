#!/usr/bin/env python3
"""Diagnose a mission trace via the Spanda Python SDK."""

from spanda import SpandaClient


def main() -> None:
    client = SpandaClient.local()
    report = client.diagnose("path/to/mission.trace")
    print(report.get("report", report))


if __name__ == "__main__":
    main()
