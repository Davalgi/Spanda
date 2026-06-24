# Self-Healing Framework

Spanda self-healing follows a **safety-first** recovery workflow:

```
Detect → Diagnose → Plan Recovery → Validate Safety → Execute Recovery → Verify Outcome → Audit Evidence
```

Self-healing **never bypasses**:

- Safety validation
- Hardware verification
- Capability verification
- Kill switch
- Human approval requirements

## Recovery levels

| Level | Name | Behavior |
|-------|------|----------|
| 0 | Detection Only | Report failures only |
| 1 | Recommend Recovery | Suggest actions to operator |
| 2 | Automatic Low-Risk | Execute low-risk corrections |
| 3 | Automatic With Validation | Execute after all validation gates pass |
| 4 | Human Approval Required | High-risk actions need operator approval |

## CLI

```bash
spanda heal rover.sd
spanda heal mission.trace
spanda recover rover.sd --failure gps
spanda recovery-report rover.sd
spanda sim rover.sd --inject-failure gps
spanda analyze-failure rover.sd --with-recovery
```

## Example output

```
Issue:
gps.failed

Diagnosis:
Satellite lock lost

Recovery:
switch_to visual_odometry

Risk:
Low

Safety Validation:
PASS

Outcome:
Success
```

## Example

See `examples/showcase/self_healing/rover.sd`.
