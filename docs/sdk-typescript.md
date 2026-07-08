# TypeScript SDK (`@davalgi-spanda/sdk`)

Official TypeScript/JavaScript client for Control Center UI, web dashboards, VS Code extensions, and
cloud integrations. Part of the [official multi-language SDK family](./sdk.md#why-three-sdks) — use
this package when your application is written in TypeScript or JavaScript.

## Install

From npm (released package):

```bash
npm install @davalgi-spanda/sdk
```

From this monorepo (development):

```bash
cd sdk/typescript && npm install && npm run build
```

In a monorepo workspace:

```json
{ "dependencies": { "@davalgi-spanda/sdk": "file:../../sdk/typescript" } }
```

## Usage

```typescript
import { SpandaClient } from "@davalgi-spanda/sdk";

const client = SpandaClient.local();
const report = await client.readiness("rover.sd");
console.log(report.score);
```

## Authentication

```typescript
const client = new SpandaClient({
  baseUrl: process.env.SPANDA_CONTROL_CENTER_URL,
  apiKey: process.env.SPANDA_API_KEY,
});
```

## gRPC client (Control Center tonic)

Native gRPC over `@grpc/grpc-js` (proto **1.0.15+** bundled under `proto/`):

```typescript
import { GrpcClient } from "@davalgi-spanda/sdk";

const grpc = await GrpcClient.connect({
  address: process.env.SPANDA_GRPC_URL ?? "127.0.0.1:50051",
  apiKey: process.env.SPANDA_API_KEY,
});
const health = await grpc.getMeshHealth();
const graph = await grpc.getMeshGraph();
grpc.close();
```

Entity Mesh RPCs: `getMeshTopology`, `getMeshNodes`, `getMeshRoutes`, `getMeshPartitions`,
`getMeshHealth`, `getMeshGraph`, `getMeshMergeReport`, `discoverMesh`, `findMeshCapability`,
`simulateMeshPartition`.

## Event stream

```typescript
import { EventStream } from "@davalgi-spanda/sdk";

const stream = EventStream.local();
// Connect with ws package or browser WebSocket to stream.wsUrl
console.log(stream.wsUrl);
```

## Control Center integration pattern

```typescript
const client = SpandaClient.local();
const [health, entities] = await Promise.all([
  client.healthCheck(),
  client.listEntities(),
]);
```

## Cognitive & Resilience domain clients

Functional domain wrappers over `/v1/autonomy/*` (guide:
[cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)):

```typescript
import { SpandaClient } from "@davalgi-spanda/sdk";

const client = SpandaClient.local();
const reflexes = await client.reflex().list();
const homeostasis = await client.homeostasis().summary();
const immunity = await client.immunity().scan();
const attention = await client.attention().queue();
const fusion = await client.fusion().summary();
const memory = await client.memory().summary();
const profile = await client.memory().entityRefs("rover-001");
```

Legacy methods (`listAutonomyReflex()`, `getAutonomyHomeostasis()`, …) remain available.

| Client | Methods |
|--------|---------|
| `ReflexClient` | `list()`, `traces()` |
| `HomeostasisClient` | `summary()` |
| `ImmunityClient` | `scan()` |
| `AttentionClient` | `queue()` |
| `FusionClient` | `summary()` |
| `MemoryClient` | `summary()`, `entityRefs(id)` |

## Examples

```bash
npx tsx examples/sdk/typescript/readiness.ts
npx tsx examples/sdk/typescript/control_center.ts
```

## Tests

```bash
cd sdk/typescript && npm test
```

## Publishing

See [Publishing SDKs](sdk-publishing.md) for npm org `@davalgi-spanda`, `NPM_TOKEN`, release tags
(`npm-sdk-v*`), and 90-day token rotation.
