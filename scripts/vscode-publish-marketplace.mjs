#!/usr/bin/env node
/**
 * Publish the VS Code extension without dev-only package.json scripts in the VSIX.
 */
import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const extDir = join(root, "editor/vscode");
const packagePath = join(extDir, "package.json");
const backupPath = join(root, ".vscode-package-publish-backup.json");

const original = readFileSync(packagePath, "utf8");
const manifest = JSON.parse(original);

const bundle = spawnSync("npm", ["run", "bundle"], { cwd: extDir, stdio: "inherit", env: process.env });
if (bundle.status !== 0) {
  process.exit(bundle.status ?? 1);
}

const { scripts, devDependencies, ...publishManifest } = manifest;
writeFileSync(backupPath, original);
writeFileSync(packagePath, `${JSON.stringify(publishManifest, null, 2)}\n`);

const result = spawnSync(
  "npx",
  ["@vscode/vsce", "publish"],
  { cwd: extDir, stdio: "inherit", env: process.env },
);

writeFileSync(packagePath, readFileSync(backupPath, "utf8"));
try {
  unlinkSync(backupPath);
} catch {
  /* ignore */
}
process.exit(result.status ?? 1);
