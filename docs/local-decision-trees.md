# Local Decision Trees

**Status: Stable** — tree evaluation, Ed25519 signing (`spanda decision sign-tree`), and cache merge.

Local decision trees encode **bounded edge autonomy** — conditional workflows that run on-device without central approval, within safety and trust policy limits.

## Syntax

```sd
decision_tree <Name> <scope> {
    version "1.0.0";
    when <condition> {
        <actions>
        if <nested_condition> { <actions> }
        else if <nested_condition> { <actions> }
        else { <actions> }
    }
}
```

**Scope** maps to decision layer:

| Scope | Layer |
|-------|-------|
| `reflex` | Layer 0 — immediate safety |
| `local` | Layer 1 — local entity |
| `fleet`, `group`, `swarm` | Layer 2 — coordination |
| `central`, `cloud`, `control_center` | Layer 3 — governance |

## Requirements

Decision trees must be:

- **Versioned** — `version "1.0.0"` field
- **Signed** — attached via policy cache signature
- **Auditable** — every evaluation emits a `DistributedDecisionRecord`
- **Testable** — `spanda decision inspect` and `spanda decision simulate`
- **Simulatable** — `POST /v1/decisions/simulate`
- **Bounded** — cannot override safety policy, kill switch, or trust blocks

## Evaluation

Trees evaluate top-down: first matching `when` branch wins; nested `if` / `else if` / `else` provide sub-branching.

```bash
spanda decision inspect gps_loss.sd \
  --entity Rover001 \
  --signal "gps.status == Failed=true,visual_odometry.available=true"
```

## Example

See `examples/showcase/distributed_decisions/gps_loss_local_recovery/main.sd`.
