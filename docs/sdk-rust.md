# Rust SDK (`spanda-sdk`)

Official Rust client for Spanda Control Center API v1. Part of the [official multi-language SDK family](./sdk.md#why-three-sdks) — use this crate when your application is written in Rust.

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
spanda-sdk = "0.4"
# optional native gRPC client:
# spanda-sdk = { version = "0.4", features = ["grpc"] }
# or path = "../crates/spanda-sdk" from this monorepo
```

## Usage

```rust
use spanda_sdk::SpandaClient;

fn main() -> Result<(), spanda_sdk::SpandaError> {
    let client = SpandaClient::local();
    let report = client.readiness("rover.sd")?;
    println!("score = {:?}", report.score);
    Ok(())
}
```

## `SpandaClient` methods

| Method | API endpoint |
|--------|----------------|
| `readiness(file)` | `POST /v1/programs/readiness` |
| `assure(file)` | `POST /v1/programs/assure` |
| `diagnose(trace_or_file)` | `POST /v1/programs/diagnose` |
| `heal(target)` | `POST /v1/programs/recovery/heal` |
| `verify_hardware(project)` | `POST /v1/programs/verify/hardware` |
| `verify_capabilities(project)` | `POST /v1/programs/verify/capabilities` |
| `list_entities()` | `GET /v1/entities` |
| `get_entity(id)` | `GET /v1/entities/{id}` |
| `list_devices()` | `GET /v1/devices` |
| `provision_device(id, body)` | `POST /v1/devices/{id}/provision` |
| `run_simulation(project, execute)` | `POST /v1/programs/simulation` |
| `replay(trace)` / `replay_with_options(...)` | `POST /v1/programs/replay` |
| `get_health(entity_id)` | `GET /v1/entities/{id}/health` |
| `get_trust(entity_id)` | `GET /v1/entities/{id}/trust` |
| `get_package_trust(name, version)` | `GET /v1/trust/package` |

## Cognitive & Resilience domain clients

REST wrappers for functional domains (see [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)):

```rust
use spanda_sdk::SpandaClient;

let client = SpandaClient::local();
let reflexes = client.reflex().list()?;
let homeostasis = client.homeostasis().summary()?;
let immunity = client.immunity().scan()?;
let attention = client.attention().queue()?;
let fusion = client.fusion().summary()?;
let memory = client.memory().summary()?;
let profile = client.memory().entity_refs("rover-001")?;
// Legacy facade:
let legacy = client.autonomy().list_reflex()?;
```

| Client | Method | REST |
|--------|--------|------|
| `ReflexClient` | `list()`, `traces()` | `/v1/autonomy/reflex*` |
| `HomeostasisClient` | `summary()` | `/v1/autonomy/homeostasis` |
| `ImmunityClient` | `scan()` | `/v1/autonomy/immunity` |
| `AttentionClient` | `queue()` | `/v1/autonomy/attention` |
| `FusionClient` | `summary()` | `/v1/autonomy/fusion` |
| `MemoryClient` | `summary()`, `entity_refs(id)` | `/v1/autonomy/memory`, entity autonomy |
| `AutonomyClient` | facade | same routes (backward compatible) |

Entity field reference: [entity-sdk.md](./entity-sdk.md#cognitive--resilience-domain-clients).

## Authentication

```rust
let client = SpandaClient::builder()
    .base_url("https://control-center.example.com")
    .api_key(std::env::var("SPANDA_API_KEY").ok())
    .build();
```

## Event stream

```rust
use spanda_sdk::EventStream;

let stream = EventStream::local();
println!("Connect to {}", stream.url());
```

## Native gRPC (optional)

Enable the `grpc` feature for a tonic client:

```toml
spanda-sdk = { path = "../crates/spanda-sdk", features = ["grpc"] }
```

```rust
use spanda_sdk::GrpcClient;

let rt = tokio::runtime::Runtime::new()?;
rt.block_on(async {
    let mut client = GrpcClient::connect("http://127.0.0.1:50051").await?;
    let entities = client.list_entities().await?;
    let report = client.readiness("rover.sd").await?;
    Ok::<_, spanda_sdk::SpandaError>((entities, report))
})?;
```

| `GrpcClient` method | gRPC RPC |
|---------------------|----------|
| `readiness(file)` | `EvaluateProgramReadiness` |
| `assure(file)` | `EvaluateProgramAssure` |
| `run_simulation(file, execute)` | `RunProgramSimulation` |
| `replay(file, deterministic, playback)` | `ReplayProgram` |
| `list_entities()` | `ListEntities` |
| `get_entity(id)` | `GetEntity` |
| `entity_graph()` | `GetEntityGraph` |
| `entity_traceability(query)` | `GetEntityTraceability` |
| `query_entities(body)` | `QueryEntities` |
| `entity_relationships(id)` | `GetEntityRelationships` |
| `entity_readiness(id)` | `GetEntityReadiness` |
| `register_entity(body)` | `RegisterEntity` (Bearer via `SPANDA_API_KEY`) |
| `tag_entity(id, body)` | `TagEntity` (Bearer via `SPANDA_API_KEY`) |
| `relate_entities(body)` | `RelateEntities` (Bearer via `SPANDA_API_KEY`) |
| `sync_entities()` | `SyncEntities` (Bearer via `SPANDA_API_KEY`) |
| `list_devices()` | `ListDevices` |
| `list_autonomy_reflexes()` | `ListAutonomyReflexes` |
| `list_autonomy_reflex_traces()` | `ListAutonomyReflexTraces` |
| `get_autonomy_homeostasis()` | `GetAutonomyHomeostasis` |
| `get_autonomy_immunity()` | `GetAutonomyImmunity` |
| `get_autonomy_attention()` | `GetAutonomyAttention` |
| `get_autonomy_fusion()` | `GetAutonomyFusion` |
| `get_autonomy_memory()` | `GetAutonomyMemory` |
| `get_entity_autonomy(id)` | `GetEntityAutonomy` |
| `get_governance()` | `GetGovernance` |
| `get_operational_risk()` | `GetOperationalRisk` |

Proto semver **1.0.14+** for autonomy fusion/memory RPCs. REST domain clients (`client.reflex()`, `client.homeostasis()`, …) remain the default integration path.

REST + `rpc()` remain the default; gRPC requires `--grpc-bind` on Control Center. Set `SPANDA_API_KEY` before `GrpcClient::connect` so mutation RPCs send Bearer metadata. See [Publishing SDKs](sdk-publishing.md) for crates.io release (`crates-sdk-v*` tag, `CRATES_IO_TOKEN`).

## Error handling

```rust
use spanda_sdk::SpandaError;

match client.readiness("rover.sd") {
    Err(SpandaError::Connection(msg)) => eprintln!("server down: {msg}"),
    Err(SpandaError::Permission(msg)) => eprintln!("auth: {msg}"),
    Err(e) => eprintln!("{e}"),
    Ok(report) => println!("{:?}", report.score),
}
```

## Examples

```bash
cargo run --example readiness -p spanda-sdk
cargo run --example device_inventory -p spanda-sdk
```

## Tests

```bash
cargo test -p spanda-sdk
```
