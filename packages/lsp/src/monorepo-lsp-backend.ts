/**
 * Monorepo-only LSP helpers (tsx scripts). Kept out of the marketplace server bundle.
 * @module
 */

import { spawnSync } from "node:child_process";
import { unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

type CliDiagnostic = { message: string; line: number; column: number };

type CompatItem = {
  message: string;
  line: number;
  column: number;
  severity: "pass" | "warning" | "error" | "info";
  category: string;
  suggested_fix?: string;
};

export function runSymbolsMonorepo(repoRoot: string, args: string[]): unknown {
  // Run the symbols indexer via the monorepo tsx script.
  const symbolScript = join(repoRoot, "scripts/lsp-symbols.mts");
  const result = spawnSync(process.execPath, ["--import", "tsx", symbolScript, ...args], {
    encoding: "utf-8",
    cwd: repoRoot,
  });

  // Return null when the script produced no stdout.
  if (!result.stdout?.trim()) {
    return null;
  }

  // Parse JSON output from the symbols script.
  try {
    return JSON.parse(result.stdout);
  } catch {
    return null;
  }
}

export function checkSourceTsMonorepo(repoRoot: string, source: string): CliDiagnostic[] {
  // Fall back to the TypeScript checker when the native CLI is unavailable.
  const tmp = join(repoRoot, ".spanda-lsp-ts-check.sd");
  writeFileSync(tmp, source);
  const script = join(repoRoot, "scripts/lsp-ts-check.mts");
  const result = spawnSync(process.execPath, ["--import", "tsx", script, tmp], {
    encoding: "utf-8",
    cwd: repoRoot,
  });

  // Remove the temporary source file.
  try {
    unlinkSync(tmp);
  } catch {
    /* ignore */
  }

  // Surface stderr when the checker produced no stdout.
  if (!result.stdout?.trim()) {
    return [{ message: result.stderr || "TypeScript check failed", line: 1, column: 1 }];
  }
  const parsed = JSON.parse(result.stdout) as { ok: boolean; diagnostics?: CliDiagnostic[] };
  return parsed.ok ? [] : (parsed.diagnostics ?? []);
}

export function readinessTsFallbackMonorepo(repoRoot: string, source: string): CompatItem[] {
  // Fall back to the readiness tsx script when the CLI JSON path is unavailable.
  const tmp = join(repoRoot, ".spanda-lsp-readiness.sd");
  writeFileSync(tmp, source);
  const script = join(repoRoot, "scripts/lsp-readiness.mts");
  const result = spawnSync(process.execPath, ["--import", "tsx", script, tmp], {
    encoding: "utf-8",
  });

  // Remove the temporary source file.
  try {
    unlinkSync(tmp);
  } catch {
    /* ignore */
  }

  // Return no items when the script produced no stdout.
  if (!result.stdout?.trim()) {
    return [];
  }
  const parsed = JSON.parse(result.stdout) as { ok: boolean; items?: CompatItem[] };
  return (parsed.items ?? []).map((item) => ({
    ...item,
    severity: item.severity === "info" ? "warning" : item.severity,
  }));
}
