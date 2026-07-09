"""Minimal protobuf encode/decode for Control Center gRPC mesh RPCs."""

from __future__ import annotations


def _encode_varint(value: int) -> bytes:
    out = bytearray()
    while value > 0x7F:
        out.append((value & 0x7F) | 0x80)
        value >>= 7
    out.append(value & 0x7F)
    return bytes(out)


def _decode_varint(data: bytes, index: int) -> tuple[int, int]:
    shift = 0
    result = 0
    while index < len(data):
        byte = data[index]
        index += 1
        result |= (byte & 0x7F) << shift
        if (byte & 0x80) == 0:
            return result, index
        shift += 7
    raise ValueError("truncated varint")


def _encode_string_field(field_number: int, value: str) -> bytes:
    payload = value.encode("utf-8")
    tag = _encode_varint((field_number << 3) | 2)
    length = _encode_varint(len(payload))
    return tag + length + payload


def encode_empty() -> bytes:
    return b""


def encode_query_request(query: str) -> bytes:
    if not query:
        return b""
    return _encode_string_field(1, query)


def encode_body_json(body_json: str) -> bytes:
    return _encode_string_field(1, body_json)


def decode_json_response(data: bytes) -> str:
    index = 0
    while index < len(data):
        tag, index = _decode_varint(data, index)
        wire_type = tag & 0x07
        field_number = tag >> 3
        if wire_type == 2 and field_number == 1:
            length, index = _decode_varint(data, index)
            end = index + length
            return data[index:end].decode("utf-8")
        if wire_type == 0:
            _, index = _decode_varint(data, index)
        elif wire_type == 2:
            length, index = _decode_varint(data, index)
            index += length
        else:
            raise ValueError(f"unsupported wire type {wire_type}")
    return "{}"
