# Publish the Spanda VS Code extension to the Marketplace

Maintainer guide for publishing **`spanda-lang.spanda-vscode`** to the
[Visual Studio Marketplace](https://marketplace.visualstudio.com).

User-facing install docs live in [editor/vscode/README.md](../editor/vscode/README.md).

## Current status (2026-07-09)

| Step | Status |
|------|--------|
| Azure DevOps org + Marketplace PAT | **Done** — PAT rotated; GitHub Actions secret **`VSCE_PAT`** configured |
| Publisher `spanda-lang` profile | **Done** — saved on [Manage Publishers](https://marketplace.visualstudio.com/manage) |
| VSIX build (local + CI) | **Done** — esbuild bundle, ~12 files / ~200 KB; `verify_vscode_vsix.sh` |
| Marketplace extension upload | **Blocked** — `Your extension has suspicious content` (automated scanner; no details). Awaiting [Microsoft manual review](https://aka.ms/marketplacepublishersupport) |
| CI on release tags | **Ready** — `release.yml` uploads VSIX and attempts publish when `VSCE_PAT` is set; publish step is non-blocking until listing is approved |

Until the extension listing is live, install from a [GitHub release VSIX](https://github.com/Davalgi/Spanda/releases) or `./scripts/verify_vscode_vsix.sh`.

## Overview

| Item | Value |
|------|--------|
| Extension ID | `spanda-lang.spanda-vscode` |
| Publisher ID | `spanda-lang` (must match `editor/vscode/package.json`) |
| Local publish | `npm run publish:marketplace` from `editor/vscode/` (uses `vsce login` or `VSCE_PAT` env) |
| CI publish | `.github/workflows/release.yml` — uses GitHub secret **`VSCE_PAT`** on release tags |

The VSIX is built with **esbuild** (no `server/node_modules` in the package). See
`scripts/bundle-vscode-extension.sh` and `scripts/bundle-vscode-server.mjs`.

## One-time setup

### 1. Azure DevOps organization

Marketplace auth uses **Azure DevOps** (`dev.azure.com`), not the Azure Portal.

1. Open [dev.azure.com/new](https://dev.azure.com/new) and create an organization (any name).
2. If the form fails, try [aex.dev.azure.com/new](https://aex.dev.azure.com/new) or an incognito window.

### 2. Personal Access Token (PAT)

1. Open [Personal access tokens](https://dev.azure.com/_usersSettings/tokens) (or
   `https://dev.azure.com/{your-org}/_usersSettings/tokens`).
2. **+ New Token**
3. Set:

| Field | Value |
|-------|--------|
| Organization | **All accessible organizations** |
| Scopes | **Custom defined** → **Show all scopes** → **Marketplace → Manage** |

4. Copy the token immediately.

### 3. Marketplace publisher

1. Open [Manage Publishers](https://marketplace.visualstudio.com/manage).
2. **Create publisher** (or use existing **`spanda-lang`**).
3. Publisher ID must be **`spanda-lang`**.
4. Save publisher profile with plain text and `https://` links only (no URL shorteners).
5. Logo: PNG, 128×128 or 256×256, under 1 MB.

If you see **“Publisher Metadata has suspicious content”**, try saving with a minimal description
and no logo first, then add assets back. Contact [vsmarketplace@microsoft.com](mailto:vsmarketplace@microsoft.com)
if it persists.

### 4. Log in locally

```bash
cd editor/vscode
npm install
npx @vscode/vsce login spanda-lang
```

Paste the PAT when prompted. Alternatively set `VSCE_PAT` in the environment (CI uses this).

## Pre-publish checklist

1. **Bump version** in `editor/vscode/package.json` (each publish needs a new version).
2. **Build and smoke-test the VSIX:**

```bash
./scripts/verify_vscode_vsix.sh
```

3. Optional local install:

```bash
code --install-extension editor/vscode/spanda-vscode-<version>.vsix
```

## Publish manually

From the repo root:

```bash
cd editor/vscode
npm run publish:marketplace
```

This runs:

1. `scripts/bundle-vscode-extension.sh` — esbuild bundles the LSP server and extension host.
2. `scripts/vscode-publish-marketplace.mjs` — strips dev-only `package.json` fields, then `vsce publish`.
3. Restores `package.json` after publish.

`vscode:prepublish` also runs `bundle` if you use `npm run package` directly.

### Manual VSIX upload

1. `npm run package` in `editor/vscode/`
2. [Manage Publishers](https://marketplace.visualstudio.com/manage) → **spanda-lang** → upload the `.vsix`

## Publish via CI (release tags)

GitHub Actions secret **`VSCE_PAT`** is configured on this repository (Marketplace PAT with
**Manage** scope). On each workspace release tag, the `vscode-extension` job in
`.github/workflows/release.yml`:

1. Builds the VSIX (`npm run package:ci`)
2. Uploads it to the GitHub release (always)
3. Runs `npm run publish:marketplace` using `VSCE_PAT`

The Marketplace publish step uses **`continue-on-error: true`** so release artifacts still ship
while the automated “suspicious content” scanner blocks the public listing. Remove that guard after
Microsoft approves the extension.

To rotate the PAT: revoke the old token in Azure DevOps, create a new one, then
`gh secret set VSCE_PAT` (see [GitHub encrypted secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)).

## Verify after publish

```bash
code --install-extension spanda-lang.spanda-vscode
```

Or search **Spanda** in the VS Code Extensions view.

## Troubleshooting

### “Your extension has suspicious content”

The Marketplace scanner often gives **no details**. Common fixes already in this repo:

- No `node_modules` in the VSIX (esbuild bundle)
- No `curl | sh` in `editor/vscode/README.md`
- Marketplace icon 128×128 PNG (not multi‑MB source art)
- No dev `scripts` in the published `package.json` (publish helper strips them)

If it still fails after a clean VSIX (~12 files, ~200 KB), email
[vsmarketplace@microsoft.com](mailto:vsmarketplace@microsoft.com) or
[marketplace publisher support](https://aka.ms/marketplacepublishersupport) with:

- Publisher: `spanda-lang`
- Extension: `spanda-lang.spanda-vscode`
- Version attempted (e.g. `0.7.4`)
- Note: `VSCE_PAT` and publisher profile are configured; only the extension scanner blocks upload
- GitHub: https://github.com/Davalgi/Spanda

### “Publisher Metadata has suspicious content”

Fix the **publisher profile** separately from the extension. Use minimal plain text, verify email,
avoid em dashes and “official publisher” boilerplate until save succeeds.

### PAT / login errors

- PAT scope must include **Marketplace → Manage**
- Organization must be **All accessible organizations**
- Re-login: `npx @vscode/vsce login spanda-lang`

### Version already exists

Bump `version` in `editor/vscode/package.json` and publish again.

## Related scripts and files

| Path | Role |
|------|------|
| `scripts/bundle-vscode-extension.sh` | Bundle LSP + extension for VSIX |
| `scripts/bundle-vscode-server.mjs` | Marketplace LSP esbuild (stubs monorepo tsx helpers) |
| `scripts/vscode-publish-marketplace.mjs` | Publish without dev metadata in VSIX |
| `scripts/verify_vscode_vsix.sh` | Local VSIX build smoke test |
| `editor/vscode/.vscodeignore` | Exclude dev files from VSIX |
| `packages/lsp/src/monorepo-lsp-backend.stub.ts` | Stub for marketplace server bundle |

## Monorepo development (not Marketplace)

```bash
npm run build --workspace=@spanda/lsp
cd editor/vscode && npm run build
# Press F5 in editor/vscode for Extension Development Host
```

For local LSP without the marketplace bundle, point VS Code at the workspace server — see
[getting-started.md](./getting-started.md#editor-support-v04).
