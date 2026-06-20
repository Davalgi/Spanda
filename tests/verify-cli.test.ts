import { describe, it, expect } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { isCliAvailable, verifyViaCli } from "../src/rust-bridge.js";

const repoRoot = join(import.meta.dirname, "..");
const roverDeploy = readFileSync(
  join(repoRoot, "examples/hardware/rover_deploy.sd"),
  "utf-8",
);

describe("spanda verify CLI", () => {
  it.skipIf(!isCliAvailable())("passes for rover_deploy against deploy target", () => {
    const result = verifyViaCli(roverDeploy);
    expect(result.ok).toBe(true);
    expect(result.compatible).toBe(true);
    expect(result.target).toBe("RoverV1");
    expect(result.items.length).toBeGreaterThan(0);
    expect(result.items.some((i) => i.severity === "pass")).toBe(true);
  });

  it.skipIf(!isCliAvailable())("fails for ESP32 target override", () => {
    const result = verifyViaCli(roverDeploy, ["--target", "ESP32"]);
    expect(result.ok).toBe(false);
    expect(result.compatible).toBe(false);
    expect(result.target).toBe("ESP32");
    expect(result.items.some((i) => i.severity === "error")).toBe(true);
  });

  it.skipIf(!isCliAvailable())("returns matrix with --all-targets", () => {
    const result = verifyViaCli(roverDeploy, ["--all-targets"]);
    expect(result.matrix?.cells.length).toBeGreaterThan(0);
    const roverOnV1 = result.matrix?.cells.find(
      (c) => c.robot === "RoverProgram" && c.target === "RoverV1",
    );
    const roverOnEsp = result.matrix?.cells.find(
      (c) => c.robot === "RoverProgram" && c.target === "ESP32",
    );
    expect(roverOnV1?.compatible).toBe(true);
    expect(roverOnEsp?.compatible).toBe(false);
  });
});
