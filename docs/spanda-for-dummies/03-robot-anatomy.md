# Chapter 3 — Anatomy of a robot program

Back to [index](./README.md)

---

## The robot block

Everything autonomous lives inside `robot Name { ... }`:

```spanda
robot Rover {
  // sensors — how it sees
  // actuators — how it moves
  // safety — what it must never violate
  // behaviors / tasks — what it does
}
```

Order inside the block is flexible, but this mental model helps.

---

## Sensors — how it sees

```spanda
sensor lidar: Lidar on "/scan";
```

| Piece | Meaning |
|-------|---------|
| `lidar` | Your name for this sensor |
| `Lidar` | Type (defines `.read()`, distances, etc.) |
| `on "/scan"` | Where data comes from (ROS-style topic) |

Read it in code:

```spanda
let scan = lidar.read();
if scan.nearest_distance < 1.0 m {
  wheels.stop();
}
```

**Tip:** Distances use units — `1.0 m`, not bare `1.0`. The compiler catches unit mistakes.

---

## Actuators — how it moves

```spanda
actuator wheels: DifferentialDrive;
```

Common commands:

```spanda
wheels.drive(linear: 0.3 m/s, angular: 0.0 rad/s);
wheels.stop();
wheels.execute(safe_action);   // only SafeAction, not raw AI output
```

---

## Safety — the reflexes

```spanda
safety {
  max_speed = 1.0 m/s;
  stop_if lidar.nearest_distance < 0.5 m;
}
```

These run **before** motion reaches hardware. Not comments. Not optional at runtime.

You can also define **zones** (keep-out regions) — see `examples/patrol_with_zones.sd`.

---

## Behaviors — what it does

One-shot or looping logic:

```spanda
behavior patrol() {
  loop every 100ms {
    // read sensors, decide, actuate
  }
}
```

`loop every 100ms` = “run this block ten times per second.” Good for control loops.

---

## Tasks — background chores

```spanda
task watchdog every 50ms {
  let scan = lidar.read();
  let _ = scan;
}
```

Tasks are scheduled separately from behaviors — useful for monitoring, logging, or parallel work. Priorities: `critical`, `high`, `low`.

---

## Events and triggers — “when X, do Y”

```spanda
event ObstacleSeen;

on ObstacleSeen {
  wheels.stop();
}
```

Emit from anywhere: `emit ObstacleSeen;`

More trigger types (timers, conditions, topics): see [triggers.md](../triggers.md) or `examples/triggers_demo.sd`.

---

## Hardware and deploy — “will it run on my board?”

```spanda
hardware RoverV1 {
  memory: 4 GB;
  sensors [ Lidar ];
  actuators [ DifferentialDrive ];
}

deploy Rover to RoverV1;
```

Then:

```bash
spanda verify rover.sd --target RoverV1
```

Verification answers: “Does this program match this hardware?” before you flash an SD card.

---

## Copy-paste starter template

```spanda
robot MyRover {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  safety {
    max_speed = 1.0 m/s;
    stop_if lidar.nearest_distance < 0.5 m;
  }

  behavior patrol() {
    loop every 100ms {
      let scan = lidar.read();
      if scan.nearest_distance < 1.0 m {
        wheels.stop();
      } else {
        wheels.drive(linear: 0.3 m/s, angular: 0.0 rad/s);
      }
    }
  }
}
```

Save as `my_rover.sd`, then `spanda check my_rover.sd` and `spanda run my_rover.sd`.

---

**Next:** [AI without the scary parts](./04-ai-made-simple.md)
