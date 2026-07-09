#!/usr/bin/env bash
# Bundle @spanda/lsp into editor/vscode for marketplace VSIX packaging.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

npm run build --workspace=@spanda/lsp

DEST="$ROOT/editor/vscode/server"
rm -rf "$DEST"
mkdir -p "$DEST/dist"

node "$ROOT/scripts/bundle-vscode-server.mjs"
cat > "$DEST/package.json" <<'EOF'
{
  "type": "module"
}
EOF

npm run build --prefix editor/vscode

# Bundle extension host code with vscode-languageclient (not shipped via root node_modules).
npx esbuild "$ROOT/editor/vscode/dist/extension.js" \
  --bundle \
  --platform=node \
  --format=cjs \
  --outfile="$ROOT/editor/vscode/dist/extension.bundle.js" \
  --external:vscode \
  --log-level=warning
mv "$ROOT/editor/vscode/dist/extension.bundle.js" "$ROOT/editor/vscode/dist/extension.js"

echo "✓ Bundled LSP into editor/vscode/server/"
