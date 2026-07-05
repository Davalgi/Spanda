# Lesson 6 — AI and the safety gate

**Goal:** Wire AI models into robots and understand why `ActionProposal` cannot drive actuators
directly.

**Examples:**

- [`examples/showcase/rover_navigation.sd`](../../examples/showcase/rover_navigation.sd)
- [`examples/showcase/ai_safety_violation.sd`](../../examples/showcase/ai_safety_violation.sd)
  (intentional compile error)

---

## The core safety rule

AI output is **untrusted**. Spanda enforces this at compile time:

```spanda
// INVALID — compile error
wheels.execute(proposal);

// VALID — only SafeAction reaches hardware
let action = safety.validate(proposal);
wheels.execute(action);
```

`planner.reason(...)` returns an **`ActionProposal`**. Only **`safety.validate()`** produces a
**`SafeAction`** that actuators accept.

---

## Declaring an AI model

```spanda
ai_model planner: LLM {
  provider: "mock";
  model: "safe-planner";
  temperature: 0.1;
}
```

In alpha, providers are simulated (`"mock"`). The syntax is stable for when live providers ship.

---

## Agents that use AI

```spanda
agent Navigator {
  uses planner;
  tools [lidar, camera, wheels];
  goal "Navigate safely";

  plan {
    let scan = lidar.read();
    let scene = camera.analyze();
    let proposal = planner.reason(
      prompt: "Plan safe forward motion",
      input: scene
    );
    let action = safety.validate(proposal);
    wheels.execute(action);
  }
}

behavior run() {
  loop every 100ms {
    Navigator.plan();
  }
}
```

The agent encapsulates perception → reasoning → validation → actuation.

---

## See the compile error

```bash
spanda check examples/showcase/ai_safety_violation.sd
```

Expect a type error mentioning `ActionProposal`. This is the language working as designed.

---

## Safe version

```bash
spanda check examples/showcase/rover_navigation.sd
spanda run examples/showcase/rover_navigation.sd
```

For the full flagship walkthrough (verify + sim + fault injection), see
[killer-demo.md](../killer-demo.md).

---

## Exercise

1. Copy `rover_navigation.sd` structure into your project
2. Add `stop_if lidar.nearest_distance < 0.4 m` in `safety { }`
3. Confirm `spanda check` passes and `spanda run` executes without type errors

---

**Next:** [Lesson 7 — Tasks, triggers, and concurrency](./07-tasks-triggers-concurrency.md)
