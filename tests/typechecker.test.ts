import { describe, it, expect } from "vitest";
import { compile } from "../src/compile.js";
import { TypeCheckError } from "../src/types/index.js";

describe("type checker", () => {
  it("accepts valid robot program", () => {
    expect(() =>
      compile(`
        robot R {
          sensor lidar: Lidar;
          actuator wheels: DifferentialDrive;
          safety { max_speed = 1.5 m/s; }
          behavior go() {
            let d = lidar.read().nearest_distance;
            wheels.drive(linear: 0.5 m/s, angular: 0.0 rad/s);
          }
        }
      `),
    ).not.toThrow();
  });

  it("rejects unit mismatch in drive args", () => {
    expect(() =>
      compile(`
        robot R {
          actuator wheels: DifferentialDrive;
          behavior go() {
            wheels.drive(linear: 0.5 m, angular: 0.0 rad/s);
          }
        }
      `),
    ).toThrow(TypeCheckError);
  });

  it("rejects unknown sensor type", () => {
    expect(() =>
      compile(`
        robot R {
          sensor cam: UnknownSensor;
        }
      `),
    ).toThrow(TypeCheckError);
  });

  it("checks stop_if boolean condition", () => {
    expect(() =>
      compile(`
        robot R {
          sensor lidar: Lidar;
          safety {
            stop_if lidar.read().nearest_distance < 0.5 m;
          }
        }
      `),
    ).not.toThrow();
  });

  it("rejects non-boolean if condition", () => {
    expect(() =>
      compile(`
        robot R {
          behavior b() {
            if 42 { }
          }
        }
      `),
    ).toThrow(TypeCheckError);
  });
});
