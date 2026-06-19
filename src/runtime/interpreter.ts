import type { Expr, Program, RobotDecl, SafetyRule, Stmt, UnitKind } from "../ast/nodes.js";

export type RuntimeValue =
  | { kind: "number"; value: number; unit: UnitKind }
  | { kind: "bool"; value: boolean }
  | { kind: "string"; value: string }
  | { kind: "void" }
  | { kind: "scan"; nearestDistance: number }
  | { kind: "object"; typeName: string; fields: Record<string, RuntimeValue> }
  | { kind: "sensor"; name: string; sensorType: string }
  | { kind: "actuator"; name: string; actuatorType: string };

export type MotionCommand =
  | { kind: "drive"; linear: number; angular: number; actuator: string }
  | { kind: "stop"; actuator: string }
  | { kind: "move_to"; x: number; y: number; z: number; actuator: string }
  | { kind: "grip"; actuator: string }
  | { kind: "release"; actuator: string }
  | { kind: "open"; actuator: string }
  | { kind: "set_thrust"; thrust: number; actuator: string }
  | { kind: "hover"; actuator: string };

export interface RobotBackend {
  readSensor(sensorName: string, sensorType: string): RuntimeValue;
  executeMotion(cmd: MotionCommand): void;
  tick(dtMs: number): void;
  getState(): RobotState;
  setEmergencyStop?(active: boolean): void;
}

export type RobotState = {
  pose: { x: number; y: number; theta: number; z?: number };
  velocity: { linear: number; angular: number };
  emergencyStop: boolean;
};

export type InterpreterOptions = {
  backend: RobotBackend;
  maxLoopIterations?: number;
  onMotionBlocked?: (reason: string) => void;
  onLog?: (message: string) => void;
};

export type SafetyContext = {
  maxSpeed: number;
  stopIfRules: Array<(env: Environment) => boolean>;
  emergencyStop: boolean;
};

export class Environment {
  private bindings = new Map<string, RuntimeValue>();

  define(name: string, value: RuntimeValue): void {
    this.bindings.set(name, value);
  }

  get(name: string): RuntimeValue | undefined {
    return this.bindings.get(name);
  }

  set(name: string, value: RuntimeValue): void {
    this.bindings.set(name, value);
  }

  clone(): Environment {
    const env = new Environment();
    for (const [k, v] of this.bindings) env.define(k, v);
    return env;
  }
}

export class Interpreter {
  private env = new Environment();
  private safety: SafetyContext = {
    maxSpeed: Infinity,
    stopIfRules: [],
    emergencyStop: false,
  };
  private currentRobot: RobotDecl | null = null;

  constructor(private options: InterpreterOptions) {}

  run(program: Program, entryBehavior?: string): RobotState {
    for (const robot of program.robots) {
      this.setupRobot(robot);
      const behaviorName = entryBehavior ?? robot.behaviors[0]?.name;
      if (!behaviorName) continue;
      const behavior = robot.behaviors.find((b) => b.name === behaviorName);
      if (behavior) {
        this.executeBlock(behavior.body);
      }
    }
    return this.options.backend.getState();
  }

  private setupRobot(robot: RobotDecl): void {
    this.currentRobot = robot;
    this.env = new Environment();

    for (const sensor of robot.sensors) {
      this.env.define(sensor.name, {
        kind: "sensor",
        name: sensor.name,
        sensorType: sensor.sensorType,
      });
    }

    for (const actuator of robot.actuators) {
      this.env.define(actuator.name, {
        kind: "actuator",
        name: actuator.name,
        actuatorType: actuator.actuatorType,
      });
    }

    this.safety = { maxSpeed: Infinity, stopIfRules: [], emergencyStop: false };

    if (robot.safety) {
      for (const rule of robot.safety.rules) {
        this.registerSafetyRule(rule);
      }
    }
  }

  private registerSafetyRule(rule: SafetyRule): void {
    if (rule.kind === "MaxSpeedRule") {
      const val = this.evalExpr(rule.value);
      if (val.kind === "number") {
        this.safety.maxSpeed = val.value;
      }
    } else {
      this.safety.stopIfRules.push((env) => {
        const saved = this.env;
        this.env = env;
        const result = this.evalExpr(rule.condition);
        this.env = saved;
        return result.kind === "bool" && result.value;
      });
    }
  }

  private executeBlock(stmts: Stmt[]): void {
    for (const stmt of stmts) {
      this.executeStmt(stmt);
    }
  }

  private executeStmt(stmt: Stmt): void {
    switch (stmt.kind) {
      case "VarDecl": {
        const value = this.evalExpr(stmt.init);
        this.env.define(stmt.name, value);
        break;
      }
      case "IfStmt": {
        const cond = this.evalExpr(stmt.condition);
        if (cond.kind === "bool" && cond.value) {
          this.executeBlock(stmt.thenBranch);
        } else if (stmt.elseBranch) {
          this.executeBlock(stmt.elseBranch);
        }
        break;
      }
      case "LoopStmt": {
        const maxIter = this.options.maxLoopIterations ?? 10;
        for (let i = 0; i < maxIter; i++) {
          this.options.backend.tick(stmt.intervalMs);
          this.executeBlock(stmt.body);
          if (this.safety.emergencyStop) break;
        }
        break;
      }
      case "ExprStmt":
        this.evalExpr(stmt.expr);
        break;
      case "ReturnStmt":
        break;
    }
  }

  private evalExpr(expr: Expr): RuntimeValue {
    switch (expr.kind) {
      case "LiteralExpr":
        if (typeof expr.value === "boolean") return { kind: "bool", value: expr.value };
        if (typeof expr.value === "number") return { kind: "number", value: expr.value, unit: "none" };
        if (typeof expr.value === "string") return { kind: "string", value: expr.value };
        return { kind: "void" };

      case "UnitLiteralExpr":
        return { kind: "number", value: expr.value, unit: expr.unit };

      case "IdentExpr": {
        const val = this.env.get(expr.name);
        if (!val) throw new RuntimeError(`Undefined variable '${expr.name}'`, expr.span.start.line);
        return val;
      }

      case "BinaryExpr": {
        const left = this.evalExpr(expr.left);
        const right = this.evalExpr(expr.right);
        return this.evalBinary(expr.op, left, right);
      }

      case "UnaryExpr": {
        const operand = this.evalExpr(expr.operand);
        if (expr.op === "not") {
          return { kind: "bool", value: operand.kind === "bool" && !operand.value };
        }
        if (expr.op === "-" && operand.kind === "number") {
          return { kind: "number", value: -operand.value, unit: operand.unit };
        }
        return { kind: "void" };
      }

      case "MemberExpr": {
        const obj = this.evalExpr(expr.object);
        if (obj.kind === "scan") {
          if (expr.property === "nearest_distance") {
            return { kind: "number", value: obj.nearestDistance, unit: "m" };
          }
        }
        return { kind: "void" };
      }

      case "CallExpr":
        return this.evalCall(expr);

      default:
        return { kind: "void" };
    }
  }

  private evalBinary(op: string, left: RuntimeValue, right: RuntimeValue): RuntimeValue {
    if (op === "and") {
      return {
        kind: "bool",
        value: (left.kind === "bool" && left.value) && (right.kind === "bool" && right.value),
      };
    }
    if (op === "or") {
      return {
        kind: "bool",
        value: (left.kind === "bool" && left.value) || (right.kind === "bool" && right.value),
      };
    }

    if (left.kind === "bool" && right.kind === "bool") {
      switch (op) {
        case "==": return { kind: "bool", value: left.value === right.value };
        case "!=": return { kind: "bool", value: left.value !== right.value };
      }
    }

    if (left.kind === "number" && right.kind === "number") {
      switch (op) {
        case "+": return { kind: "number", value: left.value + right.value, unit: left.unit };
        case "-": return { kind: "number", value: left.value - right.value, unit: left.unit };
        case "*": return { kind: "number", value: left.value * right.value, unit: "none" };
        case "/": return { kind: "number", value: left.value / right.value, unit: "none" };
        case "<": return { kind: "bool", value: left.value < right.value };
        case "<=": return { kind: "bool", value: left.value <= right.value };
        case ">": return { kind: "bool", value: left.value > right.value };
        case ">=": return { kind: "bool", value: left.value >= right.value };
        case "==": return { kind: "bool", value: left.value === right.value };
        case "!=": return { kind: "bool", value: left.value !== right.value };
      }
    }

    return { kind: "void" };
  }

  private evalCall(expr: import("../ast/nodes.js").CallExpr): RuntimeValue {
    if (expr.callee.kind !== "MemberExpr" || expr.callee.object.kind !== "IdentExpr") {
      return { kind: "void" };
    }

    const targetName = expr.callee.object.name;
    const method = expr.callee.property;
    const target = this.env.get(targetName);

    if (!target) {
      throw new RuntimeError(`Undefined '${targetName}'`, expr.span.start.line);
    }

    if (target.kind === "sensor") {
      if (method === "read") {
        return this.options.backend.readSensor(target.name, target.sensorType);
      }
    }

    if (target.kind === "actuator") {
      return this.executeActuatorMethod(target.name, target.actuatorType, method, expr);
    }

    return { kind: "void" };
  }

  private executeActuatorMethod(
    name: string,
    actuatorType: string,
    method: string,
    expr: import("../ast/nodes.js").CallExpr,
  ): RuntimeValue {
    const motionMethods = ["drive", "move_to", "set_thrust", "grip", "release", "open", "hover"];
    if (motionMethods.includes(method) || method === "stop") {
      if (!this.checkSafetyBeforeMotion()) {
        this.options.onMotionBlocked?.("Safety rule triggered — motion blocked");
        this.options.backend.executeMotion({ kind: "stop", actuator: name });
        return { kind: "void" };
      }
    }

    switch (method) {
      case "stop": {
        this.options.backend.executeMotion({ kind: "stop", actuator: name });
        break;
      }
      case "drive": {
        const linear = this.getNamedArg(expr, "linear", 0);
        const angular = this.getNamedArg(expr, "angular", 0);
        const clampedLinear = Math.min(Math.abs(linear), this.safety.maxSpeed) * Math.sign(linear || 1);
        this.options.backend.executeMotion({
          kind: "drive",
          linear: clampedLinear,
          angular,
          actuator: name,
        });
        break;
      }
      case "move_to": {
        const x = this.getNamedArg(expr, "x", 0);
        const y = this.getNamedArg(expr, "y", 0);
        const z = this.getNamedArg(expr, "z", 0);
        this.options.backend.executeMotion({ kind: "move_to", x, y, z, actuator: name });
        break;
      }
      case "grip":
        this.options.backend.executeMotion({ kind: "grip", actuator: name });
        break;
      case "release":
        this.options.backend.executeMotion({ kind: "release", actuator: name });
        break;
      case "open":
        this.options.backend.executeMotion({ kind: "open", actuator: name });
        break;
      case "set_thrust": {
        const thrust = this.getNamedArg(expr, "thrust", 0);
        this.options.backend.executeMotion({ kind: "set_thrust", thrust, actuator: name });
        break;
      }
      case "hover":
        this.options.backend.executeMotion({ kind: "hover", actuator: name });
        break;
    }

    this.options.onLog?.(`${name}.${method}()`);
    return { kind: "void" };
  }

  private getNamedArg(expr: import("../ast/nodes.js").CallExpr, name: string, defaultVal: number): number {
    const arg = expr.namedArgs.find((a) => a.name === name);
    if (!arg) return defaultVal;
    const val = this.evalExpr(arg.value);
    return val.kind === "number" ? val.value : defaultVal;
  }

  private checkSafetyBeforeMotion(): boolean {
    if (this.safety.emergencyStop) return false;

    for (const rule of this.safety.stopIfRules) {
      if (rule(this.env)) {
        this.safety.emergencyStop = true;
        this.options.backend.setEmergencyStop?.(true);
        this.options.onLog?.("EMERGENCY STOP: safety rule triggered");
        return false;
      }
    }

    return true;
  }
}

export class RuntimeError extends Error {
  constructor(message: string, public line: number) {
    super(message);
    this.name = "RuntimeError";
  }
}

