# Human / Robot Teaming

**Status:** Experimental · **Horizon:** LATER (v0.7) · **Priority:** P2

Support collaborative autonomy with verified approval, escalation, and fallback paths.

## CLI

```bash
spanda team verify examples/showcase/human_robot/approval_escalation.sd
spanda team verify approval_escalation.sd --json
```

## Core types

| Type | Purpose |
|------|---------|
| `HumanApproval` | Required human sign-off before action |
| `HumanOverride` | Operator override of autonomous decision |
| `HumanEscalation` | Escalation chain on critical events |
| `HumanReview` | Post-hoc review request |

## Example

```spanda
mission Patrol {
    requires approval Operator;

    escalation on critical_fault -> Supervisor;
    fallback on approval_timeout -> enter_safe_mode;
}
```

## Verification

| Path | Validated by |
|------|--------------|
| Approval path | `requires approval` + recovery approval hooks |
| Escalation path | Escalation chain completeness |
| Fallback path | Recovery policy on timeout |

Builds on existing `requires approval`, `SPANDA_OPERATOR_APPROVAL`, and Recovery approval integration. Showcase: `examples/showcase/human_robot/approval_escalation.sd` · `spanda demo team`.

See [differentiation-roadmap.md](./differentiation-roadmap.md) · [self-healing.md](./self-healing.md) · [mission-contracts.md](./mission-contracts.md).
