# Lesson 11 — Distributed decisions

**Goal:** Declare which decisions a robot can make locally versus which require central approval,
and
encode offline and reflex decision trees in source.

**Examples:**

- [`examples/features/decision_tree.sd`](../../examples/features/decision_tree.sd) — minimal
- [`examples/features/decision_tree_options.sd`](../../examples/features/decision_tree_options.sd) —
  all scopes and nested branches
- [`examples/features/offline_policy_options.sd`](../../examples/features/offline_policy_options.sd)
  — signing and action lists
- [`examples/workflows/offline_signed_autonomy.sd`](../../examples/workflows/offline_signed_autonomy.sd)
  — stitched signing workflow

**Options reference:** [platform-feature-examples.md](../platform-feature-examples.md)

Full reference: [distributed-decisions.md](../distributed-decisions.md)

---

## Brain, spinal cord, reflex

Spanda models autonomy in layers:

| Layer | Typical decisions | Syntax |
|-------|-------------------|--------|
| **Reflex** | Emergency stop, cut power | `decision_tree … reflex` |
| **Local (spinal cord)** | Degraded mode, sensor failover | `decision_tree … local` |
| **Central (brain)** | Firmware updates, safety overrides | `requires_central_approval` |

---

## Local authority on the robot

Declare what the robot may decide without waiting for the fleet or operator:

```spanda
robot Rover {
    local_decision_authority [emergency_stop, degraded_mode, return_home];
    requires_central_approval [update_firmware, override_safety_policy];
    // ...
}
```

---

## Decision trees

Encode conditional responses as first-class declarations:

```spanda
decision_tree GPSLossRecovery local {
    when gps.status == Failed {
        enter degraded_mode;
        reduce_speed 0.4 m/s;
    }
}

decision_tree ObstacleReflex reflex {
    when obstacle.detected {
        stop_motor;
    }
}
```

---

## Offline policy

When connectivity is lost, constrain what the robot may still do:

```spanda
offline_policy RoverOffline {
    max_duration = 30 min;
    allowed_actions [pause_mission, return_home];
    forbidden_actions [disable_safety];
}
```

---

## Try it

```bash
spanda check examples/features/decision_tree.sd
spanda decision list examples/features/decision_tree.sd
spanda decision simulate-attack examples/features/decision_tree.sd --offline
spanda demo distributed-decisions
```

---

## Exercise

1. Add `sensor_failover` to `local_decision_authority`.
2. Extend `GPSLossRecovery` with a branch that calls `request_takeover` when visual odometry is
   unavailable.
3. Run `spanda decision sign-tree` on your program and inspect the signed cache.

---

## Next

[Lesson 12 — Recovery, continuity, and cognitive policies](./12-platform-policies.md)
