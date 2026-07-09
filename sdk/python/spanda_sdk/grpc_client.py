"""Native gRPC client for Spanda Control Center (`spanda.v1.ControlCenter`)."""

from __future__ import annotations

import json
import os
from typing import Any, Callable, Optional

from spanda_sdk._grpc_proto import (
    decode_json_response,
    encode_body_json,
    encode_empty,
    encode_query_request,
)
from spanda_sdk.errors import ConnectionError, SpandaError


def parse_grpc_json(raw: str) -> Any:
    try:
        return json.loads(raw)
    except json.JSONDecodeError as exc:
        raise SpandaError(str(exc)) from exc


def _require_grpc():
    try:
        import grpc  # type: ignore[import-untyped]
    except ImportError as exc:
        raise RuntimeError(
            "Install gRPC extras: pip install 'spanda-sdk[grpc]'"
        ) from exc
    return grpc


class GrpcClient:
    """Sync gRPC client for Control Center mesh RPCs."""

    def __init__(self, channel: Any, api_key: Optional[str] = None) -> None:
        self._channel = channel
        self._api_key = api_key
        self._stubs: dict[str, Callable[..., Any]] = {}

    @classmethod
    def connect(
        cls,
        *,
        address: Optional[str] = None,
        api_key: Optional[str] = None,
    ) -> "GrpcClient":
        grpc = _require_grpc()
        endpoint = (
            address
            or os.environ.get("SPANDA_GRPC_URL")
            or os.environ.get("SPANDA_CONTROL_CENTER_GRPC")
            or "127.0.0.1:50051"
        )
        key = api_key or os.environ.get("SPANDA_API_KEY")
        channel = grpc.insecure_channel(endpoint)
        return cls(channel, key)

    @classmethod
    def local(cls) -> "GrpcClient":
        return cls.connect()

    def close(self) -> None:
        self._channel.close()

    def _metadata(self) -> tuple[tuple[str, str], ...]:
        if self._api_key:
            return (("authorization", f"Bearer {self._api_key}"),)
        return ()

    def _unary(self, method: str, request: bytes) -> Any:
        grpc = _require_grpc()
        if method not in self._stubs:
            self._stubs[method] = self._channel.unary_unary(
                f"/spanda.v1.ControlCenter/{method}",
                request_serializer=lambda payload: payload,
                response_deserializer=decode_json_response,
            )
        try:
            raw = self._stubs[method](request, metadata=self._metadata(), timeout=30)
        except grpc.RpcError as exc:
            raise ConnectionError(str(exc)) from exc
        return parse_grpc_json(raw)

    def get_mesh_topology(self) -> Any:
        return self._unary("GetMeshTopology", encode_empty())

    def get_mesh_nodes(self) -> Any:
        return self._unary("GetMeshNodes", encode_empty())

    def get_mesh_routes(self, query: str = "") -> Any:
        return self._unary("GetMeshRoutes", encode_query_request(query))

    def get_mesh_partitions(self) -> Any:
        return self._unary("GetMeshPartitions", encode_empty())

    def get_mesh_health(self) -> Any:
        return self._unary("GetMeshHealth", encode_empty())

    def get_mesh_graph(self) -> Any:
        return self._unary("GetMeshGraph", encode_empty())

    def get_mesh_merge_report(self) -> Any:
        return self._unary("GetMeshMergeReport", encode_empty())

    def discover_mesh(self, body: Optional[dict[str, Any]] = None) -> Any:
        payload = json.dumps(body or {})
        return self._unary("DiscoverMesh", encode_body_json(payload))

    def find_mesh_capability(self, capability: str) -> Any:
        payload = json.dumps({"capability": capability})
        return self._unary("FindMeshCapability", encode_body_json(payload))

    def simulate_mesh_partition(self, entity_ids: list[str]) -> Any:
        payload = json.dumps({"entity_ids": entity_ids})
        return self._unary("SimulateMeshPartition", encode_body_json(payload))
