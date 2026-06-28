"""Structured errors for the Spanda Python SDK."""

from __future__ import annotations


class SpandaError(Exception):
    """Base SDK error."""

    def __init__(self, message: str, *, status: int | None = None) -> None:
        super().__init__(message)
        self.status = status


class ValidationError(SpandaError):
    """Invalid request or response payload."""


class ReadinessError(SpandaError):
    """Readiness evaluation failed."""


class VerificationError(SpandaError):
    """Hardware or capability verification failed."""


class SecurityError(SpandaError):
    """Authentication or authorization failure."""


class ConnectionError(SpandaError):
    """Network or server connectivity failure."""


class PermissionError(SpandaError):
    """Insufficient permissions for the requested operation."""
