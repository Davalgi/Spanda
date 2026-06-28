#!/usr/bin/env python3
"""Evaluate program readiness via the Spanda Python SDK."""

from spanda import SpandaClient


def main() -> None:
    client = SpandaClient.local()
    report = client.readiness("examples/robotics/rover.sd")
    score = report.get("report", {}).get("score", {})
    total = score.get("total") if isinstance(score, dict) else score
    print(f"Readiness score: {total}")


if __name__ == "__main__":
    main()
