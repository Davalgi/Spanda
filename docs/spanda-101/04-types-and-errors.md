# Lesson 4 — Types, units, and errors

**Goal:** Model success and failure with `Result` and `Option` instead of exceptions.

**Example:** [`examples/basics/04_result_and_option.sd`](../../examples/basics/04_result_and_option.sd)

---

## Result for fallible operations

Navigation, planning, and hardware calls can fail. Spanda uses **`Result<T, E>`**:

```spanda
enum NavError {
  Blocked,
  Timeout
}

export fn plan_route() -> Result<Path, NavError> {
  return Err(Blocked);
}
```

Handle outcomes with `match`:

```spanda
match plan_route() {
  Ok => wheels.drive(linear: 0.2 m/s, angular: 0.0 rad/s);
  Err => wheels.stop();
};
```

No exceptions — the type system forces you to handle both paths.

---

## Option for missing values

When a value may not exist (optional sensor reading, cached map tile):

```spanda
export fn latest_scan() -> Option<Scan> {
  return None();
}

match latest_scan() {
  Some => wheels.stop();
  None => wheels.stop();
};
```

Use `Option<T>` when absence is normal, `Result<T, E>` when failure carries a reason.

---

## Structs and typed fields

Group related data:

```spanda
struct NavGoal {
  x: Distance;
  y: Distance;
  heading: Angle;
}

let goal = NavGoal { x: 1.0 m, y: 0.0 m, heading: 0.0 rad };
```

Built-in aliases like `Distance`, `Angle`, and `Path` keep robotics code readable.

---

## Serialization (preview)

For telemetry and IPC, serialize values to JSON or YAML:

```spanda
let data = serialize(pose, "json");
let restored = deserialize(data, "json");
```

Full walkthrough: `examples/basics/06_serialize_telemetry.sd` and [spanda-language.md](../spanda-language.md#serialization).

---

## Try it

```bash
spanda check examples/basics/04_result_and_option.sd
spanda run examples/basics/04_result_and_option.sd
```

---

## Exercise

In a new module function `check_clearance() -> Result<Path, NavError>`:

1. Return `Err(Blocked)` when a lidar read would be below `0.5 m` (use a stub condition for now)
2. Return `Ok(...)` with a dummy path otherwise
3. In your robot behavior, `match` the result and stop on `Err`

---

**Next:** [Lesson 5 — Modules and traits](./05-modules-and-traits.md)
