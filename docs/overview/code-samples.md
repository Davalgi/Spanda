# Code samples

[← Overview](./README.md) · More examples: [examples/README.md](../../examples/README.md)

## AI agent with safety validation

```spanda
robot Rover {
  sensor lidar: Lidar on "/scan";
  sensor camera: Camera on "/camera";
  actuator wheels: DifferentialDrive;

  ai_model planner: LLM {
    provider: "mock";
    model: "safe-planner";
    temperature: 0.1;
  }

  safety {
    max_speed = 1.0 m/s;
    stop_if lidar.nearest_distance < 0.5 m;
  }

  agent Navigator {
    uses planner;
    tools [lidar, camera, wheels];
    memory short_term;
    goal "Reach destination while avoiding obstacles";

    plan {
      let scene = camera.analyze();
      let proposal = planner.reason(
        prompt: "Create a safe navigation action",
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
}
```

## Hardware deploy verification

```spanda
requires_hardware {
  memory >= 2 GB;
  sensors [ Camera, Lidar ];
}

hardware RoverV1 {
  cpu: CortexA78;
  memory: 4 GB;
  sensors [ Camera, Lidar, IMU ];
  actuators [ DifferentialDrive ];
  battery { capacity: 100 Wh; }
  timing { min_period: 10 ms; }
}

robot RoverMission {
  sensor camera: Camera on "/camera";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;

  mission { duration: 1 h; }

  task control_loop every 50ms {
    budget {
      cpu <= 25%;
      memory <= 256 MB;
    }
    let scan = lidar.read();
    wheels.drive(linear: 0.2 m/s, angular: 0.0 rad/s);
  }

  verify {
    robot.velocity().linear <= 2.0 m/s;
  }
}

simulate_compatibility {
  fault BatteryDegradation;
}

deploy RoverMission to RoverV1;
```

```bash
spanda verify examples/showcase/hardware_compatibility.sd --json
```

## Distributed decisions

```spanda
robot Rover {
  local_decision_authority [emergency_stop, degraded_mode];
  requires_central_approval [update_firmware];

  sensor gps: GPS;
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior patrol() { loop every 50ms { } }
}

decision_tree GPSLossRecovery local {
  when gps.status == Failed {
    enter degraded_mode;
    reduce_speed 0.4 m/s;
  }
}

offline_policy RoverOffline {
  max_duration = 30 min;
  allowed_actions [pause_mission, return_home];
  forbidden_actions [disable_safety];
}
```

```bash
spanda check examples/features/decision_tree.sd
spanda decision list examples/features/decision_tree.sd
```

## Recovery and continuity policies

```spanda
recovery_policy RoverRecovery {
  on gps.failed {
    enter degraded_mode;
    reduce_speed 0.5 m/s;
  }
}

continuity_policy PatrolContinuity {
  on robot.failed {
    resume from checkpoint;
    reassign mission;
  }
}
```

```bash
spanda heal examples/features/recovery_policy.sd
spanda continuity examples/features/continuity_policy.sd --failed RoverAlpha --progress 60 --trigger robot_failed
```

See also: [killer-demo.md](../killer-demo.md) ·
[hardware-compatibility.md](../hardware-compatibility.md) ·
[distributed-decisions.md](../distributed-decisions.md) ·
[examples/features/README.md](../../examples/features/README.md)
