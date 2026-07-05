# Self-Correction

Self-correction actions are typed recovery operations that restore degraded capability without
uncontrolled autonomy.

## Supported actions

| Action | Description |
|--------|-------------|
| Recalibrate Sensor | Re-tune sensor readings |
| Reconnect Provider | Restore provider backend |
| Restart Connectivity | Re-establish network link |
| Reload Package | Reload capability package |
| Reinitialize Device | Reset hardware device |
| Switch Redundant Hardware | Failover to backup component |
| Change Route | Replan navigation path |
| Reduce Speed | Lower velocity limit |
| Pause Mission | Halt mission execution |

## Safety constraints

Every self-correction action passes through:

1. Safety validation
2. Hardware verification
3. Capability verification
4. Readiness validation

High-risk corrections (resume mission, restart fleet, open gate) require **operator approval**.

## Example

```bash
spanda heal examples/showcase/self_correction/rover.sd
```

See `examples/showcase/self_correction/rover.sd`.
