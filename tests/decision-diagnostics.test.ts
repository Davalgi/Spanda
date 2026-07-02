import { describe, expect, it } from "vitest";
import { tokenize } from "../src/lexer/index.js";
import { parse } from "../src/parser/index.js";
import { collectDecisionDiagnostics } from "../src/decision-diagnostics.js";
import { readinessDiagnostics } from "../src/readiness.js";

describe("decision diagnostics", () => {
  it("warns when decision_tree has no branches", () => {
    const program = parse(
      tokenize(`
decision_tree Empty local {}
robot R {
  sensor gps: GPS;
  actuator w: DifferentialDrive;
  safety { max_speed = 1 m/s; }
  behavior b() {}
}
`),
    );
    const diags = collectDecisionDiagnostics(program);
    expect(diags.some((d) => d.category === "decision:tree")).toBe(true);
  });

  it("flags conflicting local and central approval", () => {
    const program = parse(
      tokenize(`
robot R {
  local_decision_authority [emergency_stop];
  requires_central_approval [emergency_stop];
  sensor gps: GPS;
  actuator w: DifferentialDrive;
  safety { max_speed = 1 m/s; }
  behavior b() {}
}
`),
    );
    const diags = collectDecisionDiagnostics(program);
    expect(diags.some((d) => d.category === "decision:authority" && d.severity === "error")).toBe(
      true,
    );
  });

  it("merges decision diagnostics into readinessDiagnostics", () => {
    const source = `
robot R {
  local_decision_authority [degraded_mode];
  sensor gps: GPS;
  actuator w: DifferentialDrive;
  safety { max_speed = 1 m/s; }
  behavior b() {}
}
`;
    const items = readinessDiagnostics(source);
    expect(items.some((d) => d.category === "decision:authority")).toBe(true);
  });
});
