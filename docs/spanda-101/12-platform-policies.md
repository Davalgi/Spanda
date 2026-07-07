# Lesson 12 — Recovery, continuity, and cognitive policies

**Goal:** Use declarative policy blocks for failure recovery, mission handoff, and platform
resilience — without scattering imperative glue code through behaviors.

**Examples:**

- [`examples/features/recovery_policy.sd`](../../examples/features/recovery_policy.sd)
- [`examples/features/continuity_policy.sd`](../../examples/features/continuity_policy.sd)
- [`examples/features/homeostasis_policy.sd`](../../examples/features/homeostasis_policy.sd)
- [`examples/features/attention_policy.sd`](../../examples/features/attention_policy.sd)

Guides: [self-healing.md](../self-healing.md) · [mission-continuity.md](../mission-continuity.md) ·
[cognitive-resilience-architecture.md](../cognitive-resilience-architecture.md)

---

## Recovery policy

Map sensor or subsystem failures to safe operating modes:

```spanda
recovery_policy RoverRecovery {
    on gps.failed {
        switch_to visual_odometry;
        reduce_speed 0.5 m/s;
        enter degraded_mode;
    }
}
```

CLI: `spanda heal`, `spanda recover`, `spanda sim --inject-failure`.

---

## Continuity policy

When a robot fails mid-mission, define how work continues on a successor:

```spanda
continuity_policy PatrolContinuity {
    on robot.failed {
        resume from checkpoint;
        reassign mission;
    }
}
```

CLI: `spanda continuity`, `spanda takeover`, `spanda succession`.

---

## Cognitive & resilience policies

Optional blocks that tie programs to the platform homeostasis and attention subsystems:

```spanda
homeostasis_policy PlatformStability {
    metric cpu_pct;
    metric memory_pct;
    metric battery_pct;
}

attention_policy MissionFocus {
    rule suppress_low_priority;
    rule boost_critical_health;
}
```

CLI: `spanda homeostasis check`, `spanda reflex list`, Control Center **Cognitive & Resilience**
tab.

---

## Try it

```bash
spanda check examples/features/recovery_policy.sd
spanda heal examples/features/recovery_policy.sd
spanda recover examples/features/recovery_policy.sd --failure gps

spanda continuity examples/features/continuity_policy.sd \
  --failed RoverAlpha --progress 60 --trigger robot_failed

spanda homeostasis check --json
spanda demo self-healing
spanda demo continuity
```

---

## Exercise

1. Add a `on lidar.failed` branch to your `recovery_policy` that enters `safe_mode`.
2. Declare two robots and a `continuity_policy` that transfers state to a named successor.
3. Run `./scripts/cognitive_resilience_smoke.sh` and inspect homeostasis output.

---

## After Spanda 101

| Next step | Resource |
|-----------|----------|
| Full showcase index | [examples/showcase/README.md](../../examples/showcase/README.md) |
| Feature lookup | [examples/features/README.md](../../examples/features/README.md) |
| Mission assurance | [mission-assurance.md](../mission-assurance.md) + `spanda demo assurance` |
| Language reference | [spanda-language.md](../spanda-language.md) |
