# Recovery SDK

SDK methods for the Recovery Orchestrator across Rust, Python, and TypeScript.

**SDK version:** **0.5.6+** adds `getRecoveryPredictive`, `listRecoverableEntities`, and `recommendRecovery`. Rust gRPC (`spanda-sdk` feature `grpc`, proto semver from `GET /v1/version` — currently **1.0.14**) mirrors the same three RPCs.

## Rust (`spanda-sdk`)

```rust
use serde_json::json;

let client = SpandaClient::local();
let body = json!({
    "file": "rover.sd",
    "entity_id": "robot-1",
    "failure": "gps_loss"
});

client.plan_recovery(&body)?;
client.simulate_recovery(&body)?;
client.validate_recovery(&body)?;
client.execute_recovery(&json!({ "file": "rover.sd", "force_execute": true }))?;
client.list_recovery_policies()?;
client.list_recovery_playbooks()?;
client.get_recovery_history()?;
client.get_recovery_metrics()?;
client.get_recovery_graph(Some("robot-1"))?;
client.explain_recovery(&json!({ "entity_id": "robot-1", "failure": "gps_loss" }))?;
client.get_recovery_predictive(None)?;
client.list_recoverable_entities()?;
client.recommend_recovery(&json!({ "failure": "gps_loss" }))?;
```

Legacy: `client.heal("rover.sd")` → `POST /v1/programs/recovery/heal`

## Rust gRPC (`spanda-sdk` feature `grpc`)

```rust
use serde_json::json;
use spanda_sdk::grpc::GrpcClient;

let mut client = GrpcClient::connect_blocking("http://127.0.0.1:50051")?;
let body = json!({ "failure": "gps_loss" });

client.list_recovery_plans().await?;
client.get_recovery_history().await?;
client.plan_recovery(&body).await?;
client.simulate_recovery(&body).await?;
client.validate_recovery(&body).await?;
client.list_recovery_playbooks().await?;
client.get_recovery_metrics().await?;
client.get_recovery_graph(Some("robot-1")).await?;
client.list_recovery_policies().await?;
client.explain_recovery(&body).await?;
client.get_recovery_predictive(&body).await?;
client.list_recoverable_entities().await?;
client.recommend_recovery(&body).await?;
```

Python and TypeScript SDKs use REST only today (gRPC recovery RPCs are Rust `GrpcClient` only).

## Python (`spanda_sdk`)

```python
from spanda_sdk import SpandaClient

client = SpandaClient()
body = {"file": "rover.sd", "entity_id": "robot-1", "failure": "gps_loss"}

client.plan_recovery(body)
client.simulate_recovery(body)
client.validate_recovery(body)
client.execute_recovery({**body, "force_execute": True})
client.list_recovery_policies()
client.list_recovery_playbooks()
client.get_recovery_history()
client.get_recovery_metrics()
client.get_recovery_predictive()
client.list_recoverable_entities()
client.recommend_recovery({"failure": "gps_loss"})
```

## TypeScript (`@davalgi-spanda/sdk`)

```typescript
import { SpandaClient } from "@davalgi-spanda/sdk";

const client = new SpandaClient();
const body = { file: "rover.sd", entity_id: "robot-1", failure: "gps_loss" };

await client.planRecovery(body);
await client.simulateRecovery(body);
await client.validateRecovery(body);
await client.executeRecovery({ ...body, force_execute: true });
await client.listRecoveryPolicies();
await client.listRecoveryPlaybooks();
await client.getRecoveryHistory();
await client.getRecoveryMetrics();
await client.getRecoveryPredictive();
await client.listRecoverableEntities();
await client.recommendRecovery({ failure: "gps_loss" });
```

## See also

- [recovery-api.md](./recovery-api.md)
- [recovery-orchestrator.md](./recovery-orchestrator.md)
