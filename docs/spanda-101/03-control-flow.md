# Lesson 3 — Control flow and loops

**Goal:** Use enums, `match`, and `if`/`else` to structure robot decision logic.

**Example:** [`examples/basics/03_control_flow.sd`](../../examples/basics/03_control_flow.sd)

---

## Enums for robot modes

```spanda
enum DriveMode {
  Idle,
  Cruise,
  Avoid
}
```

Enums model discrete states — navigation modes, mission phases, fault levels. Use them instead of
string flags when the set of values is fixed.

---

## Pattern matching with `match`

```spanda
match mode {
  Idle => wheels.stop();
  Cruise => wheels.drive(linear: 0.5 m/s, angular: 0.0 rad/s);
  Avoid => wheels.drive(linear: 0.0 m/s, angular: 0.3 rad/s);
};
```

`match` is exhaustive: the compiler warns if you miss a variant. Each arm executes one action (or
block) for that variant.

---

## Combining `match` and `if`

Real programs mix structured modes with sensor-driven branches:

```spanda
loop every 50ms {
  let scan = lidar.read();

  match mode {
    Idle => wheels.stop();
    Cruise => wheels.drive(linear: 0.5 m/s, angular: 0.0 rad/s);
    Avoid => wheels.drive(linear: 0.0 m/s, angular: 0.3 rad/s);
  };

  if scan.nearest_distance > 2.0 m {
    wheels.drive(linear: 0.3 m/s, angular: 0.0 rad/s);
  }
}
```

Order matters: safety rules still apply after your logic runs.

---

## Try it

```bash
spanda check examples/basics/03_control_flow.sd
spanda run examples/basics/03_control_flow.sd
```

---

## When to use what

| Construct | Use when |
|-----------|----------|
| `if` / `else` | Sensor thresholds, one-off conditions |
| `match` | Discrete enum variants, Result/Option (Lesson 4) |
| `loop every Nms` | Fixed-rate control or polling |
| `state_machine` + `enter` | Explicit transitions with named states (Lesson 5+) |

For full state-machine syntax, see [spanda-language.md](../spanda-language.md#state-machines) and
`examples/basics/10_state_machine.sd`.

---

## Exercise

Extend your patrol robot from Lesson 2:

1. Add `enum DriveMode { Stop, Forward, Turn }`
2. In the loop, `match` on mode: `Forward` drives straight, `Turn` spins in place, `Stop` calls
   `wheels.stop()`
3. Switch to `Turn` when `scan.nearest_distance < 0.8 m`

---

**Next:** [Lesson 4 — Types, units, and errors](./04-types-and-errors.md)
