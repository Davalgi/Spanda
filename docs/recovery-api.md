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
| `GET` | `/v1/recovery/predictive` | Telemetry-driven degradation indicators |
| `POST` | `/v1/recovery/predictive` | Predictive indicators for program context |
| `GET` | `/v1/recovery/recoverable-entities` | Recoverable entities from orchestrator registry |
| `POST` | `/v1/recovery/recommend` | Knowledge-base strategy recommendation |

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

## Persistence

Recovery evidence history (`GET /v1/recovery/history`) is backed by `control-center-recovery.json` in `SPANDA_CONTROL_CENTER_STATE_DIR`. Records are appended after `POST /v1/recovery/execute` and reloaded on Control Center startup.

## Predictive response

`GET /v1/recovery/predictive` returns:

```json
{
  "version": "v1",
  "indicators": [...],
  "should_trigger_preventative": false
}
```

## See also

- [recovery-sdk.md](./recovery-sdk.md)
- [recovery-orchestrator.md](./recovery-orchestrator.md)

## gRPC (proto semver from `GET /v1/version` — currently **1.0.14**)

Mirrors REST with `JsonResponse` envelopes on `spanda.v1.ControlCenter`:

| gRPC RPC | REST equivalent |
|----------|-----------------|
| `ListRecoveryPlans` | `GET /v1/recovery/plans` |
| `GetRecoveryHistory` | `GET /v1/recovery/history` |
| `PlanRecovery` | `POST /v1/recovery/plan` |
| `SimulateRecovery` | `POST /v1/recovery/simulate` |
| `ExecuteRecovery` | `POST /v1/recovery/execute` |
| `ValidateRecovery` | `POST /v1/recovery/validate` |
| `ListRecoveryPlaybooks` | `GET /v1/recovery/playbooks` |
| `GetRecoveryMetrics` | `GET /v1/recovery/metrics` |
| `GetRecoveryGraph` | `GET /v1/recovery/graph` |
| `ListRecoveryPolicies` | `GET /v1/recovery/policies` |
| `ExplainRecovery` | `POST /v1/recovery/explain` |
| `GetRecoveryPredictive` | `GET/POST /v1/recovery/predictive` |
| `ListRecoverableEntities` | `GET /v1/recovery/recoverable-entities` |
| `RecommendRecovery` | `POST /v1/recovery/recommend` |

`GetRecoveryGraph` accepts `QueryRequest.query` as `entity_id=<id>`. POST RPCs use `JsonBodyRequest.body_json` with the same JSON body as REST.
