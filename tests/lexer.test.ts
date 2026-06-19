import { describe, it, expect } from "vitest";
import { tokenize } from "../src/lexer/index.js";

describe("lexer", () => {
  it("tokenizes robot declaration keywords", () => {
    const tokens = tokenize("robot Rover { sensor lidar: Lidar; }");
    const types = tokens.map((t) => t.type);
    expect(types).toContain("ROBOT");
    expect(types).toContain("SENSOR");
    expect(types).toContain("IDENT");
    expect(types[types.length - 1]).toBe("EOF");
  });

  it("tokenizes attached unit literals", () => {
    const tokens = tokenize("1.5m/s");
    const unitTok = tokens.find((t) => t.type === "UNIT_LITERAL");
    expect(unitTok).toBeDefined();
    expect(unitTok!.value).toBe(1.5);
    expect(unitTok!.unit).toBe("m/s");
  });

  it("tokenizes spaced number and unit as unit literal", () => {
    const tokens = tokenize("1.5 m/s");
    const unitTok = tokens.find((t) => t.type === "UNIT_LITERAL");
    expect(unitTok).toBeDefined();
    expect(unitTok!.value).toBe(1.5);
    expect(unitTok!.unit).toBe("m/s");
  });

  it("tokenizes duration units", () => {
    const tokens = tokenize("loop every 50ms");
    const msTok = tokens.find((t) => t.type === "UNIT_LITERAL");
    expect(msTok).toBeDefined();
    expect(msTok!.unit).toBe("ms");
    expect(msTok!.value).toBe(50);
  });

  it("tokenizes stop_if keyword", () => {
    const tokens = tokenize("stop_if x < 0.5 m;");
    expect(tokens.some((t) => t.type === "STOP_IF")).toBe(true);
  });

  it("skips line comments", () => {
    const tokens = tokenize("// comment\nrobot R {}");
    expect(tokens[0].type).toBe("ROBOT");
  });

  it("tokenizes comparison operators", () => {
    const tokens = tokenize("< <= > >= == !=");
    const types = tokens.filter((t) => t.type !== "EOF").map((t) => t.type);
    expect(types).toEqual(["LT", "LTE", "GT", "GTE", "EQ", "NEQ"]);
  });
});
