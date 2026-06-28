import { describe, expect, it } from "vitest";
import { SpandaClient } from "../src/client.js";
import { SpandaError } from "../src/errors.js";
import { ReadinessReport } from "../src/types.js";

describe("SpandaClient", () => {
  it("constructs local client", () => {
    const client = SpandaClient.local();
    expect(client.baseUrl).toContain("127.0.0.1");
  });

  it("maps permission errors", () => {
    const err = SpandaError.fromStatus(403, "forbidden");
    expect(err.name).toBe("PermissionError");
  });
});

describe("ReadinessReport", () => {
  it("extracts score from API envelope", () => {
    const report = ReadinessReport.fromApi({
      report: { score: { total: 88 }, status: "Ready" },
    });
    expect(report.score).toBe(88);
  });
});
