# Spanda VS Code Extension

Language support for Spanda (`.sd`) with bundled LSP diagnostics and debug adapter wiring.

## Install

Search **Spanda** in the VS Code Extensions view (`spanda-lang.spanda-vscode`), or:

```bash
code --install-extension spanda-lang.spanda-vscode
```

## Prerequisites

Install the native Spanda CLI so `check` and `verify` diagnostics work. See the [Spanda getting started guide](https://github.com/Davalgi/Spanda/blob/main/docs/getting-started.md) for install options.

Or build from source:

```bash
cargo build -p spanda-cli -p spanda-dap --release
```

## Features

| Feature | How |
|---------|-----|
| Syntax highlighting | Automatic for `.sd` files |
| Type diagnostics | LSP → `spanda check` |
| Verify diagnostics | LSP → `spanda verify` (warnings/errors in Problems panel) |
| Readiness / recovery / continuity quick-fixes | LSP caches `spanda check --readiness-json` diagnostics; code actions for missing `recovery_policy`, `continuity_policy`, and operator approval |
| Snippets | `continuitypolicy`, `recoverypolicy`, and other Spanda declarations |
| Deploy target autocomplete | Type `deploy Robot to ` — suggests `RoverV1`, `JetsonOrin`, … |
| Verify with picker | Command Palette → **Spanda: Verify Deploy Target…** |
| Debug | F5 with Spanda debug configuration — steps through `behavior`, `task every`, and `every` triggers via `spanda-dap` |

## Settings

| Setting | Description |
|---------|-------------|
| `spanda.cliPath` | Path to `spanda` binary (default: `spanda` on PATH) |
| `spanda.languageServerPath` | Override bundled LSP server (advanced) |

## Debug workflow

1. Open a `.sd` file with `behavior` or `task every` blocks
2. Set breakpoints in the gutter
3. Run **Debug: Start Debugging** (Spanda configuration)
4. Use Step Over to advance through periodic tasks

More docs: [debugging](https://github.com/Davalgi/Spanda/blob/main/docs/debugging.md) · [killer demo](https://github.com/Davalgi/Spanda/blob/main/docs/killer-demo.md)

## Repository

https://github.com/Davalgi/Spanda
