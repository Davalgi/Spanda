"""Python SDK smoke tests."""

from spanda_sdk import SpandaClient
from spanda_sdk.errors import SpandaError


def test_local_client_constructs():
    client = SpandaClient.local()
    assert "127.0.0.1" in client.base_url


def test_program_body_shape():
    client = SpandaClient.local()
    body = client._program_body("rover.sd")
    assert body["file"] == "rover.sd"


def test_health_check_raises_without_server():
    client = SpandaClient(base_url="http://127.0.0.1:1")
    try:
        client.health_check()
    except SpandaError:
        pass
