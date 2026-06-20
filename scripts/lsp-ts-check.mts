#!/usr/bin/env node
/** JSON type-check for LSP when Rust CLI is unavailable. Usage: node --import tsx scripts/lsp-ts-check.mts <file.sd> */
import { readFileSync } from "node:fs";
import { tokenize } from "../src/lexer/index.js";
import { parse } from "../src/parser/index.js";
import { typeCheck, TypeCheckError } from "../src/types/index.js";

const path = process.argv[2];
if (!path) {
  console.log(JSON.stringify({ ok: false, diagnostics: [{ message: "missing file path", line: 1, column: 1 }] }));
  process.exit(1);
}

try {
  const source = readFileSync(path, "utf-8");
  typeCheck(parse(tokenize(source)));
  console.log(JSON.stringify({ ok: true, diagnostics: [] }));
} catch (err) {
  if (err instanceof TypeCheckError) {
    console.log(JSON.stringify({ ok: false, diagnostics: err.errors }));
  } else {
    const message = err instanceof Error ? err.message : String(err);
    console.log(JSON.stringify({ ok: false, diagnostics: [{ message, line: 1, column: 1 }] }));
  }
}
