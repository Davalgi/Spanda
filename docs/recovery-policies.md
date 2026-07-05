# Recovery Policies

Recovery policies declare conditional self-healing actions in Spanda source and TOML configuration.

## Orchestrator policies (TOML)

Configure per-entity recovery policies in `spanda.toml` for the Recovery Orchestrator:

```toml
[recovery]
known_failures = ["gps_loss", "battery_critical", "sensor_failure"]

[recovery.default]
priority = 50
timeout_secs = 300
retry_limit = 3
max_escalation_level = 4
requires_approval = false
validation_rules = ["health", "readiness", "trust"]

[recovery.policies.robot-1]
entity_id = "robot-1"
entity_kind = "robot"
max_escalation_level = 5
requires_approval = true
safety_constraints = ["unsafe_mode"]

[[recovery.policies.robot-1.escalation_rules]]
from_level = 0
to_level = 1
after_retries = 3
strategy = "restart_component"
```

Escalation levels 0–8 are documented in [recovery-orchestrator.md](./recovery-orchestrator.md).

List policies: `spanda recovery playbooks` / `GET /v1/recovery/policies`

## Program policies (Spanda source)

```spanda
recovery_policy RoverRecovery {
    on gps.failed {
        switch_to visual_odometry;
        reduce_speed 0.5 m/s;
        enter degraded_mode;
    }
    on lidar.failed {
        reduce_speed 0.3 m/s;
        enter safe_mode;
    }
}
```

## Operating modes

Declare modes for verified transitions:

```spanda
operating_mode NormalMode { normal; }
operating_mode DegradedMode { degraded; }
operating_mode SafeMode { safe; }
operating_mode EmergencyMode { emergency; }
operating_mode RecoveryMode { recovery; }
```

## Relationship to mitigation

`mitigation` blocks use `if` conditions; `recovery_policy` blocks use `on` triggers. Both feed the
recovery planner.

## Human approval

Actions that require operator approval:

- Resume mission
- Open gate
- Enable unsafe mode
- Restart fleet

Use `requires approval Operator` in mission declarations for high-risk recovery paths.

## Fleet recovery

```spanda
recovery_policy FleetRecovery {
    on fleet.failed {
        reassign mission;
        promote backup coordinator;
        redistribute tasks;
    }
}
```

Mesh relay: set `SPANDA_FLEET_MESH_URL` on the coordinator runtime; the mesh coordinator exposes
`POST /v1/fleet/recovery`. Recovery handoff actions (`reassign mission`, `promote`, `replace`) also
relay continuity takeover via `POST /v1/fleet/continuity`. Pair with `continuity_policy` for
takeover mode inference — see [continuity-policies.md](./continuity-policies.md). Deployed fleet
agents load programs via `POST /v1/program` and run interpreter-backed recovery (`recovery_engine:
interpreter`) or assurance fallback. See [fleet-distributed.md](./fleet-distributed.md) and
[self-healing.md](./self-healing.md).

## Example

See `examples/showcase/self_healing/rover.sd` and `examples/resilience/degraded_mode_recovery.sd`.
