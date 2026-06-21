# Lesson 2 — Sensors and safety

**Goal:** Add perception, physical units, and safety rules that run before every motion command.

**Example:** [`examples/basics/02_sensors_and_safety.sd`](../../examples/basics/02_sensors_and_safety.sd)

---

## Adding a sensor

```spanda
sensor lidar: Lidar on "/scan";
```

Sensors are **first-class declarations**, not opaque driver handles. The type (`Lidar`) tells the compiler what methods exist (`read()`, `nearest_distance`, etc.). The `on "/scan"` clause binds a transport topic (ROS2-style in production; simulated in `spanda run`).

Read sensor data inside a behavior:

```spanda
let scan = lidar.read();
if scan.nearest_distance < 1.0 m {
  wheels.stop();
}
```

---

## Physical units

Spanda enforces **unit algebra** at compile time:

| Unit | Example | Meaning |
|------|---------|---------|
| `m/s` | `0.4 m/s` | Linear speed |
| `rad/s` | `0.2 rad/s` | Angular speed |
| `m` | `1.0 m` | Distance |
| `ms` | `100ms` | Time interval |

You cannot accidentally add meters to seconds — the type checker rejects it.

---

## The safety block

Safety rules are evaluated **before every motion command** reaches an actuator:

```spanda
safety {
  max_speed = 1.0 m/s;
  stop_if lidar.nearest_distance < 0.5 m;
}
```

| Rule | Effect |
|------|--------|
| `max_speed` | Clamps drive velocity |
| `stop_if` | Emergency stop when condition is true |

Safety is not a comment or a TODO — it is part of the language and checked by the compiler and runtime.

---

## Periodic behaviors

Use `loop every` for fixed-rate control loops:

```spanda
behavior patrol() {
  loop every 100ms {
    let scan = lidar.read();
    // react to scan...
  }
}
```

This maps to deterministic scheduling in simulation and (eventually) real-time targets on hardware.

---

## Try it

```bash
spanda check examples/basics/02_sensors_and_safety.sd
spanda run examples/basics/02_sensors_and_safety.sd
spanda sim examples/basics/02_sensors_and_safety.sd
```

`sim` adds verbose simulation output — useful when debugging reactive logic.

---

## Exercise

Starting from Lesson 1’s robot, add:

1. A `Lidar` sensor on `"/scan"`
2. `max_speed = 0.5 m/s` and `stop_if lidar.nearest_distance < 0.3 m`
3. A `patrol()` behavior that drives forward at `0.2 m/s` when clear, stops when close

Verify with `spanda check` then `spanda run`.

---

**Next:** [Lesson 3 — Control flow and loops](./03-control-flow.md)
