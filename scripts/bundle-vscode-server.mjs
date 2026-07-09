#!/usr/bin/env node
/**
 * Bundle the Spanda LSP server for the marketplace VSIX (no monorepo tsx helpers).
 */
import * as esbuild from "esbuild";
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const outFile = join(root, "editor/vscode/server/dist/server.js");
mkdirSync(dirname(outFile), { recursive: true });

await esbuild.build({
  entryPoints: [join(root, "packages/lsp/src/server.ts")],
  bundle: true,
  platform: "node",
  format: "esm",
  minify: true,
  outfile: outFile,
  logLevel: "warning",
  define: {
    __SPANDA_MARKETPLACE__: "true",
  },
  plugins: [
    {
      name: "marketplace-stub",
      setup(build) {
        build.onResolve({ filter: /monorepo-lsp-backend\.js$/ }, () => ({
          path: join(root, "packages/lsp/src/monorepo-lsp-backend.stub.ts"),
        }));
      },
    },
  ],
});
