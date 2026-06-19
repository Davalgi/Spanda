import type { Environment, RobotBackend, RobotState } from "../runtime/interpreter.js";

export type SafetyEvaluation = {
  allowed: boolean;
  reason?: string;
  emergencyStop: boolean;
};

export type SafetyConfig = {
  maxSpeed: number;
  stopIfRules: Array<(env: Environment) => boolean>;
};

export class SafetyMonitor {
  private emergencyStop = false;

  constructor(private config: SafetyConfig) {}

  evaluateBeforeMotion(env: Environment): SafetyEvaluation {
    if (this.emergencyStop) {
      return { allowed: false, reason: "Emergency stop active", emergencyStop: true };
    }

    for (const rule of this.config.stopIfRules) {
      if (rule(env)) {
        this.emergencyStop = true;
        return {
          allowed: false,
          reason: "stop_if safety rule triggered",
          emergencyStop: true,
        };
      }
    }

    return { allowed: true, emergencyStop: false };
  }

  clampSpeed(requested: number): number {
    return Math.min(Math.abs(requested), this.config.maxSpeed) * Math.sign(requested || 1);
  }

  isEmergencyStop(): boolean {
    return this.emergencyStop;
  }

  reset(): void {
    this.emergencyStop = false;
  }
}

export function createSafetyConfigFromRobot(
  maxSpeed: number,
  stopIfRules: Array<(env: Environment) => boolean>,
): SafetyConfig {
  return { maxSpeed, stopIfRules };
}

export function applyEmergencyStop(state: RobotState): RobotState {
  return {
    ...state,
    emergencyStop: true,
    velocity: { linear: 0, angular: 0 },
  };
}
