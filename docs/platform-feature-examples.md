# Platform feature examples and options

Runnable reference for **v0.6+ platform policy blocks** in `.sd` source: syntax fields, CLI options,
and stitched multi-feature workflows.

**Quick links:**

| Resource | Path |
|----------|------|
| Minimal snippets | [`examples/features/`](../examples/features/) |
| All options per feature | [`examples/features/*_options.sd`](../examples/features/) |
| Stitched workflows | [`examples/workflows/`](../examples/workflows/) |
| Flagship demos | [`examples/showcase/`](../examples/showcase/) |

---

## How to use this guide

1. Start with the **minimal** file for a feature (e.g. `decision_tree.sd`).
2. Open the **options** file for every field and variant (e.g. `decision_tree_options.sd`).
3. Run a **stitched workflow** to see commands chained end-to-end.
4. Deep-dive in the topic guide linked per section.

```bash
spanda check examples/features/decision_tree.sd
spanda check examples/features/decision_tree_options.sd
spanda check examples/workflows/gps_loss_full_stack.sd
```

---

## `local_decision_authority` / `requires_central_approval`

Declared inside `robot { }`. Split actions between local (reflex/spinal) and central (brain) layers.

| Field | Type | Description |
|-------|------|-------------|
| `local_decision_authority` | `[action, …]` | Actions the entity may take without central approval |
| `requires_central_approval` | `[action, …]` | Actions that must escalate to Control Center / operator |

**Common local actions:** `emergency_stop`, `degraded_mode`, `return_home`, `sensor_failover`,
`reduce_speed`, `pause_mission`, `obstacle_avoidance`

**Common central actions:** `update_firmware`, `override_safety_policy`, `disable_kill_switch`,
`resume_high_risk_mission`

| Example | File |
|---------|------|
| Minimal | [`decision_tree.sd`](../examples/features/decision_tree.sd) |
| Full authority lists | [`decision_tree_options.sd`](../examples/features/decision_tree_options.sd) |

**CLI:**

```bash
spanda decision list <file.sd>
spanda decision inspect <file.sd> --entity <RobotName> --action <action>
```

Guide: [distributed-decisions.md](./distributed-decisions.md) ·
[decision-authority.md](./decision-authority.md)

---

## `decision_tree`

Conditional decision logic at reflex, local, or fleet layer.

| Field / option | Values | Description |
|----------------|--------|-------------|
| Scope (2nd label) | `reflex`, `local`, `fleet` | Decision layer (latency vs coordination) |
| `version` | string | Tree version for signing and cache |
| `signature` | hex string | Pre-signed tree (optional) |
| `when <condition>` | expression | Top-level trigger |
| Nested `if` / `else if` / `else` | inside `when` | Branching recovery paths |
| Actions | statements | e.g. `enter degraded_mode`, `stop_motor`, `request_takeover` |

| Example | File |
|---------|------|
| Minimal (reflex + local) | [`decision_tree.sd`](../examples/features/decision_tree.sd) |
| All scopes + nested branches | [`decision_tree_options.sd`](../examples/features/decision_tree_options.sd) |
| Flagship GPS loss | [`showcase/distributed_decisions/gps_loss_recovery/mission.sd`](../examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd) |

**CLI:**

```bash
spanda decision list <file.sd>
spanda decision inspect <file.sd> --entity <name>
spanda decision sign-tree <file.sd> --tree <TreeName> --write-cache
spanda decision simulate <file.sd> --offline
spanda decision simulate-attack <file.sd> --offline-abuse
```

Guide: [local-decision-trees.md](./local-decision-trees.md)

---

## `offline_policy`

Bounds autonomous operation when disconnected from Control Center.

| Field | Type | Description |
|-------|------|-------------|
| `max_duration` | duration (`30 min`) | Maximum offline autonomy window |
| `policy_version` | string | Version for signing and audit |
| `signature` | hex string | Pre-signed policy (optional) |
| `expires_at` | ms timestamp | Policy expiration (optional) |
| `allowed_actions` | `[action, …]` | Permitted actions while offline |
| `forbidden_actions` | `[action, …]` | Blocked actions while offline |

| Example | File |
|---------|------|
| Inline (with decision tree) | [`decision_tree.sd`](../examples/features/decision_tree.sd) |
| All fields + multiple policies | [`offline_policy_options.sd`](../examples/features/offline_policy_options.sd) |

**CLI:**

```bash
spanda decision policy <file.sd>
spanda decision sign-policy <file.sd> --policy <PolicyName> --write-cache
spanda decision cache show|sync|clear
spanda decision simulate <file.sd> --offline
spanda decision security-audit
```

---

## `recovery_policy`

Maps subsystem failures to safe operating modes. Evaluated during `run`/`sim` and by
`heal`/`recover`.

| Field | Type | Description |
|-------|------|-------------|
| `on <condition>` | dotted name | Trigger: `gps.failed`, `lidar.failed`, `battery.critical`, `connectivity.lost`, … |
| Actions | statements | `switch_to`, `reduce_speed`, `enter *_mode`, `pause_mission`, `restart connectivity`, … |

| Example | File |
|---------|------|
| Minimal | [`recovery_policy.sd`](../examples/features/recovery_policy.sd) |
| Multiple `on` branches | [`recovery_policy_options.sd`](../examples/features/recovery_policy_options.sd) |
| Full showcase | [`showcase/self_healing/rover.sd`](../examples/showcase/self_healing/rover.sd) |

**CLI:**

```bash
spanda heal <file.sd>
spanda recover <file.sd> --failure gps|lidar|…
spanda recovery-report <file.sd>
spanda sim <file.sd> --inject-failure gps
spanda recovery plan <file.sd> --failure gps
spanda recovery explain <file.sd> --failure gps
```

Guide: [self-healing.md](./self-healing.md) · [recovery-orchestrator.md](./recovery-orchestrator.md)

---

## `continuity_policy`

Mission handoff when robots fail, degrade, or coordinators go offline.

| Field | Type | Description |
|-------|------|-------------|
| `on <trigger>` | dotted name | `robot.failed`, `battery_critical`, `gps_health.degraded`, `coordinator.failed`, … |
| Actions | statements | `resume from checkpoint`, `reassign mission`, `promote backup coordinator`, `transfer state to successor`, `notify fleet`, `escalate to control center`, `redistribute swarm tasks` |

| Example | File |
|---------|------|
| Minimal | [`continuity_policy.sd`](../examples/features/continuity_policy.sd) |
| Multiple triggers + fleet | [`continuity_policy_options.sd`](../examples/features/continuity_policy_options.sd) |
| Warehouse showcase | [`showcase/continuity/warehouse.sd`](../examples/showcase/continuity/warehouse.sd) |

**CLI flags:**

| Flag | Description |
|------|-------------|
| `--failed` / `--failed-robot` | Entity that failed |
| `--progress` | Mission progress 0–100 |
| `--trigger` | `robot_failed`, `battery_critical`, `fleet_offline`, … |
| `--successor` / `--to` | Target for takeover/delegation |
| `--scope` | `robot`, `fleet`, `swarm`, … |
| `--json` | Machine-readable report |

```bash
spanda continuity <file.sd> --failed <name> --progress <pct> --trigger robot_failed
spanda takeover <file.sd> --failed <name> --successor <name>
spanda delegate <file.sd> --failed <name> --to <name>
spanda succession <file.sd> --failed <name> --scope fleet
```

Guide: [mission-continuity.md](./mission-continuity.md)

---

## `@policy` homeostasis / attention

Cognitive & Resilience Architecture hooks — platform stability metrics and attention rules.
Prefer `@policy(kind: …)` only; legacy `homeostasis_policy` / `attention_policy` keywords were removed.

### Homeostasis (`kind: "homeostasis"`)

| Field | Values |
|-------|--------|
| `metric` | `cpu_pct`, `memory_pct`, `battery_pct`, … |

### Attention (`kind: "attention"`)

| Field | Values |
|-------|--------|
| `rule` | `suppress_low_priority`, `boost_critical_health`, … |

| Example | File |
|---------|------|
| Minimal (separate) | [`homeostasis_policy.sd`](../examples/features/homeostasis_policy.sd), [`attention_policy.sd`](../examples/features/attention_policy.sd) |
| Multiple policies | [`cognitive_policies_options.sd`](../examples/features/cognitive_policies_options.sd) |

**CLI:**

```bash
spanda homeostasis check --json
spanda reflex list --json
spanda fusion analyze
spanda immunity quarantine
spanda recovery confidence
./scripts/cognitive_resilience_smoke.sh
```

Guide: [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)

---

## Stitched workflows

Multi-feature programs with full command sequences in file headers.

| Workflow | Features | File |
|----------|----------|------|
| **GPS loss full stack** | Decisions + offline + recovery + continuity + cognitive + assurance | [`workflows/gps_loss_full_stack.sd`](../examples/workflows/gps_loss_full_stack.sd) |
| **Offline signed autonomy** | Policy signing, cache, offline simulate, attack scenarios | [`workflows/offline_signed_autonomy.sd`](../examples/workflows/offline_signed_autonomy.sd) |
| **Fleet patrol handoff** | Multi-robot fleet, succession, takeover, delegate | [`workflows/fleet_patrol_handoff.sd`](../examples/workflows/fleet_patrol_handoff.sd) |
| **Differentiation audit trail** | Decision trace, audit, explain, contract verify | [`workflows/differentiation_audit_trail.sd`](../examples/workflows/differentiation_audit_trail.sd) |

### GPS loss full stack (example sequence)

```bash
spanda check examples/workflows/gps_loss_full_stack.sd
spanda decision list examples/workflows/gps_loss_full_stack.sd
spanda heal examples/workflows/gps_loss_full_stack.sd
spanda recover examples/workflows/gps_loss_full_stack.sd --failure gps
spanda continuity examples/workflows/gps_loss_full_stack.sd --failed Rover001 --progress 65 --trigger robot_failed
spanda sim examples/workflows/gps_loss_full_stack.sd --inject-failure gps --record
spanda assure examples/workflows/gps_loss_full_stack.sd
```

Index: [`examples/workflows/README.md`](../examples/workflows/README.md)

---

## Other language features (options examples)

Beyond platform policies, these `*_options.sd` files document fields and CLI for core capabilities:

| Area | Options file | Guide |
|------|--------------|-------|
| Triggers (`on` / `every` / `when` / `while` / state / safety) | [`features/triggers_options.sd`](../examples/features/triggers_options.sd) | [triggers.md](./triggers.md) |
| Kill switch (local + `remote_signed`) | [`features/kill_switch_options.sd`](../examples/features/kill_switch_options.sd) | [kill-switch.md](./kill-switch.md) |
| Verify + deploy + `requires_hardware` | [`features/verify_options.sd`](../examples/features/verify_options.sd) | [hardware-compatibility.md](./hardware-compatibility.md) |
| Mission assurance blocks | [`features/assurance_blocks_options.sd`](../examples/features/assurance_blocks_options.sd) | [mission-assurance.md](./mission-assurance.md) |
| Fleet + swarm + OTA deploy | [`features/fleet_swarm_options.sd`](../examples/features/fleet_swarm_options.sd) | [concurrency.md](./concurrency.md) |

Full capability index: [`examples/features/README.md`](../examples/features/README.md)

---

## Related documentation

| Topic | Guide |
|-------|--------|
| Distributed decisions overview | [distributed-decisions.md](./distributed-decisions.md) |
| Decision demo script | [distributed-decision-demo.md](./distributed-decision-demo.md) |
| Mission assurance blocks | [mission-assurance.md](./mission-assurance.md) |
| Feature status matrix | [feature-status.md](./feature-status.md) |
| Examples hub | [examples/README.md](../examples/README.md) |
| Spanda 101 lessons 11–12 | [11-distributed-decisions.md](./spanda-101/11-distributed-decisions.md) · [12-platform-policies.md](./spanda-101/12-platform-policies.md) |
