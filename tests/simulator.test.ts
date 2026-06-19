import { describe, it, expect } from "vitest";
import { createDefaultSimulator } from "../src/simulator/index.js";

describe("simulator", () => {
  it("updates pose on drive command", () => {
    const sim = createDefaultSimulator();
    sim.executeMotion({ kind: "drive", linear: 1.0, angular: 0, actuator: "wheels" });
    sim.tick(1000);
    const state = sim.getState();
    expect(state.pose.x).toBeCloseTo(1.0, 1);
  });

  it("simulates lidar nearest distance", () => {
    const sim = createDefaultSimulator({
      initialPose: { x: 0, y: 0, theta: 0 },
      obstacles: [{ x: 3, y: 0, radius: 0.5 }],
    });
    const reading = sim.readSensor("lidar", "Lidar");
    expect(reading.kind).toBe("scan");
    if (reading.kind === "scan") {
      expect(reading.nearestDistance).toBeCloseTo(2.5, 1);
    }
  });

  it("stops motion on emergency stop", () => {
    const sim = createDefaultSimulator();
    sim.executeMotion({ kind: "drive", linear: 1.0, angular: 0, actuator: "wheels" });
    sim.setEmergencyStop(true);
    sim.tick(1000);
    expect(sim.getState().velocity.linear).toBe(0);
  });

  it("simulates drone altitude with thrust", () => {
    const sim = createDefaultSimulator({ initialPose: { x: 0, y: 0, theta: 0, z: 1.0 } });
    sim.executeMotion({ kind: "set_thrust", thrust: 0.8, actuator: "rotors" });
    sim.tick(500);
    expect(sim.getState().pose.z).toBeGreaterThan(1.0);
  });

  it("tracks arm move_to position", () => {
    const sim = createDefaultSimulator();
    sim.executeMotion({ kind: "move_to", x: 0.5, y: 0.3, z: 0.2, actuator: "arm" });
    expect(sim.getArmPosition()).toEqual({ x: 0.5, y: 0.3, z: 0.2 });
  });

  it("logs motion events", () => {
    const sim = createDefaultSimulator();
    sim.executeMotion({ kind: "stop", actuator: "wheels" });
    expect(sim.getEventLog()).toContain("stop()");
  });
});
