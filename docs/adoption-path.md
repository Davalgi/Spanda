# Adoption path: wrap your existing Python + ROS2 stack

Spanda is a **2–5k LOC coordination layer** — not a rewrite of your Python models, C++ drivers, or ROS2 graph. This guide is a one-sprint path for robotics engineers who already ship on Python + ROS2.

## What Spanda adds (and what it does not replace)

| Keep in your stack | Move into Spanda (`.sd`) |
|--------------------|--------------------------|
| PyTorch / OpenCV / training pipelines | Safety gate (`safety.validate`), deploy targets, `spanda verify` |
| ROS2 drivers and existing nodes | Typed `topic` / `service` / `action` declarations + bridge |
| C++ vendor SDKs | `extern cpp fn` at the boundary |
| CI linting for Python | `spanda check` + `spanda verify` alongside existing jobs |

**Positioning:** Spanda orchestrates perception → planning → safety → actuation. Your libraries stay where the ecosystem is strongest.

## One-sprint timeline

### Week 1 — CI without hardware

Add two commands to your existing pipeline:

```bash
spanda check src/main.sd
spanda verify src/main.sd --json --target JetsonOrin
```

- `check` catches type errors and unsafe AI patterns at compile time.
- `verify` fails CI when the program cannot run on the declared hardware profile.

Copy-paste CI templates: [ci-verify.md](./ci-verify.md).

**Smoke test with repo examples:**

```bash
spanda check examples/showcase/killer_demo.sd
spanda verify examples/showcase/hardware_compatibility.sd --json --target RoverV1
```

### Week 2 — One `extern python` call

Wrap an existing Python function without moving model code:

```spanda
extern python fn detect_objects(frame: String) -> String;

robot MyRobot {
  sensor camera: Camera on "/camera";
  actuator wheels: DifferentialDrive;

  behavior run() {
    let frame = camera.read();
    let detections = detect_objects(frame);
    let _ = detections;
    wheels.stop();
  }
}
```

Register the handler in `scripts/spanda_python_bridge.py`:

```python
HANDLERS = {
    "detect_objects": lambda frame: my_vision_pkg.detect(frame),
}
```

Run:

```bash
spanda run my_robot.sd
```

Bridge details: [ffi-and-ecosystem.md](./ffi-and-ecosystem.md). Example: [`examples/ffi_python_extern.sd`](../examples/ffi_python_extern.sd).

**Optional — in-process PyO3 (faster, no subprocess):**

```bash
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build -p spanda-core --features python-native
spanda run my_robot.sd
```

Subprocess mode remains the default when `python-native` is not enabled.

### Week 3 — One ROS2 topic (bridge only)

Spanda does **not** replace your ROS2 graph. Bridge one topic — typically `/cmd_vel` or `/scan`:

```spanda
robot BridgeBot {
  topic cmd_vel: Velocity publish on "/cmd_vel";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  behavior patrol() {
    publish cmd_vel with velocity(linear: 0.2 m/s, angular: 0.0 rad/s);
    let scan = lidar.read();
    let _ = scan;
  }
}
```

Enable live transport (rclpy golden path):

```bash
export SPANDA_ROS2_LIVE=1
# ROS 2 Humble sourced: source /opt/ros/humble/setup.bash
spanda run examples/ros2_bridge.sd
```

Full setup and manual validation: [ros2-golden-path.md](./ros2-golden-path.md).

## Minimal program template

```spanda
requires_hardware {
  memory >= 4 GB;
  sensors [ Lidar ];
}

hardware JetsonOrin {
  memory: 8 GB;
  sensors [ Lidar, Camera ];
  actuators [ DifferentialDrive ];
}

robot Patrol {
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

deploy Patrol to JetsonOrin;
```

```bash
spanda check patrol.sd
spanda verify patrol.sd --json --target JetsonOrin
spanda sim patrol.sd
```

## Three flagship demos (5 minutes)

Evaluators should start here — not the full 70+ example library:

| Pillar | Purpose | Command |
|--------|---------|---------|
| **Safety** | Block `ActionProposal` from reaching actuators | `spanda check examples/showcase/ai_safety_violation.sd` |
| **Verify** | Hardware fit before deploy | `spanda verify examples/showcase/hardware_compatibility.sd --json` |
| **Sim** | Patrol with simulated emergency stop | `spanda sim examples/showcase/killer_demo.sd` |

Unified walkthrough: [killer-demo.md](./killer-demo.md).

## When to add more Spanda

| Signal | Next step |
|--------|-----------|
| Unsafe AI proposals reach actuators in code review | Add `safety.validate()` and agent `plan` blocks |
| Deploy surprises on new hardware | Add `hardware` profiles and `deploy ... to` |
| Integration tests need repeatability | `spanda sim --record` + `spanda replay --deterministic` |
| Multi-robot coordination | `spanda fleet run` — see `examples/end_to_end/fleet_coordination.sd` |

## Related

- [getting-started.md](./getting-started.md) — first robot in 10 minutes
- [ci-verify.md](./ci-verify.md) — GitHub Actions and GitLab CI
- [ros2-golden-path.md](./ros2-golden-path.md) — ROS2 bridge setup
- [product-strategy.md](./product-strategy.md) — v0.5 beta scope and priorities
