# Lesson 5 — Modules and traits

**Goal:** Split code across files and define agent interfaces with traits.

**Examples:**

- [`examples/basics/05_traits_and_impl.sd`](../../examples/basics/05_traits_and_impl.sd)
- [`examples/modules/path_planning.sd`](../../examples/modules/path_planning.sd) +
  [`navigation.sd`](../../examples/modules/navigation.sd)

---

## Modules

```spanda
module navigation.path_planning;

export fn plan_path(from: Pose, to: Pose) -> Path {
  return trajectory(from: from, to: to, steps: 8);
}
```

| Keyword | Purpose |
|---------|---------|
| `module name;` | Identifies this compilation unit |
| `export fn` | Visible to importers |
| `private fn` | Internal to the module |
| `import other.module;` | Pull exported symbols into scope |

Cross-file example:

```bash
spanda check examples/modules/navigation.sd
```

---

## Traits and implementations

Traits define **interfaces** agents can implement:

```spanda
trait PathPlanner {
  fn plan(goal: NavGoal) -> Path;
}

agent Navigator {
  tools [wheels];
  goal "Plan safe paths";
  plan { wheels.stop(); }
}

impl PathPlanner for Navigator {
  fn plan(goal: NavGoal) -> Path {
    wheels.stop();
  }
}
```

Call the trait method from a behavior:

```spanda
let route = Navigator.plan(goal);
```

This separates *what* planning means (trait) from *who* does it (agent).

---

## Agents vs behaviors

| Construct | Role |
|-----------|------|
| `behavior` | Direct robot logic, often the main loop |
| `agent` | AI-facing entity with goals, tools, and optional trait impls |
| `ai_model` | LLM, vision, or other model (Lesson 6) |

---

## Try it

```bash
spanda check examples/basics/05_traits_and_impl.sd
spanda run examples/basics/05_traits_and_impl.sd
spanda check examples/modules/path_planning.sd
spanda check examples/modules/navigation.sd
```

---

## Exercise

1. Create `examples/modules/my_planner.sd` with `module my.planner` and an exported `plan_path`
   function
2. Import it from a robot file and call `plan_path` inside a behavior
3. Add a trait `Planner` and `impl Planner for` an agent that wraps the call

---

**Next:** [Lesson 6 — AI and the safety gate](./06-ai-and-the-safety-gate.md)
