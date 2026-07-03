# Decision Traceability

**Status: Stable** — v3 trace emission and proof-field validation tested; envelope crypto signing planned.

Every distributed decision is fully traceable for audit, replay, assurance, diagnosis, and explainability.

## Recorded fields

| Field | Description |
|-------|-------------|
| `decision_layer` | Reflex, local entity, fleet, or control center |
| `entity_id` | Originating entity |
| `inputs` | Signal values and context at decision time |
| `policy_version` | Active policy version |
| `local_context` | Offline duration, connectivity state |
| `selected_action` | Chosen action |
| `rejected_alternatives` | Actions considered but not taken |
| `safety_validation` | Safety gate results |
| `trust_validation` | Trust gate results |
| `escalation_path` | Escalation chain if applicable |
| `outcome` | Post-execution result |

## Security envelope

Every local decision includes:

- Entity identity
- Authority scope
- Policy version
- Decision tree hash
- Timestamp and nonce (replay protection)
- Signature
- Safety and trust validation results
- Audit record ID

## Integration

| System | Integration |
|--------|-------------|
| **Audit** | `DistributedDecisionRecord` → mutation and mission audit |
| **Replay** | Trace files via `spanda decision trace` |
| **Assurance** | Policy version and safety checks in assurance reports |
| **Diagnosis** | Decision chain in diagnosis context |
| **Explainability** | `spanda decision explain` plain-language output |

## CLI

```bash
spanda decision trace mission.trace --json
spanda decision explain mission.trace
```

Enable live emission with `SPANDA_DECISION_TRACE=1` during mission execution.

See also [decision audit trail](./decision-audit-trail.md).
