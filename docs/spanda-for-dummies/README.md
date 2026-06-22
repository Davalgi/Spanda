# Spanda for Dummies

*The no-jargon guide to writing your first autonomous robot program.*

You do **not** need to be a Rust expert, a ROS guru, or an ML researcher. If you can read a short recipe and copy a command into a terminal, you can use Spanda.

This guide is the friendly on-ramp. When you want structured lessons with exercises, switch to [Spanda 101](../spanda-101/README.md).

**All tutorials:** [Tutorials index](../tutorials/README.md)

---

## Who this is for

- Robotics curious — “I have a lidar and no idea where to start”
- Python/C++ developers tired of gluing five repos together
- Safety reviewers who want to see *rules in the source*, not a slide deck
- Anyone who opened `rover.sd` and thought “okay but what does any of this mean?”

---

## Table of contents

| Part | Chapter | What you'll learn |
|------|---------|-------------------|
| I | [What is Spanda, anyway?](./01-what-is-spanda.md) | The big picture in plain English |
| I | [Your first five minutes](./02-five-minutes.md) | Install, check, run — done |
| II | [Anatomy of a robot program](./03-robot-anatomy.md) | Sensors, actuators, safety, behaviors |
| II | [AI without the scary parts](./04-ai-made-simple.md) | Why the compiler says “no” to raw LLM output |
| II | [The ten commands you'll actually use](./05-commands-cheat-sheet.md) | CLI cheat sheet |
| III | [Oops — common mistakes](./06-common-mistakes.md) | Fixes for the errors everyone hits once |
| III | [Glossary](./07-glossary.md) | Jargon → English |

**Total read time:** ~45 minutes. Skim the cheat sheet and glossary anytime.

---

## The one-minute version

Spanda lets you describe a **robot** in one file:

```spanda
robot Rover {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  safety {
    max_speed = 1.0 m/s;
    stop_if lidar.nearest_distance < 0.5 m;
  }

  behavior patrol() {
    loop every 100ms {
      wheels.drive(linear: 0.3 m/s, angular: 0.0 rad/s);
    }
  }
}
```

Then:

```bash
spanda check rover.sd    # does it make sense?
spanda run rover.sd      # run it (simulated — no hardware required)
```

That's the whole idea: **describe the robot, check it, run it.**

---

## Spanda vs the rest of your stack

| Tool | What it's great at | What Spanda adds |
|------|-------------------|------------------|
| Python | ML, notebooks, quick scripts | Typed robot + safety in one language |
| C++ | Drivers, hard real-time | Orchestration without rewriting drivers |
| ROS2 | Messaging, ecosystem | Robot-native syntax + verify-before-deploy |
| Spanda | End-to-end autonomous logic | AI gate, units, hardware fit, simulation |

Spanda is **not** trying to replace your favorite language. It orchestrates perception → AI → safety → motion in one place.

---

## Where to go next

| You want… | Read this |
|-----------|-----------|
| **All tutorials (index)** | [Tutorials index](../tutorials/README.md) |
| Step-by-step lessons | [Spanda 101](../spanda-101/README.md) |
| Fast install | [installation.md](../installation.md) |
| First project in 10 min | [getting-started.md](../getting-started.md) |
| Flagship demo | [killer-demo.md](../killer-demo.md) |
| Full language spec | [spanda-language.md](../spanda-language.md) |
| Examples library | [examples/README.md](../../examples/README.md) |

---

*Spanda — the pulse of autonomous intelligence.*
