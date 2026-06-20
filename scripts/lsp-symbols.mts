#!/usr/bin/env node
/** LSP symbol index + definition lookup. Usage:
 *  node --import tsx scripts/lsp-symbols.mts index <file.sd>
 *  node --import tsx scripts/lsp-symbols.mts define <file.sd> <line> <column>
 *  node --import tsx scripts/lsp-symbols.mts hover <file.sd> <line> <column>
 */
import { readFileSync } from "node:fs";
import {
  formatHover,
  indexSource,
  resolveDefinition,
  symbolAtPosition,
} from "../src/lsp/symbols.js";

const [, , cmd, file, lineArg, colArg] = process.argv;

if (!cmd || !file) {
  console.log(JSON.stringify({ error: "usage: index|define|hover <file> [line column]" }));
  process.exit(1);
}

const source = readFileSync(file, "utf-8");

if (cmd === "index") {
  const index = indexSource(source);
  console.log(JSON.stringify({ symbols: index.symbols }));
  process.exit(0);
}

const line = Number(lineArg ?? 1);
const column = Number(colArg ?? 1);

if (cmd === "define") {
  const sym = resolveDefinition(source, line, column);
  console.log(JSON.stringify(sym ? { symbol: sym } : { symbol: null }));
  process.exit(0);
}

if (cmd === "hover") {
  const index = indexSource(source);
  const sym = symbolAtPosition(index, line, column) ?? resolveDefinition(source, line, column);
  console.log(JSON.stringify(sym ? { markdown: formatHover(sym) } : { markdown: null }));
  process.exit(0);
}

console.log(JSON.stringify({ error: `unknown command ${cmd}` }));
process.exit(1);
