# spanda-recovery(1)

## NAME

heal, recover, recovery — Self-healing and recovery planning CLI.

## SYNOPSIS

```
spanda heal <file.sd|mission.trace> [--json|--markdown|--html] [--failure <kind>]
spanda recover <file.sd> [--json] [--failure <kind>]
spanda recovery-report <file.sd> [--json|--markdown|--html]
spanda recovery plan <file.sd> [--json|--markdown|--html]
spanda recovery simulate <file.sd> [--json] [--failure <kind>] [--entity <id>]
spanda recovery dry-run <file.sd> [--json] [--failure <kind>]
spanda recovery execute <file.sd> [--json] [--failure <kind>] [--force]
spanda recovery validate <file.sd> [--json] [--failure <kind>]
spanda recovery history [--json]
spanda recovery metrics [--json]
spanda recovery graph [--json] [--entity <id>]
spanda recovery playbooks [--json]
spanda recovery explain [--json] [--entity <id>] [--failure <kind>] [<file.sd>]
spanda recovery knowledge <file.sd> [--json]
spanda sim <file.sd> --inject-failure <kind>
spanda analyze-failure <file.sd> --with-recovery
spanda demo self-healing
```

## DESCRIPTION

Safety-first recovery workflow: detect → diagnose → plan → validate → execute → verify → audit.
Recovery never bypasses safety validation, hardware/capability verification, kill switch, or
operator approval.

**`heal`** — Run the full recovery evaluation on a program or mission trace. Exits non-zero when
recovery is not ready.

**`recover`** — Plan and validate recovery for a specific failure kind (default: policy triggers or
`gps.failed`).

**`recovery plan`** — Emit orchestrator recovery plans, validation gates, and readiness metrics.

**`recovery simulate`** / **`recovery dry-run`** / **`recovery validate`** — Simulate or validate
recovery without side effects.

**`recovery execute`** — Execute orchestrator recovery (requires operator approval for high-risk
actions).

**`recovery history`** / **`recovery metrics`** / **`recovery graph`** / **`recovery playbooks`** —
Query orchestrator evidence, metrics, dependency graph, and playbook catalog.

**`recovery explain`** — Explain recovery strategy selection for an entity and failure kind.

**`recovery knowledge`** — Show merged static policy knowledge and persisted
`.spanda/recovery_knowledge.json` entries.

**`sim --inject-failure`** — Simulate a failure and run recovery planning in the simulation path.

**`analyze-failure --with-recovery`** — Failure impact analysis plus recovery plans.

**`demo self-healing`** — Showcase heal, recover, knowledge, sim inject-failure, and fleet recovery
paths.

## OPTIONS

`--failure <kind>` — Failure trigger for heal/recover/sim (e.g. `gps`, `gps.failed`, `fleet`).

`--json` / `--markdown` / `--html` — Output format for heal and recovery-report.

Operator approval for high-risk actions (testing):

```bash
export SPANDA_OPERATOR_APPROVAL=1
export SPANDA_GRANT_RECOVERY_APPROVAL="resume mission"
```

Fleet mesh relay at runtime:

```bash
export SPANDA_FLEET_MESH_URL=http://coordinator:9700
export SPANDA_FLEET_MESH_TOKEN=...
```

## EXAMPLES

```bash
spanda heal examples/showcase/self_healing/rover.sd
spanda recover examples/showcase/self_healing/rover.sd --failure gps
spanda recovery plan examples/showcase/self_healing/rover.sd --json
spanda recovery explain examples/showcase/self_healing/rover.sd --failure gps
spanda recovery knowledge examples/showcase/self_healing/rover.sd
spanda check examples/showcase/self_healing/rover.sd --readiness-json --json
spanda demo self-healing
```

## SEE ALSO

[self-healing.md](../self-healing.md), [recovery-policies.md](../recovery-policies.md),
[mission-continuity.md](../mission-continuity.md), [spanda-continuity(1)](./spanda-continuity.md),
[spanda-check(1)](./spanda-check.md), [spanda-sim(1)](./spanda-sim.md), [spanda(1)](./spanda.md)
