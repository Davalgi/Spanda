# Recovery Simulation

The Recovery Orchestrator supports failure simulation without affecting production systems.

## Modes

| Mode | CLI | API | Side effects |
|------|-----|-----|--------------|
| **Plan** | `spanda recovery plan` | `POST /v1/recovery/plan` | None — generates plans only |
| **Simulate** | `spanda recovery simulate` | `POST /v1/recovery/simulate` | None — timelines + mission impact |
| **Dry-run** | `spanda recovery dry-run` | `POST /v1/recovery/validate` | None — full pipeline, no execution |
| **Validate** | `spanda recovery validate` | `POST /v1/recovery/validate` | Runs validation gates |
| **Execute** | `spanda recovery execute --force` | `POST /v1/recovery/execute` | Delegates to assurance execution |

## Examples

```bash
# Plan recovery for GPS loss on a robot
spanda recovery plan rover.sd --entity robot-1 --failure gps_loss --json

# Simulate sensor failure with expected downtime
spanda recovery simulate rover.sd --failure sensor_failure

# Dry-run full recovery pipeline
spanda recovery dry-run rover.sd --entity robot-1 --failure connectivity_loss

# Validate gates without execution
spanda recovery validate rover.sd --failure lidar.failed
```

## Output

Simulation reports include:

- Orchestrated recovery plans with strategies and escalation levels
- Upstream/downstream impact from the recovery graph
- Decision explanations (can/should/safe/authorized)
- Estimated duration and mission disruption score
- Predictive indicators (when telemetry provided)
- Legacy assurance report for backward compatibility

## See also

- [recovery-orchestrator.md](./recovery-orchestrator.md)
- [recovery-playbooks.md](./recovery-playbooks.md)
