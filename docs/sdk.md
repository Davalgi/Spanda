# Spanda SDK Overview

Official SDKs integrate with Spanda through the **Control Center API** (`spanda-api`) — a stable REST v1, gRPC, and WebSocket gateway. SDKs are **thin clients**: all business logic remains in Rust runtime crates; the CLI and SDKs call the same APIs.

## SDKs

| SDK | Package | Priority | Status |
|-----|---------|----------|--------|
| Rust | [`spanda-sdk`](https://crates.io/crates/spanda-sdk) on crates.io | P0 | Stable (v0.4.2 — publish via `crates-sdk-v0.4.2`) |
| Python | [`spanda-sdk`](https://pypi.org/project/spanda-sdk/) on PyPI | P1 | Stable (v0.4.2 — publish via `sdk-python-v0.4.2`) |
| TypeScript | [`@davalgi-spanda/sdk`](https://www.npmjs.com/package/@davalgi-spanda/sdk) on npm | P2 | Stable (v0.4.2 published) |
| Web panel | [`@davalgi-spanda/web`](https://www.npmjs.com/package/@davalgi-spanda/web) on npm | — | Experimental (publish via `npm-web-v*`) |
| Desktop | `@spanda/control-center-desktop` (Tauri) | — | Stable (v0.6.3 — publish via `desktop-v0.6.3` GitHub Release) |

Install from registries:

```bash
cargo add spanda-sdk
pip install spanda-sdk
npm install @davalgi-spanda/sdk
npm install @davalgi-spanda/web
```

Install failures (PEP 668 on Python, `cargo add` in the Spanda monorepo): [troubleshooting.md — Official SDK install](./troubleshooting.md#official-sdk-install).

Maintainers: [Publishing SDKs](sdk-publishing.md) (`crates-sdk-v*`, `sdk-python-v*`, `npm-sdk-v*`, `desktop-v*` tags) · [Control Center versioning](control-center-versioning.md).

Legacy Python client: `packages/sdk-python` (Control Center helpers; use `sdk/python` for full SDK surface).

## Why three SDKs?

Spanda ships **one Control Center API** and **three official client libraries** — not three different platforms. All business logic stays in Rust (`spanda-api`, runtime crates); each SDK is a **thin HTTP/gRPC/WebSocket client** that calls the same `/v1/*` routes the CLI uses.

You do **not** need all three in one project. Pick the package that matches the language your app is written in:

| Install | Language | Typical use |
|---------|----------|-------------|
| `cargo add spanda-sdk` | **Rust** | On-robot services, embedded tools, high-performance backends, crates already in a Cargo workspace |
| `pip install spanda-sdk` | **Python** | Notebooks, CI scripts, ROS2 / rclpy bridges, ML pipelines, quick ops automation |
| `npm install @davalgi-spanda/sdk` | **TypeScript / JavaScript** | Web dashboards, Node backends, VS Code extensions, Control Center integrations |

Each SDK exposes the same operations (`readiness`, fleet ops, recovery, admin, …) with idiomatic types for that ecosystem:

| SDK | Idioms |
|-----|--------|
| Rust | `Result<T, SpandaError>`, optional `grpc` feature for native tonic client |
| Python | dict/JSON-style responses, `from spanda import SpandaClient`, ROS2-friendly scripts |
| TypeScript | `async/await`, typed interfaces, runs in Node or bundled for the browser |

**Package names differ by registry**, not by capability:

- **crates.io** — flat name: `spanda-sdk`
- **PyPI** — same flat name: `spanda-sdk` (different registry from Rust)
- **npm** — scoped name: `@davalgi-spanda/sdk`

SDK versions are bumped **together** when client APIs change — see [versioning.md](./versioning.md).

All three assume Control Center is running (or reachable):

```bash
spanda control-center serve --bind 127.0.0.1:8080
```

Then connect with the client for your language — same server, same API:

```rust
// Rust
let client = SpandaClient::local();
```

```python
# Python
client = SpandaClient.local()
```

```typescript
// TypeScript
const client = SpandaClient.local();
```

Install issues: [troubleshooting.md — Official SDK install](./troubleshooting.md#official-sdk-install).

## Architecture

```
Application / Robot / Dashboard
        │
        ▼
   SDK (Rust / Python / TypeScript)
        │
        ▼
   spanda-api  ── REST /v1/*  ──► domain crates
        │         gRPC ControlCenter
        │         WS /v1/stream/telemetry
        ▼
   spanda-readiness, spanda-assurance, spanda-config, …
```

## Quick start

Start Control Center (serves API + optional UI):

```bash
spanda control-center serve --config examples/robotics --program examples/robotics/rover.sd
```

### Rust

```rust
use spanda_sdk::SpandaClient;

let client = SpandaClient::local();
let report = client.readiness("rover.sd")?;
println!("{}", report.score.unwrap_or(0));
```

### Python

```python
from spanda import SpandaClient

client = SpandaClient.local()
report = client.readiness("rover.sd")
print(report["report"]["score"])
```

### TypeScript

```typescript
import { SpandaClient } from "@davalgi-spanda/sdk";

const client = SpandaClient.local();
const report = await client.readiness("rover.sd");
console.log(report.score);
```

## Authentication

| Mode | Configuration |
|------|----------------|
| Local | Default `http://127.0.0.1:8080` — no auth for read-only program ops |
| API key | `SPANDA_API_KEY` or client `api_key` / Bearer token |
| Remote | `SPANDA_CONTROL_CENTER_URL` |
| mTLS / API keys (future) | Planned; do not hardcode secrets |

## Event stream

Real-time events (`health_changed`, `readiness_changed`, `mission_started`, `recovery_triggered`, …) are available via:

- **WebSocket:** `WS /v1/stream/telemetry`
- **gRPC:** streaming RPCs on `ControlCenter` service

See language-specific docs for stream helpers.

## Error model

All SDKs expose structured errors:

- `SpandaError` — base type
- `ValidationError`, `ReadinessError`, `VerificationError`
- `SecurityError`, `ConnectionError`, `PermissionError`

## Documentation

- [Rust SDK](sdk-rust.md)
- [Python SDK](sdk-python.md)
- [TypeScript SDK](sdk-typescript.md)
- [Cognitive & Resilience domain clients](cognitive-resilience-architecture.md#sdk-and-api) — `ReflexClient`, `HomeostasisClient`, `FusionClient`, …
- [Recovery Orchestrator SDK](recovery-sdk.md) — `planRecovery`, `getRecoveryPredictive`, `listRecoverableEntities`, … (SDK **0.5.6+**)
- [Publishing SDKs (PyPI / npm / desktop)](sdk-publishing.md)
- [Control Center versioning (UI / CLI / desktop releases)](control-center-versioning.md)
- [Control Center API](control-center-api.md)

## Examples

| Language | Path |
|----------|------|
| Rust | `crates/spanda-sdk/examples/` |
| Python | `examples/sdk/python/` |
| TypeScript | `examples/sdk/typescript/` |

## Known limitations

- **Simulation / replay:** Pass `"execute": true` on `POST /v1/programs/simulation` to run the driver; replay supports `"deterministic": true` and `"playback": true`. Default remains inspect-only metadata.
- **Local file paths:** Program endpoints resolve paths relative to Control Center `--config` project root.
- **Pool vs program readiness:** `POST /v1/readiness/run` remains device-pool impact; use `POST /v1/programs/readiness` for CLI-equivalent program scoring.
