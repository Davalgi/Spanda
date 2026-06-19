export type SourceLocation = {
  line: number;
  column: number;
  offset: number;
};

export type Span = {
  start: SourceLocation;
  end: SourceLocation;
};

export type UnitKind =
  | "none"
  | "m"
  | "s"
  | "ms"
  | "rad"
  | "m/s"
  | "rad/s"
  | "deg";

export type RoboType =
  | { kind: "void" }
  | { kind: "bool" }
  | { kind: "number"; unit: UnitKind }
  | { kind: "string" }
  | { kind: "named"; name: string }
  | { kind: "scan" }
  | { kind: "pose" }
  | { kind: "velocity" };

export type Program = {
  kind: "Program";
  robots: RobotDecl[];
  span: Span;
};

export type RobotDecl = {
  kind: "RobotDecl";
  name: string;
  sensors: SensorDecl[];
  actuators: ActuatorDecl[];
  safety: SafetyBlock | null;
  behaviors: BehaviorDecl[];
  span: Span;
};

export type SensorDecl = {
  kind: "SensorDecl";
  name: string;
  sensorType: string;
  topic: string | null;
  span: Span;
};

export type ActuatorDecl = {
  kind: "ActuatorDecl";
  name: string;
  actuatorType: string;
  span: Span;
};

export type SafetyBlock = {
  kind: "SafetyBlock";
  rules: SafetyRule[];
  span: Span;
};

export type SafetyRule =
  | {
      kind: "MaxSpeedRule";
      name: string;
      value: Expr;
      unit: UnitKind;
      span: Span;
    }
  | {
      kind: "StopIfRule";
      condition: Expr;
      span: Span;
    };

export type BehaviorDecl = {
  kind: "BehaviorDecl";
  name: string;
  body: Stmt[];
  span: Span;
};

export type Stmt =
  | VarDecl
  | IfStmt
  | LoopStmt
  | ExprStmt
  | ReturnStmt;

export type VarDecl = {
  kind: "VarDecl";
  name: string;
  init: Expr;
  span: Span;
};

export type IfStmt = {
  kind: "IfStmt";
  condition: Expr;
  thenBranch: Stmt[];
  elseBranch: Stmt[] | null;
  span: Span;
};

export type LoopStmt = {
  kind: "LoopStmt";
  intervalMs: number;
  body: Stmt[];
  span: Span;
};

export type ExprStmt = {
  kind: "ExprStmt";
  expr: Expr;
  span: Span;
};

export type ReturnStmt = {
  kind: "ReturnStmt";
  value: Expr | null;
  span: Span;
};

export type Expr =
  | LiteralExpr
  | UnitLiteralExpr
  | IdentExpr
  | BinaryExpr
  | UnaryExpr
  | CallExpr
  | MemberExpr
  | DriveArgsExpr;

export type LiteralExpr = {
  kind: "LiteralExpr";
  value: number | string | boolean | null;
  span: Span;
};

export type UnitLiteralExpr = {
  kind: "UnitLiteralExpr";
  value: number;
  unit: UnitKind;
  span: Span;
};

export type IdentExpr = {
  kind: "IdentExpr";
  name: string;
  span: Span;
};

export type BinaryExpr = {
  kind: "BinaryExpr";
  op: BinaryOp;
  left: Expr;
  right: Expr;
  span: Span;
};

export type UnaryExpr = {
  kind: "UnaryExpr";
  op: UnaryOp;
  operand: Expr;
  span: Span;
};

export type CallExpr = {
  kind: "CallExpr";
  callee: Expr;
  args: Expr[];
  namedArgs: NamedArg[];
  span: Span;
};

export type NamedArg = {
  name: string;
  value: Expr;
  span: Span;
};

export type MemberExpr = {
  kind: "MemberExpr";
  object: Expr;
  property: string;
  span: Span;
};

export type DriveArgsExpr = {
  kind: "DriveArgsExpr";
  linear: Expr | null;
  angular: Expr | null;
  span: Span;
};

export type BinaryOp =
  | "+"
  | "-"
  | "*"
  | "/"
  | "<"
  | "<="
  | ">"
  | ">="
  | "=="
  | "!="
  | "and"
  | "or";

export type UnaryOp = "-" | "not";
