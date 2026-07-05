/**
 * Vite config for the embedded Control Center SPA (served by spanda-api at `/`).
 * @module
 */

import { readFileSync } from "node:fs";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";

const packageRoot = path.dirname(fileURLToPath(import.meta.url));
const packageVersion = JSON.parse(
  readFileSync(path.resolve(packageRoot, "package.json"), "utf8"),
).version as string;

export default defineConfig({
  define: {
    __CONTROL_CENTER_VERSION__: JSON.stringify(packageVersion),
  },
  plugins: [react()],
  resolve: {
    alias: {
      "@spanda/core": path.resolve(packageRoot, "../../src"),
    },
  },
  build: {
    outDir: "dist-control-center",
    emptyOutDir: true,
    rollupOptions: {
      input: path.resolve(packageRoot, "control-center.html"),
    },
  },
});
