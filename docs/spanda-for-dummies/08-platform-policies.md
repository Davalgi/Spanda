# Chapter 8 — Platform policies in plain English

Back to [index](./README.md) · Next: [Spanda 101 lesson 11](../spanda-101/11-distributed-decisions.md)

When your robot is **offline**, **fails mid-mission**, or needs to decide **without calling home**,
Spanda lets you write the rules in the program — not in a spreadsheet or a runbook nobody reads.

---

## The idea in one sentence

**Policy blocks** say *what the platform may do automatically* when something goes wrong — recovery,
handoff, offline limits, and who gets to decide locally vs centrally.

---

## The five policy types (cheat sheet)

| Policy | Plain English | Example trigger |
|--------|---------------|-----------------|
| `decision_tree` | “If X happens, do Y” at reflex/local/fleet speed | GPS fails → slow down |
| `offline_policy` | “While disconnected, you may / may not …” | No firmware updates offline |
| `recovery_policy` | “When sensor X fails, switch to backup mode” | Lidar fails → safe mode |
| `continuity_policy` | “When robot dies, who continues the mission?” | Resume from checkpoint on Beta |
| `homeostasis_policy` / `attention_policy` | Platform stability + priority rules | CPU high → throttle low-priority work |

Full options reference: [platform-feature-examples.md](../platform-feature-examples.md)

---

## Minimal example (copy-paste)

```spanda
robot Rover {
  local_decision_authority [emergency_stop, degraded_mode];
  requires_central_approval [update_firmware];
  sensor gps: GPS;
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior patrol() { loop every 50ms { } }
}

decision_tree GPSLoss local {
  when gps.status == Failed {
    enter degraded_mode;
    reduce_speed 0.4 m/s;
  }
}

recovery_policy RoverRecovery {
  on gps.failed {
    enter degraded_mode;
  }
}

continuity_policy Handoff {
  on robot.failed {
    resume from checkpoint;
    reassign mission;
  }
}
```

```bash
spanda check rover.sd
spanda decision list rover.sd
spanda heal rover.sd
```

---

## Stitched workflows (try these)

Multi-step demos with full command lists in the file header:

| Workflow | File |
|----------|------|
| GPS loss (everything) | [`examples/workflows/gps_loss_full_stack.sd`](../../examples/workflows/gps_loss_full_stack.sd) |
| Offline + signed policy | [`examples/workflows/offline_signed_autonomy.sd`](../../examples/workflows/offline_signed_autonomy.sd) |
| Fleet handoff | [`examples/workflows/fleet_patrol_handoff.sd`](../../examples/workflows/fleet_patrol_handoff.sd) |
| Decision audit trail | [`examples/workflows/differentiation_audit_trail.sd`](../../examples/workflows/differentiation_audit_trail.sd) |

Index: [`examples/workflows/README.md`](../../examples/workflows/README.md)

---

## Commands you'll use

| Command | What it does |
|---------|--------------|
| `spanda decision list file.sd` | Show trees, authorities, offline policy |
| `spanda heal file.sd` | Diagnose + suggest recovery |
| `spanda recover file.sd --failure gps` | Run recovery for a failure |
| `spanda continuity file.sd --failed X --progress N` | Mission handoff report |
| `spanda homeostasis check --json` | Platform stability snapshot |

More: [Chapter 5 cheat sheet](./05-commands-cheat-sheet.md) · [getting-started.md](../getting-started.md)

---

## Where to go next

| You want… | Read this |
|-----------|-----------|
| Structured lessons | [Spanda 101 lessons 11–12](../spanda-101/11-distributed-decisions.md) |
| Every field and CLI flag | [platform-feature-examples.md](../platform-feature-examples.md) |
| Flagship demo | `spanda demo distributed-decisions` |
