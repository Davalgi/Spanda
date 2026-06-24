import { describe, expect, it } from "vitest";
import { tokenize } from "../src/lexer/index.js";
import { parse } from "../src/parser/index.js";
import { evaluateRecoveryTs, formatRecoveryReport } from "../src/recovery.js";

const SOURCE = `
recovery_policy RoverRecovery {
  on gps.failed {
    switch_to visual_odometry;
    reduce_speed 0.5 m/s;
    enter degraded_mode;
  }
}

robot Rover {
  sensor gps: GPS;
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior patrol() { wheels.drive(0.3 m/s); }
}
`;

describe("recovery framework", () => {
  it("parses recovery_policy declarations", () => {
    const program = parse(tokenize(SOURCE));
    expect(program.recoveryPolicies).toHaveLength(1);
    expect(program.recoveryPolicies[0]?.branches[0]?.condition).toContain("gps");
  });

  it("evaluates heal workflow", () => {
    const program = parse(tokenize(SOURCE));
    const report = evaluateRecoveryTs(program, {
      issue: "gps.failed",
      diagnosis: "Satellite lock lost",
      level: 2,
    });
    expect(report.plans[0]?.diagnosis).toBe("Satellite lock lost");
    expect(formatRecoveryReport(report)).toContain("Safety Validation");
  });
});
