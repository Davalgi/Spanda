# Flagship demos

[← Overview](./README.md)

Four primary stories for new visitors. Other demos (`fleet`, `health`, `self-healing`, `continuity`, and more) are listed in [demos-and-examples.md](./demos-and-examples.md) and the root [quick start](../../README.md#quick-start).

## 1. Safety-Typed AI

**Flow:** `ActionProposal` → Safety Validation → `SafeAction`

```bash
./target/release/spanda demo safety
```

**Expected:** unsafe program fails at compile time; safe program passes `safety.validate()` gate.

**Example:** [examples/showcase/unsafe_ai/](../../examples/showcase/unsafe_ai/)

## 2. Hardware + Capability Verification

**Flow:** Mission → Capability → Hardware → Provider → Safety Rule

```bash
./target/release/spanda demo verify
```

**Expected:** mission without Lidar fails verification; complete robot passes with JSON report.

**Example:** [examples/showcase/hardware_verification/](../../examples/showcase/hardware_verification/)

## 3. Readiness / Assurance / Diagnosis

**Questions:** Can the robot run? Why should we trust it? What happened and why?

```bash
./target/release/spanda readiness examples/showcase/readiness/rover.sd
./target/release/spanda assure examples/showcase/assurance/rover.sd
./target/release/spanda diagnose examples/showcase/root_cause_analysis/mission.trace
```

**Expected:** readiness score and go/no-go; assurance report with evidence cases; diagnosis report from mission trace.

**Examples:** [examples/showcase/readiness/](../../examples/showcase/readiness/) · [examples/showcase/assurance/](../../examples/showcase/assurance/) · [examples/showcase/root_cause_analysis/](../../examples/showcase/root_cause_analysis/)

## 4. GPS Loss Recovery (Distributed Decisions)

**Flow:** GPS fault → reflex safety → local recovery tree → fleet escalation → replayable audit trail

```bash
./target/release/spanda demo distributed-decisions
```

**Expected:** GPS failure triggers `GPSLossRecovery` (visual odometry + degraded mode); offline policy bounds recovery; v3 decision trace records layer-by-layer outcomes for replay and audit.

**Example:** [examples/showcase/distributed_decisions/gps_loss_recovery/](../../examples/showcase/distributed_decisions/gps_loss_recovery/) · Walkthrough: [distributed-decision-demo.md](../distributed-decision-demo.md)

## Try Spanda in 5 minutes

One path to evaluate Spanda from a fresh clone:

```bash
git clone https://github.com/Davalgi/Spanda.git
cd Spanda
cargo build --release
./target/release/spanda demo rover
./target/release/spanda demo safety
./target/release/spanda demo verify
```

Optional — readiness, assurance, diagnosis, distributed decisions, and runtime fault detection on showcase examples:

```bash
./target/release/spanda readiness examples/showcase/readiness/rover.sd
./target/release/spanda assure examples/showcase/assurance/rover.sd
./target/release/spanda diagnose examples/showcase/root_cause_analysis/mission.trace
./target/release/spanda demo distributed-decisions
./target/release/spanda fault scan examples/showcase/runtime_faults/crash_detection.sd
./target/release/spanda runtime health examples/showcase/runtime_faults/reboot_detection.sd
```

Install on `PATH` instead: `./scripts/install.sh` or `cargo install --path crates/spanda-cli --locked` — then use `spanda` without the `./target/release/` prefix. See [installation.md](../installation.md).

Walkthrough: [killer-demo.md](../killer-demo.md) · [distributed-decision-demo.md](../distributed-decision-demo.md) · Video script: [demo-script.md](../demo-script.md)
