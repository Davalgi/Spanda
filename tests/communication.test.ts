import { describe, expect, it } from "vitest";
import { tokenize } from "../src/lexer/index.js";
import { parse } from "../src/parser/index.js";
import { typeCheck } from "../src/types/index.js";
import { checkViaCli, isCliAvailable } from "../src/rust-bridge.js";

function checkSource(source: string): void {
  const program = parse(tokenize(source));
  typeCheck(program);
}

describe("communication framework", () => {
  it("parses and type-checks custom message declarations", () => {
    const source = `message LidarReading {
  scan: Scan;
  timestamp: String;
  version: 1;
}

robot CommBot {
  topic lidar_scan: LidarReading publish on "/scan";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior noop() { }
}`;
    expect(() => checkSource(source)).not.toThrow();
  });

  it("type-checks topic QoS and transport", () => {
    const source = `robot QosBot {
  bus sim;
  topic stream: Scan {
    qos reliable;
    rate 20Hz;
    deadline 50ms;
  } on sim;
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior run() {
    publish stream with lidar.read();
  }
}`;
    expect(() => checkSource(source)).not.toThrow();
  });

  it("type-checks typed service and action declarations", () => {
    const source = `message BatteryStatus { level: String; }

robot ServiceBot {
  service GetBattery {
    request String;
    response BatteryStatus;
  };
  action NavigateTo {
    request Pose;
    feedback String;
    result String;
  };
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior run() {
    let status = call GetBattery();
    let nav = execute NavigateTo(pose(x: 1.0 m, y: 0.0 m, theta: 0.0 rad));
  }
}`;
    expect(() => checkSource(source)).not.toThrow();
  });

  it("type-checks subscribe and publish(expr) forms", () => {
    const source = `robot PubSubBot {
  topic scan_out: Scan publish on "/out";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior run() {
    subscribe scan_out;
    publish scan_out(lidar.read());
  }
}`;
    expect(() => checkSource(source)).not.toThrow();
  });

  it("type-checks agent communication capabilities", () => {
    const source = `robot AgentBot {
  topic data: Scan publish on "/data";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  ai_model m: LLM { provider: "mock"; model: "x"; temperature: 0.1; }
  agent Worker {
    uses m;
    can [ subscribe(data), publish(data) ];
    goal "work";
    plan { }
  }
  safety { max_speed = 1.0 m/s; }
  behavior run() { }
}`;
    expect(() => checkSource(source)).not.toThrow();
  });

  it("type-checks peer robots and devices", () => {
    const source = `robot FleetBot {
  bus local;
  robot RoverA;
  device Lidar: Lidar;
  topic pose: Pose publish on "/pose";
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior run() {
    subscribe RoverA.pose;
    discover robots;
  }
}`;
    expect(() => checkSource(source)).not.toThrow();
  });

  it("type-checks discover with capability filter", () => {
    const source = `robot DiscoverBot {
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior run() {
    discover agents where capability includes Planner;
  }
}`;
    expect(() => checkSource(source)).not.toThrow();
  });

  it("type-checks events with payload fields", () => {
    const source = `message Alert { text: String; }

robot EventBot {
  event ObstacleDetected {
    alert: Alert;
  };
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  on ObstacleDetected { wheels.stop(); }
  behavior run() { emit ObstacleDetected; }
}`;
    expect(() => checkSource(source)).not.toThrow();
  });

  it("rejects unknown message types on topics", () => {
    const source = `robot Bad {
  topic x: UnknownMsg publish on "/x";
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior run() { }
}`;
    expect(() => checkSource(source)).toThrow();
  });

  it.skipIf(!isCliAvailable())("checks basic publish/subscribe example via Rust CLI", () => {
    const source = `message LidarReading {
  scan: Scan;
  timestamp: String;
}

robot SensorBot {
  bus sim;
  topic lidar_scan: LidarReading publish on "/scan";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior stream() {
    publish lidar_scan(lidar.read());
  }
}`;
    const result = checkViaCli(source);
    expect(result.ok).toBe(true);
  });

  it.skipIf(!isCliAvailable())("checks typed service declarations via Rust CLI", () => {
    const source = `message BatteryStatus { level: String; }
robot S {
  service GetBattery {
    request String;
    response BatteryStatus;
  };
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior run() { let x = call GetBattery(); }
}`;
    const result = checkViaCli(source);
    expect(result.ok).toBe(true);
  });

  it.skipIf(!isCliAvailable())("rejects unknown message types on topics via Rust CLI", () => {
    const source = `robot Bad {
  topic x: UnknownMsg publish on "/x";
  actuator wheels: DifferentialDrive;
  safety { max_speed = 1.0 m/s; }
  behavior run() { }
}`;
    const result = checkViaCli(source);
    expect(result.ok).toBe(false);
  });
});
