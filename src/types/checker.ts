import type {
  BehaviorDecl,
  Expr,
  Program,
  RobotDecl,
  SafetyRule,
  Stmt,
  UnitKind,
  RoboType,
} from "../ast/nodes.js";
import {
  ACTUATOR_TYPES,
  BUILTIN_METHODS,
  SCAN_PROPERTIES,
  SENSOR_TYPES,
  TypeCheckError,
  resultUnitForBinary,
  unitsCompatible,
  type TypeError,
} from "./units.js";

type SymbolEntry = {
  name: string;
  roboType: RoboType;
  kind: "sensor" | "actuator" | "variable" | "behavior";
  sensorType?: string;
  actuatorType?: string;
};

export function typeCheck(program: Program): void {
  const checker = new TypeChecker();
  checker.checkProgram(program);
  if (checker.errors.length > 0) {
    throw new TypeCheckError(checker.errors);
  }
}

class TypeChecker {
  errors: TypeError[] = [];
  private symbols = new Map<string, SymbolEntry>();
  private currentRobot: RobotDecl | null = null;

  checkProgram(program: Program): void {
    for (const robot of program.robots) {
      this.checkRobot(robot);
    }
  }

  private checkRobot(robot: RobotDecl): void {
    this.currentRobot = robot;
    this.symbols.clear();

    for (const sensor of robot.sensors) {
      if (!SENSOR_TYPES[sensor.sensorType]) {
        this.error(`Unknown sensor type '${sensor.sensorType}'`, sensor.span.start.line, sensor.span.start.column);
      }
      this.symbols.set(sensor.name, {
        name: sensor.name,
        roboType: SENSOR_TYPES[sensor.sensorType] ?? { kind: "named", name: sensor.sensorType },
        kind: "sensor",
        sensorType: sensor.sensorType,
      });
    }

    for (const actuator of robot.actuators) {
      if (!ACTUATOR_TYPES[actuator.actuatorType]) {
        this.error(`Unknown actuator type '${actuator.actuatorType}'`, actuator.span.start.line, actuator.span.start.column);
      }
      this.symbols.set(actuator.name, {
        name: actuator.name,
        roboType: ACTUATOR_TYPES[actuator.actuatorType] ?? { kind: "named", name: actuator.actuatorType },
        kind: "actuator",
        actuatorType: actuator.actuatorType,
      });
    }

    if (robot.safety) {
      const saved = new Map(this.symbols);
      for (const rule of robot.safety.rules) {
        this.checkSafetyRule(rule);
      }
      this.symbols = saved;
    }

    for (const behavior of robot.behaviors) {
      this.symbols.set(behavior.name, {
        name: behavior.name,
        roboType: { kind: "void" },
        kind: "behavior",
      });
      this.checkBehavior(behavior);
    }
  }

  private checkSafetyRule(rule: SafetyRule): void {
    if (rule.kind === "MaxSpeedRule") {
      const t = this.checkExpr(rule.value);
      if (t.kind !== "number" || !unitsCompatible(t.unit, rule.unit)) {
        this.error(
          `Expected value with unit '${rule.unit}' for ${rule.name}`,
          rule.span.start.line,
          rule.span.start.column,
        );
      }
    } else {
      const t = this.checkExpr(rule.condition);
      if (t.kind !== "bool") {
        this.error("stop_if condition must be boolean", rule.span.start.line, rule.span.start.column);
      }
    }
  }

  private checkBehavior(behavior: BehaviorDecl): void {
    const parentScope = new Map(this.symbols);
    this.symbols = new Map(parentScope);
    for (const stmt of behavior.body) {
      this.checkStmt(stmt);
    }
    this.symbols = parentScope;
  }

  private checkStmt(stmt: Stmt): void {
    switch (stmt.kind) {
      case "VarDecl": {
        const t = this.checkExpr(stmt.init);
        this.symbols.set(stmt.name, {
          name: stmt.name,
          roboType: t,
          kind: "variable",
        });
        break;
      }
      case "IfStmt": {
        const cond = this.checkExpr(stmt.condition);
        if (cond.kind !== "bool") {
          this.error("if condition must be boolean", stmt.span.start.line, stmt.span.start.column);
        }
        for (const s of stmt.thenBranch) this.checkStmt(s);
        if (stmt.elseBranch) for (const s of stmt.elseBranch) this.checkStmt(s);
        break;
      }
      case "LoopStmt": {
        for (const s of stmt.body) this.checkStmt(s);
        break;
      }
      case "ExprStmt":
        this.checkExpr(stmt.expr);
        break;
      case "ReturnStmt":
        if (stmt.value) this.checkExpr(stmt.value);
        break;
    }
  }

  private checkExpr(expr: Expr): RoboType {
    switch (expr.kind) {
      case "LiteralExpr":
        if (typeof expr.value === "boolean") return { kind: "bool" };
        if (typeof expr.value === "number") return { kind: "number", unit: "none" };
        if (typeof expr.value === "string") return { kind: "string" };
        return { kind: "void" };

      case "UnitLiteralExpr":
        return { kind: "number", unit: expr.unit };

      case "IdentExpr": {
        const sym = this.symbols.get(expr.name);
        if (!sym) {
          this.error(`Undefined identifier '${expr.name}'`, expr.span.start.line, expr.span.start.column);
          return { kind: "void" };
        }
        return sym.roboType;
      }

      case "BinaryExpr": {
        const left = this.checkExpr(expr.left);
        const right = this.checkExpr(expr.right);
        const result = resultUnitForBinary(expr.op, left, right);
        if (!result) {
          this.error(
            `Invalid operation '${expr.op}' for types`,
            expr.span.start.line,
            expr.span.start.column,
          );
          return { kind: "void" };
        }
        return result;
      }

      case "UnaryExpr": {
        const operand = this.checkExpr(expr.operand);
        if (expr.op === "not" && operand.kind !== "bool") {
          this.error("Operand of 'not' must be boolean", expr.span.start.line, expr.span.start.column);
        }
        if (expr.op === "-" && operand.kind !== "number") {
          this.error("Operand of '-' must be numeric", expr.span.start.line, expr.span.start.column);
        }
        return expr.op === "not" ? { kind: "bool" } : operand;
      }

      case "MemberExpr": {
        const objType = this.checkExpr(expr.object);
        if (objType.kind === "scan") {
          const prop = SCAN_PROPERTIES[expr.property];
          if (!prop) {
            this.error(`Unknown scan property '${expr.property}'`, expr.span.start.line, expr.span.start.column);
            return { kind: "void" };
          }
          return prop;
        }
        if (objType.kind === "named") {
          const methods = BUILTIN_METHODS[objType.name];
          if (methods && methods[expr.property]) {
            return methods[expr.property].returns;
          }
        }
        this.error(`Unknown member '${expr.property}'`, expr.span.start.line, expr.span.start.column);
        return { kind: "void" };
      }

      case "CallExpr": {
        return this.checkCall(expr);
      }

      default:
        return { kind: "void" };
    }
  }

  private checkCall(expr: import("../ast/nodes.js").CallExpr): RoboType {
    if (expr.callee.kind !== "MemberExpr") {
      this.error("Only method calls are supported", expr.span.start.line, expr.span.start.column);
      return { kind: "void" };
    }

    const member = expr.callee;
    if (member.object.kind !== "IdentExpr") {
      this.error("Invalid call target", expr.span.start.line, expr.span.start.column);
      return { kind: "void" };
    }

    const sym = this.symbols.get(member.object.name);
    if (!sym) {
      this.error(`Undefined identifier '${member.object.name}'`, expr.span.start.line, expr.span.start.column);
      return { kind: "void" };
    }

    let typeName = "";
    if (sym.kind === "sensor" && sym.sensorType) typeName = sym.sensorType;
    else if (sym.kind === "actuator" && sym.actuatorType) typeName = sym.actuatorType;
    else if (sym.roboType.kind === "named") typeName = sym.roboType.name;
    else if (sym.roboType.kind === "scan") typeName = "Scan";

    const methods = BUILTIN_METHODS[typeName];
    const method = methods?.[member.property];
    if (!method) {
      this.error(
        `Unknown method '${member.property}' on ${typeName}`,
        expr.span.start.line,
        expr.span.start.column,
      );
      return { kind: "void" };
    }

    if (method.namedParams) {
      for (const arg of expr.namedArgs) {
        const expected = method.namedParams[arg.name];
        if (!expected) {
          this.error(`Unknown named argument '${arg.name}'`, arg.span.start.line, arg.span.start.column);
          continue;
        }
        const actual = this.checkExpr(arg.value);
        this.assertCompatible(expected, actual, arg.span.start.line, arg.span.start.column);
      }
    }

    for (const arg of expr.args) {
      this.checkExpr(arg);
    }

    if (member.property === "read" && typeName === "Lidar") {
      return { kind: "scan" };
    }

    return method.returns;
  }

  private assertCompatible(expected: RoboType, actual: RoboType, line: number, column: number): void {
    if (expected.kind === "void" && actual.kind === "void") return;
    if (expected.kind === "number" && actual.kind === "number") {
      if (!unitsCompatible(expected.unit, actual.unit)) {
        this.error(
          `Unit mismatch: expected '${expected.unit}', got '${actual.unit}'`,
          line,
          column,
        );
      }
      return;
    }
    if (expected.kind !== actual.kind) {
      this.error(`Type mismatch: expected ${expected.kind}, got ${actual.kind}`, line, column);
    }
  }

  private error(message: string, line: number, column: number): void {
    this.errors.push({ message, line, column });
  }
}

export { typeCheck as check };
