import { describe, it, expect } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { tokenize } from "../src/lexer/index.js";
import { parse } from "../src/parser/index.js";

const repoRoot = join(import.meta.dirname, "..");
const capabilityVerification = readFileSync(
  join(repoRoot, "examples/hardware/capability_verification.sd"),
  "utf-8",
);

describe("capability verification parsing (Phase 27)", () => {
  it("parses capability_verification.sd AST fields", () => {
    const program = parse(tokenize(capabilityVerification));

    expect(program.hardwareProfiles).toHaveLength(1);
    const hw = program.hardwareProfiles[0];
    expect(hw?.name).toBe("RoverV1");
    expect(hw?.sensors).toEqual(["GPS", "Lidar"]);
    expect(hw?.actuators).toEqual(["DifferentialDrive"]);
    expect(hw?.connectivity).toEqual(["WiFi", "LTE"]);

    expect(program.killSwitches).toHaveLength(1);
    const killSwitch = program.killSwitches[0];
    expect(killSwitch?.name).toBe("EmergencyStop");
    expect(killSwitch?.priority).toBe("critical");
    expect(killSwitch?.body.length).toBeGreaterThan(0);

    expect(program.requiresCapabilities).toHaveLength(1);
    const reqCap = program.requiresCapabilities[0];
    expect(reqCap?.capability).toBe("gps_navigation");
    expect(reqCap?.anyOfSensors).toEqual(["GPS", "GNSS"]);
    expect(reqCap?.anyOfActuators).toEqual(["DifferentialDrive"]);

    expect(program.robots).toHaveLength(1);
    const robot = program.robots[0];
    expect(robot?.name).toBe("Rover");
    expect(robot?.usesHardware).toBe("RoverV1");
    expect(robot?.exposesCapabilities).toEqual([
      "autonomous_navigation",
      "gps_navigation",
      "obstacle_avoidance",
      "telemetry_streaming",
    ]);
    expect(robot?.mission?.name).toBe("Patrol");
    expect(robot?.mission?.requiredCapabilities).toEqual([
      "gps_navigation",
      "obstacle_avoidance",
    ]);
    expect(robot?.mission?.steps).toEqual(["patrol_loop"]);

    expect(program.healthChecks).toHaveLength(1);
    const healthCheck = program.healthChecks[0];
    expect(healthCheck?.name).toBe("RoverHealth");
    expect(healthCheck?.targetKind).toBe("robot");
    expect(healthCheck?.target).toBe("Rover");
    expect(healthCheck?.conditions).toHaveLength(2);
    expect(healthCheck?.conditions[0]?.metric).toBe("gps.status");
    expect(healthCheck?.conditions[0]?.operator).toBe("==");
    expect(healthCheck?.conditions[0]?.threshold).toBe("Healthy");
    expect(healthCheck?.conditions[1]?.metric).toBe("wheels.emergency_stop_supported");
    expect(healthCheck?.conditions[1]?.threshold).toBe("true");

    expect(program.healthPolicies).toHaveLength(1);
    const policy = program.healthPolicies[0];
    expect(policy?.name).toBe("SafetyPolicy");
    expect(policy?.reactions).toHaveLength(2);
    expect(policy?.reactions[0]?.[0]).toBe("Critical");
    expect(policy?.reactions[1]?.[0]).toBe("Unsafe");

    expect(program.tests).toHaveLength(1);
    expect(program.tests[0]?.name).toBe("rover exposes gps navigation");
  });
});
