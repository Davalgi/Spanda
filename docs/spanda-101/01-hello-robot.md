# Lesson 1 — Hello, robot

**Goal:** Understand what a Spanda program looks like and run your first one.

**Example:** [`examples/basics/01_minimal_robot.sd`](../../examples/basics/01_minimal_robot.sd)

---

## What is Spanda?

Spanda is a language for **autonomous systems** — robots, drones, agents, and edge devices where
sensors, AI, actuators, and safety rules belong in the source code, not scattered across scripts and
config files.

A Spanda program describes a **robot**: what it can sense, what it can move, and what behaviors it
runs.

---

## Your first program

```spanda
module basics.minimal;

robot TutorialBot {
  actuator wheels: DifferentialDrive;

  behavior greet() {
    wheels.stop();
  }
}
```

| Piece | Meaning |
|-------|---------|
| `module` | Names this file in a multi-file project (optional for single-file demos) |
| `robot` | The autonomous system you are programming |
| `actuator wheels: DifferentialDrive` | A drive system the robot controls |
| `behavior greet()` | A named entry point the runtime can execute |
| `wheels.stop()` | A safe command sent to the actuator |

Spanda is **not** object-oriented in the Java sense. You do not subclass `Robot`. You declare
hardware and behaviors directly.

---

## Try it

```bash
spanda check examples/basics/01_minimal_robot.sd
spanda run examples/basics/01_minimal_robot.sd
```

- **`check`** — type-check without running. Catches unit errors, unsafe AI usage, and missing
  symbols at compile time.
- **`run`** — execute against the simulated backend (no physical robot required).

---

## Create your own project

```bash
spanda init my_bot
cd my_bot
```

Edit `src/main.sd` with a robot block, then:

```bash
spanda check src/main.sd
spanda run src/main.sd
```

See [getting-started.md](../getting-started.md) for the full first-project walkthrough.

---

## Key ideas

1. **`.sd` files** — Spanda source extension (think “Spanda definition”).
2. **Robots, not classes** — `robot { }` is the top-level unit of autonomous logic.
3. **Actuators are typed** — `DifferentialDrive`, `RobotArm`, `Gripper`, etc. The compiler knows
   what commands are valid.
4. **Check before run** — always run `spanda check` in CI and before deploy.

---

## Exercise

Create `my_bot/src/main.sd` with:

- A robot named `Greeter`
- One `DifferentialDrive` actuator
- A behavior `wave()` that calls `wheels.stop()`

Run `spanda check` and `spanda run`.

---

**Next:** [Lesson 2 — Sensors and safety](./02-sensors-and-safety.md)
