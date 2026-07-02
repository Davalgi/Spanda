# Recovery SDK

SDK methods for the Recovery Orchestrator across Rust, Python, and TypeScript.

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
```

Legacy: `client.heal("rover.sd")` → `POST /v1/programs/recovery/heal`

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
```

## See also

- [recovery-api.md](./recovery-api.md)
- [recovery-orchestrator.md](./recovery-orchestrator.md)
