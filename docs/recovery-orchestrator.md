# Recovery Orchestrator

The **Recovery Orchestrator** (`spanda-recovery`) is the platform-wide recovery intelligence for
Spanda. It coordinates planning, simulation, validation, execution, evidence, and learning across
every entity type — without replacing existing recovery APIs.

## Architecture

```
Detect → Diagnose → Plan → Validate → Execute → Verify → Audit
         ↑ existing assurance/fleet layers
         ↓ Recovery Orchestrator coordinates all stages
```

### Layer placement

| Component | Crate | Role |
|-----------|-------|------|
| **Recovery Orchestrator** | `spanda-recovery` | Planning, graph, policies, playbooks, decision engine |
| Recovery assurance | `spanda-assurance` | Legacy planner, validation gates, execution |
| Recovery types | `spanda-runtime` | Shared DTOs (`RecoveryPlan`, `RecoveryEvidence`, …) |
| Entity model | `spanda-config` | Universal recoverable entities |
| Fleet relay | `spanda-fleet` | Mesh recovery execution |
| Control Center | `spanda-api` | REST `/v1/recovery/*` |

### Core service

```rust
use spanda_recovery::RecoveryOrchestrator;

let orchestrator = RecoveryOrchestrator::new();
let report = orchestrator.plan_recovery(&program, &registry, resolved, &request);
```

### Responsibilities

- Recovery planning and strategy selection
- Dependency and impact analysis (recovery graph)
- Policy evaluation (TOML + program declarations)
- Playbook execution
- Predictive recovery from telemetry
- Validation through health, readiness, trust, security gates
- Immutable evidence generation (persisted on Control Center state)
- Rule-based learning (historical statistics, no ML initially)
- Knowledge-base strategy recommendations (`recommend_from_knowledge`)

### Backward compatibility

- `spanda heal`, `spanda recover`, `POST /v1/programs/recovery/heal` unchanged
- Orchestrator wraps `spanda-assurance` — does not replace it
- Existing `RecoveryLevel` (autonomy) coexists with `RecoveryEscalationLevel` (0–8)

## Escalation levels

| Level | Name | Example strategies |
|-------|------|-------------------|
| 0 | Retry | `retry` |
| 1 | Restart component | `restart_component`, `graceful_degradation` |
| 2 | Restart package | `restart_package`, `switch_provider` |
| 3 | Recover device | `reinitialize`, `switch_sensor` |
| 4 | Recover robot | `restart_robot` |
| 5 | Mission reassignment | `transfer_mission`, `delegate_mission`, `takeover_mission` |
| 6 | Fleet redistribution | `restart_fleet`, `switch_fleet` |
| 7 | Human intervention | `human_escalation` |
| 8 | Emergency shutdown | `emergency_shutdown`, `safe_shutdown` |

## CLI

```bash
spanda recovery plan rover.sd --entity robot-1 --failure gps_loss
spanda recovery simulate rover.sd --failure sensor_failure
spanda recovery dry-run rover.sd --entity robot-1
spanda recovery execute rover.sd --force
spanda recovery validate rover.sd
spanda recovery history
spanda recovery metrics rover.sd
spanda recovery graph rover.sd --entity robot-1
spanda recovery playbooks
spanda recovery explain rover.sd --entity robot-1 --failure gps_loss
```

When `--entity` is omitted, `recovery explain` defaults to the first recoverable entity from the
program registry overlay (for example `Rover` in the self-healing showcase).

## REST API (summary)

| Method | Path | Purpose |
|--------|------|---------|
| `GET/POST` | `/v1/recovery/predictive` | Telemetry-driven degradation indicators |
| `GET` | `/v1/recovery/recoverable-entities` | Entities eligible for orchestrator recovery |
| `POST` | `/v1/recovery/recommend` | Knowledge-base strategy recommendation |

Full reference: [recovery-api.md](./recovery-api.md).

## Persistence

Orchestrator evidence history is stored in `control-center-recovery.json` (under
`SPANDA_CONTROL_CENTER_STATE_DIR`) and hydrated on Control Center startup. `GET
/v1/recovery/history` returns the persisted store.

## Integration points

- **Entity Model** — all entities are recoverable via generic APIs
- **Health / Readiness / Trust** — validation gates
- **Diagnosis** — failure classification feeds decision engine
- **Mission Continuity** — delegation, takeover, succession strategies
- **Fleet** — fleet redistribution playbooks
- **Plugins** — `[recovery.extensions]` in `spanda.plugin.toml` (playbook, strategy, validator);
  `on_recovery_completed` hook after execute; example: `examples/plugins/recovery-plugin/`
- **gRPC** — pin proto semver via `GET /v1/version` (currently **1.0.15**); mirrors REST (14
  recovery RPCs including `GetRecoveryPredictive`, `ListRecoverableEntities`, `RecommendRecovery`)
- **Control Center** — **Recovery** tab: plans, metrics, playbooks, history, graph (nodes/edges),
  plan/simulate/execute actions

## CI & promotion

```bash
./scripts/recovery_orchestrator_smoke.sh
./scripts/recovery_orchestrator_stable_promotion_gate.sh
```

See [stable-hardening-recovery-orchestrator.md](./stable-hardening-recovery-orchestrator.md).

## See also

- [recovery-policies.md](./recovery-policies.md)
- [recovery-playbooks.md](./recovery-playbooks.md)
- [recovery-graph.md](./recovery-graph.md)
- [recovery-simulation.md](./recovery-simulation.md)
- [predictive-recovery.md](./predictive-recovery.md)
- [recovery-api.md](./recovery-api.md)
- [recovery-sdk.md](./recovery-sdk.md)
- [self-healing.md](./self-healing.md)
