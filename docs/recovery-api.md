# Recovery REST API

Recovery Orchestrator endpoints (Control Center `/v1`).

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/v1/recovery/plans` | List active recovery plans |
| `GET` | `/v1/recovery/history` | Recovery evidence history |
| `POST` | `/v1/recovery/plan` | Plan recovery |
| `POST` | `/v1/recovery/simulate` | Simulate recovery |
| `POST` | `/v1/recovery/execute` | Execute recovery |
| `POST` | `/v1/recovery/validate` | Validate recovery (dry-run) |
| `GET` | `/v1/recovery/playbooks` | List playbooks |
| `GET` | `/v1/recovery/metrics` | Aggregated metrics |
| `GET` | `/v1/recovery/graph` | Recovery graph (`?entity_id=`) |
| `GET` | `/v1/recovery/policies` | Entity recovery policies |
| `POST` | `/v1/recovery/explain` | Decision explanations |

### Legacy (unchanged)

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/v1/programs/recovery/heal` | Assurance heal evaluation |

## Request body (POST)

```json
{
  "file": "examples/showcase/self_healing/rover.sd",
  "entity_id": "robot-1",
  "failure": "gps_loss",
  "playbook": "sensor_failure",
  "force_execute": false
}
```

## Response

```json
{
  "version": "v1",
  "file": "rover.sd",
  "report": {
    "plans": [...],
    "evidence": [...],
    "metrics": {...},
    "predictive_indicators": [...],
    "passed": true
  }
}
```

## See also

- [recovery-sdk.md](./recovery-sdk.md)
- [recovery-orchestrator.md](./recovery-orchestrator.md)
