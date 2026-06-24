# Recovery Policies

Recovery policies declare conditional self-healing actions in Spanda source.

## Syntax

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

`mitigation` blocks use `if` conditions; `recovery_policy` blocks use `on` triggers. Both feed the recovery planner.

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

## Example

See `examples/showcase/self_healing/rover.sd` and `examples/resilience/degraded_mode_recovery.sd`.
