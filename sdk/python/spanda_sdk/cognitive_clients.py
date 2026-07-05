"""Cognitive & Resilience Architecture domain SDK clients."""

from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from spanda_sdk.client import SpandaClient


class ReflexClient:
    def __init__(self, client: SpandaClient) -> None:
        self._client = client

    def list(self) -> Any:
        return self._client.list_autonomy_reflex()

    def traces(self) -> Any:
        return self._client.list_autonomy_reflex_traces()


class HomeostasisClient:
    def __init__(self, client: SpandaClient) -> None:
        self._client = client

    def summary(self) -> Any:
        return self._client.get_autonomy_homeostasis()


class ImmunityClient:
    def __init__(self, client: SpandaClient) -> None:
        self._client = client

    def scan(self) -> Any:
        return self._client.scan_autonomy_immunity()


class AttentionClient:
    def __init__(self, client: SpandaClient) -> None:
        self._client = client

    def queue(self) -> Any:
        return self._client.get_autonomy_attention()


class FusionClient:
    def __init__(self, client: SpandaClient) -> None:
        self._client = client

    def summary(self) -> Any:
        return self._client.get_autonomy_fusion()


class MemoryClient:
    def __init__(self, client: SpandaClient) -> None:
        self._client = client

    def summary(self) -> Any:
        return self._client.get_autonomy_memory()

    def entity_refs(self, entity_id: str) -> Any:
        return self._client.get_entity_autonomy(entity_id)
