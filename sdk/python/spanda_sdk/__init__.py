"""Official Spanda Python SDK."""

from spanda_sdk.client import SpandaClient
from spanda_sdk.errors import SpandaError
from spanda_sdk.stream import TelemetryStream

try:
    from spanda_sdk.grpc_client import GrpcClient, parse_grpc_json
except ImportError:  # pragma: no cover - optional grpc extra
    GrpcClient = None  # type: ignore[assignment,misc]
    parse_grpc_json = None  # type: ignore[assignment,misc]

__all__ = ["SpandaClient", "SpandaError", "TelemetryStream", "GrpcClient", "parse_grpc_json"]
