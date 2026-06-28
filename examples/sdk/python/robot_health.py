#!/usr/bin/env python3
"""Poll entity health from Control Center."""

from spanda import SpandaClient


def main() -> None:
    client = SpandaClient.local()
    entities = client.list_entities().get("entities", [])
    for entity in entities:
        entity_id = entity.get("id")
        if not entity_id or entity.get("kind") != "device":
            continue
        health = client.get_health(entity_id)
        print(f"{entity_id}: {health.get('health')}")


if __name__ == "__main__":
    main()
