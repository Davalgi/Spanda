"""Python gRPC client tests."""

import json

import pytest

from spanda_sdk.grpc_client import GrpcClient, parse_grpc_json
from spanda_sdk._grpc_proto import (
    decode_json_response,
    encode_body_json,
    encode_empty,
    encode_query_request,
)
from spanda_sdk.errors import SpandaError


def test_parse_grpc_json_valid():
    assert parse_grpc_json('{"version":"v1"}') == {"version": "v1"}


def test_parse_grpc_json_invalid():
    with pytest.raises(SpandaError):
        parse_grpc_json("{bad")


def test_proto_roundtrip_json_response():
    payload = json.dumps({"health": {"total_nodes": 2}})
    encoded = encode_body_json(payload)
    assert decode_json_response(encoded) == payload


def test_proto_query_request():
    assert encode_query_request("source=a&target=b").startswith(b"\x0a")


def test_proto_empty():
    assert encode_empty() == b""


def test_get_mesh_health_uses_grpc_stub(monkeypatch):
    captured: dict[str, object] = {}

    class FakeRpc:
        def __call__(self, request, metadata=(), timeout=30):
            captured["request"] = request
            captured["metadata"] = metadata
            return json.dumps({"health": {"total_nodes": 1}})

    class FakeChannel:
        def unary_unary(self, path, request_serializer, response_deserializer):
            captured["path"] = path
            return FakeRpc()

        def close(self):
            captured["closed"] = True

    monkeypatch.setattr(
        "spanda_sdk.grpc_client._require_grpc",
        lambda: pytest.importorskip("grpc"),
    )
    client = GrpcClient(FakeChannel(), api_key="test-key")
    result = client.get_mesh_health()
    assert result["health"]["total_nodes"] == 1
    assert captured["path"] == "/spanda.v1.ControlCenter/GetMeshHealth"
    client.close()
    assert captured.get("closed") is True
