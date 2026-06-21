import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { compileFile } from "../src/compile.js";
import { orchestrateFleets } from "../src/fleet-orchestrator.js";

const fleetExample = join(import.meta.dirname, "..", "examples/robotics/fleet_management.sd");

describe("fleet orchestrator (TS mirror)", () => {
  it("orchestrates declared fleet groups", () => {
    const { program } = compileFile(fleetExample, "typescript");
    const result = orchestrateFleets(program, "fleet_management.sd");
    expect(result.success).toBe(true);
    expect(result.fleets).toHaveLength(1);
    expect(result.fleets[0]?.fleetName).toBe("Warehouse");
    expect(result.fleets[0]?.members).toHaveLength(2);
  });
});
