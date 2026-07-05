# Recovery Planning

The `RecoveryPlanner` generates `RecoveryPlan` values from health issues, anomalies, diagnoses, and
failures.

## Workflow

1. **Classify failure** — sensor, actuator, connectivity, provider, package, mission, health, fleet,
   swarm, or safety
2. **Diagnose root cause** — static declarations or trace-based RCA
3. **Match recovery policies** — `recovery_policy` and `mitigation` declarations
4. **Generate plan** — ordered `PlannedRecoveryAction` list with risk assessment
5. **Validate** — produce `SafeRecoveryAction` after all gates pass

## Failure classification

| Class | Example trigger |
|-------|-----------------|
| SensorFailure | `gps.failed` |
| ActuatorFailure | `motor.timeout` |
| ConnectivityFailure | `lte.failed` |
| ProviderFailure | `provider.failed` |
| PackageFailure | `package.failed` |
| MissionFailure | `mission.failed` |
| HealthDegradation | `health.degraded` |
| FleetFailure | `fleet.failed` |
| SwarmFailure | `swarm.failed` |
| SafetyFailure | `safety.violation` |

## GPS failure example

Given `gps.failed` with diagnosis "Satellite lock lost":

1. Switch to visual odometry
2. Reduce speed to 0.5 m/s
3. Enter degraded mode

## CLI

```bash
spanda recovery plan rover.sd
spanda heal rover.sd
```

## See also

- [recovery-orchestrator.md](./recovery-orchestrator.md) — platform-wide orchestrator (graph,
  playbooks, escalation, persisted evidence)
- [recovery-assurance.md](./recovery-assurance.md)
- [self-healing.md](./self-healing.md)
