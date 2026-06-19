import type { MotionCommand, RobotBackend, RobotState, RuntimeValue } from "../runtime/interpreter.js";

/**
 * ROS2 adapter interface — stub for future hardware integration.
 * Maps RoboLang concepts to ROS2 nodes, topics, services, and actions.
 */
export interface Ros2Adapter extends RobotBackend {
  connect(options: Ros2ConnectOptions): Promise<void>;
  disconnect(): Promise<void>;
  publishTopic(topic: string, message: unknown): void;
  callService(service: string, request: unknown): Promise<unknown>;
  sendAction(action: string, goal: unknown): Promise<unknown>;
  isConnected(): boolean;
}

export type Ros2ConnectOptions = {
  nodeName?: string;
  namespace?: string;
  domainId?: number;
};

export class Ros2AdapterStub implements Ros2Adapter {
  private connected = false;
  private state: RobotState = {
    pose: { x: 0, y: 0, theta: 0 },
    velocity: { linear: 0, angular: 0 },
    emergencyStop: false,
  };

  async connect(options: Ros2ConnectOptions): Promise<void> {
    this.connected = true;
    console.log(`[ROS2 stub] Connected as node '${options.nodeName ?? "robolang_node"}'`);
  }

  async disconnect(): Promise<void> {
    this.connected = false;
    console.log("[ROS2 stub] Disconnected");
  }

  readSensor(_sensorName: string, sensorType: string): RuntimeValue {
    if (!this.connected) {
      throw new Error("ROS2 adapter not connected");
    }
    if (sensorType === "Lidar") {
      return { kind: "scan", nearestDistance: Infinity };
    }
    return { kind: "void" };
  }

  executeMotion(cmd: MotionCommand): void {
    if (!this.connected) return;
    console.log(`[ROS2 stub] Motion: ${JSON.stringify(cmd)}`);
  }

  tick(dtMs: number): void {
    const dt = dtMs / 1000;
    this.state.pose.x += this.state.velocity.linear * Math.cos(this.state.pose.theta) * dt;
    this.state.pose.y += this.state.velocity.linear * Math.sin(this.state.pose.theta) * dt;
    this.state.pose.theta += this.state.velocity.angular * dt;
  }

  getState(): RobotState {
    return { ...this.state, pose: { ...this.state.pose }, velocity: { ...this.state.velocity } };
  }

  publishTopic(topic: string, message: unknown): void {
    console.log(`[ROS2 stub] publish ${topic}:`, message);
  }

  async callService(service: string, request: unknown): Promise<unknown> {
    console.log(`[ROS2 stub] service ${service}:`, request);
    return {};
  }

  async sendAction(action: string, goal: unknown): Promise<unknown> {
    console.log(`[ROS2 stub] action ${action}:`, goal);
    return { success: true };
  }

  isConnected(): boolean {
    return this.connected;
  }
}

export function createRos2Adapter(): Ros2Adapter {
  return new Ros2AdapterStub();
}
