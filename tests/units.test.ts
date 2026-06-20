import { describe, it, expect } from "vitest";
import { tokenize } from "../src/lexer/index.js";
import { compile } from "../src/compile.js";
import { convertValue, unitsCompatible, alignForBinary } from "../src/units/index.js";

describe("physical units", () => {
  it("tokenizes extended unit suffixes", () => {
    const tokens = tokenize("100 cm 2.5 kg 36 km/h 25 celsius");
    const units = tokens.filter((t) => t.type === "UNIT_LITERAL").map((t) => t.unit);
    expect(units).toEqual(["cm", "kg", "km/h", "celsius"]);
  });

  it("converts compatible units", () => {
    expect(convertValue(100, "cm", "m")).toBeCloseTo(1);
    expect(convertValue(36, "km/h", "m/s")).toBeCloseTo(10);
    expect(convertValue(32, "fahrenheit", "celsius")).toBeCloseTo(0);
  });

  it("aligns mixed duration operands", () => {
    const aligned = alignForBinary(500, "ms", 0.5, "s");
    expect(aligned).toEqual([500, 500, "ms"]);
  });

  it("rejects incompatible units", () => {
    expect(unitsCompatible("m", "kg")).toBe(false);
    expect(convertValue(1, "m", "kg")).toBeUndefined();
  });

  it("tokenizes humidity and illuminance suffixes", () => {
    const tokens = tokenize("65 %RH 320 lux 800 ppm");
    const units = tokens.filter((t) => t.type === "UNIT_LITERAL").map((t) => t.unit);
    expect(units).toEqual(["%RH", "lux", "ppm"]);
  });

  it("type-checks full environmental sensor units", () => {
    expect(() =>
      compile(`
        robot R {
          actuator wheels: DifferentialDrive;
          behavior run() {
            let uv: UvIndex = 6.5 uvi;
            let acidity: Ph = 7.2 pH;
            let ec: Conductivity = 850 uS/cm;
            let pm25: ParticulateMatter = 12 ug/m3;
            let turbidity: Turbidity = 4.5 NTU;
            let salt: Salinity = 35 ppt;
            let dose: Radiation = 0.12 uSv/h;
            let soil: SoilMoisture = 42 %VWC;
            wheels.stop();
          }
        }
      `),
    ).not.toThrow();
  });
});
