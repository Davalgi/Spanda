# Chapter 6 — Oops: common mistakes

Back to [index](./README.md)

Everyone hits these once. Here's the fix.

---

## “ActionProposal cannot be used where SafeAction is expected”

**You did:** `wheels.execute(proposal);`

**Fix:**

```spanda
let action = safety.validate(proposal);
wheels.execute(action);
```

**Example:** `examples/showcase/ai_safety_violation.sd` (broken) vs `rover_navigation.sd` (fixed)

---

## “Expected unit” / unit mismatch

**You did:** `wheels.drive(linear: 0.5, ...)` without `m/s`

**Fix:** Always attach units to physical quantities:

```spanda
wheels.drive(linear: 0.5 m/s, angular: 0.1 rad/s);
```

Distances: `m`. Angles: `rad`. Time: `ms`, `s`.

---

## `spanda: command not found`

**Fix:**

```bash
export PATH="$PWD/target/release:$PATH"
# or
./target/release/spanda check examples/hello_world.sd
```

Build first: `npm run build:rust`

---

## `check` passes but `run` does nothing visible

Simulation often **stops** or **drives slowly** by design. Try:

```bash
spanda sim file.sd
spanda run file.sd --trace-scheduler
```

Or increase loop visibility in `examples/basics/02_sensors_and_safety.sd`.

---

## `verify` fails — sensor not on hardware profile

**You declared** `sensor camera: Camera` **but hardware only lists** `Lidar`.

**Fix:** Add `Camera` to the `hardware` block's `sensors [ ... ]` list, or remove the sensor from
the robot.

```bash
spanda verify file.sd --target RoverV1 --json
```

Read the JSON report — it names the mismatch.

---

## Forgot `deploy` line

`verify` needs to know which robot maps to which hardware:

```spanda
deploy MyRobot to RoverV1;
```

---

## Module / import not found

**You did:** `import foo.bar;` without a `module foo.bar;` file in the project.

**Fix:** Match module names to file paths, or start with single-file examples in `examples/basics/`.

---

## Infinite loop in simulation

**Symptom:** `run` hangs or runs forever.

**Fix:** Behaviors with `loop every` run until the simulator hits `maxLoopIterations`. For tests,
golden fixtures pass `"run": { "maxLoopIterations": 5 }`. This is normal for control loops.

---

## “I copied Python syntax”

| Python habit | Spanda habit |
|--------------|--------------|
| `def foo():` | `behavior foo() { }` or `export fn foo() { }` |
| `True` / `False` | `true` / `false` |
| `None` | `None()` for Option, or language-specific patterns |
| No units | Always `m/s`, `rad`, `ms` |
| `import rospy` | `sensor` / `topic` declarations in the robot block |

---

## Still stuck?

1. Run `spanda check file.sd --json` and read the first diagnostic
2. Find a working example in `examples/basics/` for the same feature
3. Read the matching [Spanda 101](../spanda-101/README.md) lesson
4. See the full [troubleshooting guide](../troubleshooting.md) for fleet, ROS2, packages, CI, and
   ops issues
5. Open an issue on [GitHub](https://github.com/Davalgi/Spanda/issues)

---

**Next:** [Glossary](./07-glossary.md)
