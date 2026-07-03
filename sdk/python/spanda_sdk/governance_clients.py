"""Operational governance SDK clients."""

from __future__ import annotations

from typing import Any, Mapping, Optional, TYPE_CHECKING

if TYPE_CHECKING:
    from .client import SpandaClient


class GovernanceClient:
    def __init__(self, client: "SpandaClient") -> None:
        self._client = client

    def summary(self) -> Any:
        return self._client._request("GET", "/v1/governance")

    def validate(self, body: Optional[Mapping[str, Any]] = None) -> Any:
        return self._client._request("POST", "/v1/governance/validate", body or {}, auth=True)


class ComplianceClient:
    def __init__(self, client: "SpandaClient") -> None:
        self._client = client

    def summary(self) -> Any:
        return self._client._request("GET", "/v1/compliance", auth=True)

    def check(self, body: Optional[Mapping[str, Any]] = None) -> Any:
        return self._client._request("POST", "/v1/compliance/check", body or {}, auth=True)


class CertificationClient:
    def __init__(self, client: "SpandaClient") -> None:
        self._client = client

    def list(self) -> Any:
        return self._client._request("GET", "/v1/certifications", auth=True)


class DeploymentProfileClient:
    def __init__(self, client: "SpandaClient") -> None:
        self._client = client

    def list(self) -> Any:
        return self._client._request("GET", "/v1/deployment-profiles")

    def get(self, name: str) -> Any:
        return self._client._request("GET", f"/v1/deployment-profiles?name={name}")


class RiskClient:
    def __init__(self, client: "SpandaClient") -> None:
        self._client = client

    def summary(self) -> Any:
        return self._client._request("GET", "/v1/risk", auth=True)
