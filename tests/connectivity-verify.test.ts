import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { tokenize } from "../src/lexer/index.js";
import { parse } from "../src/parser/index.js";
import { verifyHardwareProgram } from "../src/hardware-verify.js";
import {
  faultToConnectivity,
  connectivityLinkToTransport,
  verifyRequiresConnectivity,
} from "../src/connectivity-positioning.js";

const examplesDir = join(import.meta.dirname, "..", "examples", "connectivity");

describe("connectivity verify (TS fallback)", () => {
  it("passes connectivity_hardware_verify against RoverV2", () => {
    const source = readFileSync(join(examplesDir, "connectivity_hardware_verify.sd"), "utf8");
    const program = parse(tokenize(source));
    const result = verifyHardwareProgram(program, { target: "RoverV2" });
    expect(result.ok).toBe(true);
    expect(result.target).toBe("RoverV2");
    expect(result.items.some((i) => i.severity === "pass")).toBe(true);
  });

  it("maps faults and link transports", () => {
    expect(faultToConnectivity("NetworkOutage")).toEqual({
      domain: "network",
      event: "disconnected",
    });
    expect(connectivityLinkToTransport("wifi")).toBe("mqtt");
    expect(connectivityLinkToTransport("cellular")).toBe("dds");
  });

  it("fails when required cellular missing from profile", () => {
    const program = parse(
      tokenize(`
requires_connectivity { cellular: required; }
hardware Tiny { connectivity [ WiFi6, GPS ]; }
robot R { actuator wheels: DifferentialDrive; }
deploy R to Tiny;
`),
    );
    const profile = program.hardwareProfiles[0]!;
    const items = verifyRequiresConnectivity(program.requiresConnectivity!, profile);
    expect(items.some((i) => i.severity === "error")).toBe(true);
  });
});
