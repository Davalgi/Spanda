#!/usr/bin/env node
import {
  createConnection,
  TextDocuments,
  Diagnostic,
  DiagnosticSeverity,
  ProposedFeatures,
  InitializeParams,
  TextDocumentSyncKind,
} from "vscode-languageserver/node.js";
import { TextDocument } from "vscode-languageserver-textdocument";
import { spawnSync } from "node:child_process";
import { existsSync, unlinkSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../../..");

function cliPath(): string | null {
  const release = join(repoRoot, "target/release/spanda");
  const debug = join(repoRoot, "target/debug/spanda");
  if (existsSync(release)) return release;
  if (existsSync(debug)) return debug;
  return null;
}

type CliDiagnostic = { message: string; line: number; column: number };

type CompatItem = {
  message: string;
  line: number;
  column: number;
  severity: "pass" | "warning" | "error";
  category: string;
};

function runCliJson(args: string[], source: string): unknown {
  const bin = cliPath();
  if (!bin) {
    return {
      ok: false,
      diagnostics: [
        {
          message: "Rust CLI not built — run: npm run build:rust",
          line: 1,
          column: 1,
        },
      ],
    };
  }

  const tmp = join(repoRoot, ".spanda-lsp-check.sd");
  writeFileSync(tmp, source);
  const result = spawnSync(bin, [...args, "--json", tmp], { encoding: "utf-8" });
  try {
    unlinkSync(tmp);
  } catch {
    /* ignore */
  }

  if (!result.stdout?.trim()) {
    return {
      ok: false,
      diagnostics: [{ message: result.stderr || "CLI failed", line: 1, column: 1 }],
    };
  }

  return JSON.parse(result.stdout);
}

function checkSource(source: string): CliDiagnostic[] {
  const parsed = runCliJson(["check"], source) as {
    ok: boolean;
    diagnostics?: CliDiagnostic[];
  };
  return parsed.ok ? [] : (parsed.diagnostics ?? []);
}

function verifySource(source: string): CompatItem[] {
  const parsed = runCliJson(["verify"], source) as {
    ok: boolean;
    items?: CompatItem[];
    diagnostics?: CliDiagnostic[];
  };
  if (parsed.items?.length) {
    return parsed.items.filter((i) => i.severity !== "pass");
  }
  if (!parsed.ok && parsed.diagnostics) {
    return parsed.diagnostics.map((d) => ({
      ...d,
      severity: "error" as const,
      category: "error",
    }));
  }
  return [];
}

const connection = createConnection(ProposedFeatures.all);
const documents = new TextDocuments(TextDocument);

connection.onInitialize((_params: InitializeParams) => ({
  capabilities: {
    textDocumentSync: TextDocumentSyncKind.Incremental,
  },
}));

function validate(textDocument: TextDocument): Diagnostic[] {
  const source = textDocument.getText();
  const typeErrors = checkSource(source);
  const compatItems = verifySource(source);

  const typeDiags = typeErrors.map(
    (d): Diagnostic => ({
      severity: DiagnosticSeverity.Error,
      range: {
        start: { line: Math.max(0, d.line - 1), character: Math.max(0, d.column - 1) },
        end: { line: Math.max(0, d.line - 1), character: Math.max(0, d.column) },
      },
      message: d.message,
      source: "spanda",
    }),
  );

  const compatDiags = compatItems.map((d): Diagnostic => {
    const severity =
      d.severity === "warning" ? DiagnosticSeverity.Warning : DiagnosticSeverity.Error;
    const prefix = d.category ? `[${d.category}] ` : "";
    return {
      severity,
      range: {
        start: { line: Math.max(0, d.line - 1), character: Math.max(0, d.column - 1) },
        end: { line: Math.max(0, d.line - 1), character: Math.max(0, d.column + 20) },
      },
      message: `${prefix}${d.message}`,
      source: "spanda-compat",
    };
  });

  return [...typeDiags, ...compatDiags];
}

documents.onDidChangeContent((change) => {
  connection.sendDiagnostics({ uri: change.document.uri, diagnostics: validate(change.document) });
});

documents.onDidClose((event) => {
  connection.sendDiagnostics({ uri: event.document.uri, diagnostics: [] });
});

documents.listen(connection);
connection.listen();
