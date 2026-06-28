"""Spanda Control Center Python SDK.

For the full official SDK (program readiness, assure, diagnose, …), use
``sdk/python`` (`SpandaClient`). This package provides ``ControlCenterClient``
for enterprise Control Center operations.
"""

from spanda_sdk.client import ControlCenterClient
from spanda_sdk.stream import TelemetryStream

__all__ = ["ControlCenterClient", "TelemetryStream"]
