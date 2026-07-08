import { describe, expect, it } from "vitest";
import { parseGrpcJson } from "../src/grpcClient.js";
import { ConnectionError, SpandaError } from "../src/errors.js";

describe("parseGrpcJson", () => {
  it("parses mesh health payloads", () => {
    const value = parseGrpcJson('{"version":"v1","health":{"total_nodes":3}}');
    expect(value).toEqual({ version: "v1", health: { total_nodes: 3 } });
  });

  it("throws SpandaError on invalid JSON", () => {
    expect(() => parseGrpcJson("{")).toThrow(SpandaError);
  });
});

describe("GrpcClient", () => {
  it("maps connection failures to ConnectionError", () => {
    const err = new ConnectionError("14 UNAVAILABLE");
    expect(err.name).toBe("ConnectionError");
  });
});
