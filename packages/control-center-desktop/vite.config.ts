/**
 * Vite config for Control Center desktop shell.
 * @module
 */

import { readFileSync } from "node:fs";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const packageRoot = path.dirname(fileURLToPath(import.meta.url));
const packageVersion = JSON.parse(
  readFileSync(path.resolve(packageRoot, "package.json"), "utf8"),
).version as string;

export default defineConfig({
  define: {
    __CONTROL_CENTER_VERSION__: JSON.stringify(packageVersion),
  },
  plugins: [react()],
  server: { port: 5174, strictPort: true },
  build: {
    outDir: "dist",
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      "@davalgi-spanda/web": path.resolve(packageRoot, "../web/src"),
    },
  },
});
