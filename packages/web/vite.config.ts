/**
 * vite.config module (vite.config.ts).
 * @module
 */

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";

const packageRoot = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  plugins: [react()],
  server: { port: 5173 },
  resolve: {
    alias: {
      "@spanda/core": path.resolve(packageRoot, "../../src"),
    },
  },
  optimizeDeps: {
    exclude: ["spanda-wasm"],
  },
});
