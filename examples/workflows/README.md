# Stitched workflows

Multi-feature `.sd` programs with **step-by-step CLI sequences** that connect language declarations
to platform commands. Use these after the minimal [`features/`](../features/) snippets and before
full [`showcase/`](../showcase/) demos.

**Feature options reference:** [platform-feature-examples.md](../docs/platform-feature-examples.md)

```bash
spanda check examples/workflows/gps_loss_full_stack.sd
```

---

## Workflows

| Workflow | File | Features stitched | Primary commands |
|----------|------|-------------------|------------------|
| **GPS loss full stack** | [`gps_loss_full_stack.sd`](./gps_loss_full_stack.sd) | Decision trees, offline policy, recovery, continuity, homeostasis, attention, assurance | `decision list`, `heal`, `recover`, `continuity`, `sim --inject-failure`, `assure` |
| **Offline signed autonomy** | [`offline_signed_autonomy.sd`](./offline_signed_autonomy.sd) | Offline policy, decision tree, recovery, signing, attack simulation | `decision sign-policy`, `sign-tree`, `cache show`, `simulate --offline`, `simulate-attack` |
| **Fleet patrol handoff** | [`fleet_patrol_handoff.sd`](./fleet_patrol_handoff.sd) | Multi-robot fleet, continuity, recovery, succession, takeover | `succession`, `takeover`, `delegate`, `continuity`, `fleet run` |

---

## Suggested order

1. **GPS loss full stack** — see how decisions, recovery, and continuity compose on one rover  
2. **Offline signed autonomy** — policy signing, cache, and offline abuse simulation  
3. **Fleet patrol handoff** — multi-robot handoff and fleet coordination  

---

## Per-feature option examples

Minimal syntax lives in [`features/`](../features/). **All options** for each policy block:

| Feature | Minimal | All options |
|---------|---------|-------------|
| `decision_tree` | [`decision_tree.sd`](../features/decision_tree.sd) | [`decision_tree_options.sd`](../features/decision_tree_options.sd) |
| `offline_policy` | (in `decision_tree.sd`) | [`offline_policy_options.sd`](../features/offline_policy_options.sd) |
| `recovery_policy` | [`recovery_policy.sd`](../features/recovery_policy.sd) | [`recovery_policy_options.sd`](../features/recovery_policy_options.sd) |
| `continuity_policy` | [`continuity_policy.sd`](../features/continuity_policy.sd) | [`continuity_policy_options.sd`](../features/continuity_policy_options.sd) |
| Cognitive policies | [`homeostasis_policy.sd`](../features/homeostasis_policy.sd), [`attention_policy.sd`](../features/attention_policy.sd) | [`cognitive_policies_options.sd`](../features/cognitive_policies_options.sd) |

---

## Related paths

| Path | Purpose |
|------|---------|
| [`features/README.md`](../features/README.md) | One file per capability index |
| [`end_to_end/`](../end_to_end/) | Package-style full scenarios |
| [`showcase/`](../showcase/) | Flagship demos and CI golden paths |
| [Spanda 101 lesson 11–12](../docs/spanda-101/11-distributed-decisions.md) | Guided tutorials |

---

## Adding a stitched workflow

1. Create `examples/workflows/<name>.sd` with a header comment listing the **full command sequence**
2. Combine at least three platform features (e.g. decisions + recovery + continuity)
3. Add a row to this README
4. Link from [platform-feature-examples.md](../docs/platform-feature-examples.md)
