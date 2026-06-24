# Recovery Assurance

Recovery assurance integrates with the Spanda Assurance Engine to produce verifiable recovery evidence.

## Evidence chain

```
Failure → Diagnosis → Recovery Plan → Safety Validation → Recovery Action → Outcome → Evidence
```

## Metrics

| Metric | Description |
|--------|-------------|
| Recovery Evidence | Per-action audit records |
| Recovery Readiness | Can recovery be attempted? |
| Success Rate | Historical recovery success ratio |
| Traceability | Full recovery workflow chain |

## Audit record

Each recovery attempt records:

- Detected failure
- Diagnosis
- Chosen recovery
- Safety validation result
- Recovery outcome
- Operator approval (if required)
- Verification outcome

## Integration

Recovery assurance is composed into `spanda assure` via `MissionAssuranceSummary.recovery`.

```bash
spanda assure examples/showcase/recovery_assurance/rover.sd
spanda recovery-report examples/showcase/recovery_assurance/rover.sd
```

## Knowledge base

The recovery knowledge base stores failure patterns, recovery patterns, and success rates for **recommendations only**. It does **not** modify code, safety rules, or hardware requirements automatically.
