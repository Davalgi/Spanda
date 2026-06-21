# Lesson 8 — Hardware profiles and verify

**Goal:** Declare deployment targets and verify your program fits the hardware before you ship.

**Examples:**

- [`examples/integration/verify_walkthrough.sd`](../../examples/integration/verify_walkthrough.sd)
- [`examples/hardware/rover_deploy.sd`](../../examples/hardware/rover_deploy.sd)
- [`examples/showcase/hardware_compatibility.sd`](../../examples/showcase/hardware_compatibility.sd)

Full reference: [hardware-compatibility.md](../hardware-compatibility.md)

---

## Hardware profiles

A profile describes a physical or simulated target:

```spanda
hardware RoverV1 {
  cpu: CortexA78;
  memory: 4 GB;
  sensors [ Lidar, IMU ];
  actuators [ DifferentialDrive ];
  battery { capacity: 100 Wh; }
}
```

Built-in profiles include `RoverV1`, `JetsonOrin`, `RaspberryPi5`, and `ESP32`.

---

## Deploy targets

Link your robot program to a profile:

```spanda
deploy VerifyRover to RoverV1;
```

The compiler records which robot runs on which hardware. Verification uses this mapping.

---

## Behavioral verify blocks

Runtime checks you want enforced during verification:

```spanda
verify {
  robot.velocity().linear <= 1.5 m/s;
}
```

These express invariants — max speed, sensor availability, memory headroom — as part of the program.

---

## The verify command

```bash
spanda verify examples/integration/verify_walkthrough.sd --target RoverV1 --json
spanda verify examples/integration/verify_walkthrough.sd --all-targets
spanda verify rover.sd --simulate
```

| Flag | Effect |
|------|--------|
| `--target` | Check one hardware profile |
| `--all-targets` | Check every declared deploy |
| `--simulate` | Include fault-injection scenarios |
| `--json` | Machine-readable report for CI |

Verification checks sensors, actuators, memory, timing, battery, and AI model requirements against the profile.

---

## Try it

```bash
spanda check examples/integration/verify_walkthrough.sd
spanda verify examples/integration/verify_walkthrough.sd --target RoverV1
spanda verify examples/showcase/hardware_compatibility.sd --json
```

---

## Exercise

1. Add a `hardware RoverV1 { }` block to your project (copy fields from the walkthrough)
2. Add `deploy MyRobot to RoverV1;`
3. Run `spanda verify src/main.sd --target RoverV1` and fix any reported mismatches

---

**Next:** [Lesson 9 — Packages and tests](./09-packages-and-tests.md)
