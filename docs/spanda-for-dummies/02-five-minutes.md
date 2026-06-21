# Chapter 2 — Your first five minutes

Back to [index](./README.md)

---

## Step 1: Get the `spanda` command

**Fast path** — download a prebuilt binary from [GitHub Releases](https://github.com/Davalgi/Spanda/releases). See [installation.md](../installation.md).

**From source** (if you cloned the repo):

```bash
npm install
npm run build:rust
export PATH="$PWD/target/release:$PATH"
```

Sanity check:

```bash
spanda check examples/hello_world.sd
```

You should see a green check or “no type errors”. If not, fix `PATH` or use `./target/release/spanda`.

---

## Step 2: Read the smallest program

Open `examples/basics/01_minimal_robot.sd`:

```spanda
robot TutorialBot {
  actuator wheels: DifferentialDrive;

  behavior greet() {
    wheels.stop();
  }
}
```

That's a complete Spanda program: a robot, one actuator, one behavior.

---

## Step 3: Check and run

```bash
spanda check examples/basics/01_minimal_robot.sd
spanda run examples/basics/01_minimal_robot.sd
```

| Command | Plain English |
|---------|---------------|
| `check` | “Does this program type-check?” |
| `run` | “Run it in simulation.” |

No robot hardware required.

---

## Step 4: Make it slightly interesting

Open `examples/basics/02_sensors_and_safety.sd` and run:

```bash
spanda run examples/basics/02_sensors_and_safety.sd
```

Now the robot has a **lidar**, **safety rules**, and a **loop** that reacts to distance. This is what most real programs look like — just a bit more code.

---

## Step 5: Start your own file (optional)

```bash
spanda init my_rover
cd my_rover
```

Edit `src/main.sd`, then:

```bash
spanda check src/main.sd
spanda run src/main.sd
```

---

## Five-minute checklist

- [ ] `spanda check` works on an example
- [ ] `spanda run` works on an example
- [ ] You know what `robot`, `sensor`, `actuator`, `safety`, and `behavior` mean (see next chapter)

---

**Next:** [Anatomy of a robot program](./03-robot-anatomy.md) · **Deeper dive:** [Spanda 101 Lesson 1](../spanda-101/01-hello-robot.md)
