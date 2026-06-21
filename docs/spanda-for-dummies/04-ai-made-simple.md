# Chapter 4 — AI without the scary parts

Back to [index](./README.md)

---

## The one rule

**LLMs and vision models suggest. Safety approves. Actuators execute.**

That's it. Spanda enforces this in the type system so you can't "forget" the safety step.

---

## Two types you need to know

| Type | Who makes it | Can it move the robot? |
|------|--------------|------------------------|
| `ActionProposal` | AI (`planner.reason`, agents) | **No** |
| `SafeAction` | `safety.validate(proposal)` | **Yes** |

```spanda
let proposal = planner.reason(prompt: "Go forward", input: scene);
let action = safety.validate(proposal);   // bouncer checks the proposal
wheels.execute(action);                   // only now does motion happen
```

Skip `safety.validate` and `spanda check` fails. That's a feature.

---

## See the error on purpose

```bash
spanda check examples/showcase/ai_safety_violation.sd
```

You'll get a compile error about `ActionProposal`. Compare with:

```bash
spanda check examples/showcase/rover_navigation.sd
spanda run examples/showcase/rover_navigation.sd
```

Same idea, safe version.

---

## Declaring a model (mock for now)

```spanda
ai_model planner: LLM {
  provider: "mock";
  model: "my-planner";
  temperature: 0.1;
}
```

`"mock"` means no API key, no cloud — good for learning and CI.

---

## Agents — AI with a job description

```spanda
agent Navigator {
  uses planner;
  tools [lidar, camera, wheels];
  goal "Navigate without hitting things";

  plan {
    let scene = camera.analyze();
    let proposal = planner.reason(prompt: "Plan path", input: scene);
    let action = safety.validate(proposal);
    wheels.execute(action);
  }
}
```

An **agent** is an AI worker with goals and tools. A **behavior** calls the agent on a schedule:

```spanda
behavior run() {
  loop every 100ms {
    Navigator.plan();
  }
}
```

---

## “But I trust my model”

Trust the model for *ideas*. Don't trust unvalidated output for *torque on a motor*. Safety rules (`max_speed`, `stop_if`, zones) still apply after validation.

For the full demo with verify + sim: [killer-demo.md](../killer-demo.md).

---

**Next:** [The ten commands you'll actually use](./05-commands-cheat-sheet.md) · **Lesson:** [Spanda 101 — AI and the safety gate](../spanda-101/06-ai-and-the-safety-gate.md)
