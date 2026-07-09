/**
 * Marketplace stub — keeps monorepo tsx helpers out of the published VSIX bundle.
 * @module
 */

export function runSymbolsMonorepo(): null {
  return null;
}

export function checkSourceTsMonorepo(): Array<{ message: string; line: number; column: number }> {
  return [{ message: "Install the Spanda CLI for type diagnostics", line: 1, column: 1 }];
}

export function readinessTsFallbackMonorepo(): never[] {
  return [];
}
