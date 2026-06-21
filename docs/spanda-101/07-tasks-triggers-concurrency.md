# Lesson 7 — Tasks, triggers, and concurrency

**Goal:** Run periodic work, react to events, and coordinate background tasks.

**Examples:**

- [`examples/integration/triggers_minimal.sd`](../../examples/integration/triggers_minimal.sd)
- [`examples/integration/concurrency_minimal.sd`](../../examples/integration/concurrency_minimal.sd)
- [`examples/concurrency.sd`](../../examples/concurrency.sd) (full demo)
- [`examples/triggers_demo.sd`](../../examples/triggers_demo.sd) (full catalog)

Deep dives: [triggers.md](../triggers.md), [concurrency.md](../concurrency.md)

---

## Tasks — periodic work with priorities

```spanda
task sense every 50ms {
  let scan = lidar.read();
  if scan.nearest_distance < 0.5 m {
    emit ObstacleSeen;
  }
}
```

Tasks run on a scheduler with optional priority (`critical`, `high`, `low`) and resource budgets. Use them for control loops separate from one-shot behaviors.

---

## Event triggers

```spanda
event ObstacleSeen;

on ObstacleSeen {
  wheels.stop();
}
```

Emit events from tasks or behaviors:

```spanda
emit ObstacleSeen;
```

Handlers run when the event fires. Trace them at runtime:

```bash
spanda run examples/integration/triggers_minimal.sd --trace-triggers
```

---

## Concurrency primitives

For background work and message passing:

```spanda
module comm.ping;

export fn tick() -> Int {
  return 1;
}

robot ConcurrentRover {
  behavior run() {
    let ch = channel();
    send(ch, 42);

    select {
      recv(ch) => wheels.stop();
    };

    spawn tick();
  }
}
```

| Builtin | Purpose |
|---------|---------|
| `channel()` | Create a message channel |
| `send` / `recv` | Non-blocking message pass |
| `select` | Wait on first ready channel |
| `spawn fn()` | Queue background function call |
| `parallel { }` | Cooperative concurrent blocks |

Full demo: `examples/concurrency.sd`

---

## Try it

```bash
spanda check examples/integration/triggers_minimal.sd
spanda run examples/integration/triggers_minimal.sd --trace-triggers

spanda check examples/integration/concurrency_minimal.sd
spanda run examples/integration/concurrency_minimal.sd --trace-scheduler

spanda run examples/concurrency.sd --trace-scheduler --trace-tasks
```

---

## Exercise

1. Add an `event LowBattery` to your robot
2. Add `on LowBattery { wheels.stop(); }`
3. In a task, emit `LowBattery` when a stub condition is true (e.g. always once after 5 loop iterations in simulation)

---

**Next:** [Lesson 8 — Hardware profiles and verify](./08-hardware-and-verify.md)
