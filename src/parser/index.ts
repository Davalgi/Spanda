import type { Token } from "../lexer/index.js";
import { unitFromLexeme } from "../lexer/index.js";
import type {
  ActuatorDecl,
  BehaviorDecl,
  BinaryOp,
  Expr,
  NamedArg,
  Program,
  RobotDecl,
  SafetyBlock,
  SafetyRule,
  SensorDecl,
  Span,
  Stmt,
  UnitKind,
} from "../ast/nodes.js";

export class ParseError extends Error {
  constructor(
    message: string,
    public line: number,
    public column: number,
  ) {
    super(message);
    this.name = "ParseError";
  }
}

export function parse(tokens: Token[]): Program {
  const parser = new Parser(tokens);
  return parser.parseProgram();
}

class Parser {
  private pos = 0;

  constructor(private tokens: Token[]) {}

  private peek(): Token {
    return this.tokens[this.pos];
  }

  private previous(): Token {
    return this.tokens[this.pos - 1];
  }

  private advance(): Token {
    if (this.peek().type !== "EOF") this.pos++;
    return this.previous();
  }

  private check(type: Token["type"]): boolean {
    return this.peek().type === type;
  }

  private match(...types: Token["type"][]): boolean {
    for (const t of types) {
      if (this.check(t)) {
        this.advance();
        return true;
      }
    }
    return false;
  }

  private expect(type: Token["type"], message: string): Token {
    if (this.check(type)) return this.advance();
    const t = this.peek();
    throw new ParseError(message, t.line, t.column);
  }

  private spanFrom(start: Token, end: Token): Span {
    return {
      start: { line: start.line, column: start.column, offset: start.offset },
      end: { line: end.line, column: end.column, offset: end.offset },
    };
  }

  parseProgram(): Program {
    const start = this.peek();
    const robots: RobotDecl[] = [];
    while (!this.check("EOF")) {
      robots.push(this.parseRobot());
    }
    const end = this.previous();
    return {
      kind: "Program",
      robots,
      span: this.spanFrom(start, end),
    };
  }

  private parseRobot(): RobotDecl {
    const start = this.expect("ROBOT", "Expected 'robot'");
    const nameTok = this.expect("IDENT", "Expected robot name");
    this.expect("LBRACE", "Expected '{' after robot name");

    const sensors: SensorDecl[] = [];
    const actuators: ActuatorDecl[] = [];
    let safety: SafetyBlock | null = null;
    const behaviors: BehaviorDecl[] = [];

    while (!this.check("RBRACE") && !this.check("EOF")) {
      if (this.check("SENSOR")) {
        sensors.push(this.parseSensor());
      } else if (this.check("ACTUATOR")) {
        actuators.push(this.parseActuator());
      } else if (this.check("SAFETY")) {
        safety = this.parseSafety();
      } else if (this.check("BEHAVIOR")) {
        behaviors.push(this.parseBehavior());
      } else {
        const t = this.peek();
        throw new ParseError("Expected robot member declaration", t.line, t.column);
      }
    }

    const end = this.expect("RBRACE", "Expected '}' to close robot block");
    return {
      kind: "RobotDecl",
      name: nameTok.lexeme,
      sensors,
      actuators,
      safety,
      behaviors,
      span: this.spanFrom(start, end),
    };
  }

  private parseSensor(): SensorDecl {
    const start = this.advance();
    const name = this.expect("IDENT", "Expected sensor name");
    this.expect("COLON", "Expected ':' after sensor name");
    const sensorType = this.expect("IDENT", "Expected sensor type");

    let topic: string | null = null;
    if (this.match("ON")) {
      const topicTok = this.expect("STRING", "Expected topic string after 'on'");
      topic = topicTok.value as string;
    }

    this.expect("SEMICOLON", "Expected ';' after sensor declaration");
    const end = this.previous();
    return {
      kind: "SensorDecl",
      name: name.lexeme,
      sensorType: sensorType.lexeme,
      topic,
      span: this.spanFrom(start, end),
    };
  }

  private parseActuator(): ActuatorDecl {
    const start = this.advance();
    const name = this.expect("IDENT", "Expected actuator name");
    this.expect("COLON", "Expected ':' after actuator name");
    const actuatorType = this.expect("IDENT", "Expected actuator type");
    this.expect("SEMICOLON", "Expected ';' after actuator declaration");
    const end = this.previous();
    return {
      kind: "ActuatorDecl",
      name: name.lexeme,
      actuatorType: actuatorType.lexeme,
      span: this.spanFrom(start, end),
    };
  }

  private parseSafety(): SafetyBlock {
    const start = this.advance();
    this.expect("LBRACE", "Expected '{' after safety");
    const rules: SafetyRule[] = [];

    while (!this.check("RBRACE") && !this.check("EOF")) {
      if (this.check("STOP_IF")) {
        rules.push(this.parseStopIfRule());
      } else if (this.check("IDENT")) {
        rules.push(this.parseMaxSpeedRule());
      } else {
        const t = this.peek();
        throw new ParseError("Expected safety rule", t.line, t.column);
      }
    }

    const end = this.expect("RBRACE", "Expected '}' to close safety block");
    return { kind: "SafetyBlock", rules, span: this.spanFrom(start, end) };
  }

  private parseMaxSpeedRule(): SafetyRule {
    const start = this.peek();
    const name = this.advance();
    this.expect("ASSIGN", "Expected '=' in safety rule");
    const value = this.parseExpr();
    let unit: UnitKind;
    if (value.kind === "UnitLiteralExpr") {
      unit = value.unit;
    } else {
      unit = this.parseUnitSuffix();
    }
    this.expect("SEMICOLON", "Expected ';' after safety rule");
    const end = this.previous();
    return {
      kind: "MaxSpeedRule",
      name: name.lexeme,
      value,
      unit,
      span: this.spanFrom(start, end),
    };
  }

  private parseStopIfRule(): SafetyRule {
    const start = this.advance();
    const condition = this.parseExpr();
    this.expect("SEMICOLON", "Expected ';' after stop_if rule");
    const end = this.previous();
    return { kind: "StopIfRule", condition, span: this.spanFrom(start, end) };
  }

  private parseBehavior(): BehaviorDecl {
    const start = this.advance();
    const name = this.expect("IDENT", "Expected behavior name");
    this.expect("LPAREN", "Expected '(' after behavior name");
    this.expect("RPAREN", "Expected ')' after behavior parameters");
    this.expect("LBRACE", "Expected '{' after behavior signature");
    const body = this.parseBlock();
    const end = this.expect("RBRACE", "Expected '}' to close behavior");
    return {
      kind: "BehaviorDecl",
      name: name.lexeme,
      body,
      span: this.spanFrom(start, end),
    };
  }

  private parseBlock(): Stmt[] {
    const stmts: Stmt[] = [];
    while (!this.check("RBRACE") && !this.check("EOF")) {
      stmts.push(this.parseStmt());
    }
    return stmts;
  }

  private parseStmt(): Stmt {
    const start = this.peek();

    if (this.match("LET")) {
      const name = this.expect("IDENT", "Expected variable name");
      this.expect("ASSIGN", "Expected '=' in let declaration");
      const init = this.parseExpr();
      this.expect("SEMICOLON", "Expected ';' after let declaration");
      const end = this.previous();
      return {
        kind: "VarDecl",
        name: name.lexeme,
        init,
        span: this.spanFrom(start, end),
      };
    }

    if (this.match("IF")) {
      const condition = this.parseExpr();
      this.expect("LBRACE", "Expected '{' after if condition");
      const thenBranch = this.parseBlock();
      this.expect("RBRACE", "Expected '}' after if block");

      let elseBranch: Stmt[] | null = null;
      if (this.match("ELSE")) {
        this.expect("LBRACE", "Expected '{' after else");
        elseBranch = this.parseBlock();
        this.expect("RBRACE", "Expected '}' after else block");
      }

      const end = this.previous();
      return {
        kind: "IfStmt",
        condition,
        thenBranch,
        elseBranch,
        span: this.spanFrom(start, end),
      };
    }

    if (this.match("LOOP")) {
      this.expect("EVERY", "Expected 'every' after loop");
      const interval = this.parseDuration();
      this.expect("LBRACE", "Expected '{' after loop interval");
      const body = this.parseBlock();
      const end = this.expect("RBRACE", "Expected '}' to close loop");
      return {
        kind: "LoopStmt",
        intervalMs: interval,
        body,
        span: this.spanFrom(start, end),
      };
    }

    const expr = this.parseExpr();
    this.expect("SEMICOLON", "Expected ';' after expression");
    const end = this.previous();
    return { kind: "ExprStmt", expr, span: this.spanFrom(start, end) };
  }

  private parseDuration(): number {
    const tok = this.peek();
    if (tok.type === "UNIT_LITERAL" && tok.unit === "ms") {
      this.advance();
      return tok.value as number;
    }
    if (tok.type === "UNIT_LITERAL" && tok.unit === "s") {
      this.advance();
      return (tok.value as number) * 1000;
    }
    if (tok.type === "NUMBER") {
      this.advance();
      if (this.check("IDENT") && this.peek().lexeme === "ms") {
        this.advance();
        return tok.value as number;
      }
    }
    throw new ParseError("Expected duration like 50ms", tok.line, tok.column);
  }

  private parseUnitSuffix(): UnitKind {
    const unit = this.tryParseUnitSuffix();
    if (!unit) {
      const t = this.peek();
      throw new ParseError("Expected unit suffix", t.line, t.column);
    }
    return unit;
  }

  private tryParseUnitSuffix(): UnitKind | null {
    if (this.check("UNIT_LITERAL")) {
      const t = this.advance();
      return unitFromLexeme(t.unit!);
    }

    if (this.check("IDENT") && this.peek().lexeme === "m" && this.tokens[this.pos + 1]?.type === "SLASH" && this.tokens[this.pos + 2]?.lexeme === "s") {
      this.advance();
      this.advance();
      this.advance();
      return "m/s";
    }

    if (this.check("IDENT") && this.peek().lexeme === "rad" && this.tokens[this.pos + 1]?.type === "SLASH" && this.tokens[this.pos + 2]?.lexeme === "s") {
      this.advance();
      this.advance();
      this.advance();
      return "rad/s";
    }

    if (this.check("IDENT")) {
      const lexeme = this.peek().lexeme;
      if (isUnitIdent(lexeme)) {
        this.advance();
        return unitFromLexeme(lexeme as import("../lexer/index.js").UnitLexeme);
      }
    }

    return null;
  }

  private parseExpr(): Expr {
    return this.parseOr();
  }

  private parseOr(): Expr {
    let left = this.parseAnd();
    while (this.match("OR")) {
      const opStart = this.previous();
      const right = this.parseAnd();
      left = {
        kind: "BinaryExpr",
        op: "or",
        left,
        right,
        span: this.spanFrom(
          { ...opStart, type: "OR" },
          this.previous(),
        ),
      };
    }
    return left;
  }

  private parseAnd(): Expr {
    let left = this.parseComparison();
    while (this.match("AND")) {
      const opStart = this.previous();
      const right = this.parseComparison();
      left = {
        kind: "BinaryExpr",
        op: "and",
        left,
        right,
        span: this.spanFrom(opStart, this.previous()),
      };
    }
    return left;
  }

  private parseComparison(): Expr {
    let left = this.parseAdditive();
    while (
      this.match("LT", "LTE", "GT", "GTE", "EQ", "NEQ")
    ) {
      const opTok = this.previous();
      const op = opTok.lexeme as BinaryOp;
      const right = this.parseAdditive();
      left = {
        kind: "BinaryExpr",
        op,
        left,
        right,
        span: this.spanFrom(opTok, this.previous()),
      };
    }
    return left;
  }

  private parseAdditive(): Expr {
    let left = this.parseMultiplicative();
    while (this.match("PLUS", "MINUS")) {
      const opTok = this.previous();
      const op = opTok.lexeme as BinaryOp;
      const right = this.parseMultiplicative();
      left = {
        kind: "BinaryExpr",
        op,
        left,
        right,
        span: this.spanFrom(opTok, this.previous()),
      };
    }
    return left;
  }

  private parseMultiplicative(): Expr {
    let left = this.parseUnary();
    while (this.match("STAR", "SLASH")) {
      const opTok = this.previous();
      const op = opTok.lexeme as BinaryOp;
      const right = this.parseUnary();
      left = {
        kind: "BinaryExpr",
        op,
        left,
        right,
        span: this.spanFrom(opTok, this.previous()),
      };
    }
    return left;
  }

  private parseUnary(): Expr {
    if (this.match("MINUS", "NOT")) {
      const opTok = this.previous();
      const op = (opTok.type === "NOT" ? "not" : "-") as import("../ast/nodes.js").UnaryOp;
      const operand = this.parseUnary();
      return {
        kind: "UnaryExpr",
        op,
        operand,
        span: this.spanFrom(opTok, this.previous()),
      };
    }
    return this.parsePostfix();
  }

  private parsePostfix(): Expr {
    let expr = this.parsePrimary();

    while (true) {
      if (this.match("DOT")) {
        const prop = this.expect("IDENT", "Expected property name after '.'");
        expr = {
          kind: "MemberExpr",
          object: expr,
          property: prop.lexeme,
          span: this.spanFrom(
            { ...prop, type: "DOT" },
            prop,
          ),
        };
      } else if (this.match("LPAREN")) {
        const args: Expr[] = [];
        const namedArgs: NamedArg[] = [];

        if (!this.check("RPAREN")) {
          do {
            if (this.check("IDENT") && this.tokens[this.pos + 1]?.type === "COLON") {
              const argName = this.advance();
              this.advance();
              const value = this.parseExpr();
              namedArgs.push({
                name: argName.lexeme,
                value,
                span: this.spanFrom(argName, this.previous()),
              });
            } else {
              args.push(this.parseExpr());
            }
          } while (this.match("COMMA"));
        }

        const end = this.expect("RPAREN", "Expected ')' after arguments");
        expr = {
          kind: "CallExpr",
          callee: expr,
          args,
          namedArgs,
          span: this.spanFrom(
            { line: expr.span.start.line, column: expr.span.start.column, offset: 0, type: "LPAREN", lexeme: "(", value: null },
            end,
          ),
        };
      } else {
        break;
      }
    }

    return expr;
  }

  private parsePrimary(): Expr {
    const start = this.peek();

    if (this.match("TRUE")) {
      return {
        kind: "LiteralExpr",
        value: true,
        span: this.spanFrom(start, this.previous()),
      };
    }
    if (this.match("FALSE")) {
      return {
        kind: "LiteralExpr",
        value: false,
        span: this.spanFrom(start, this.previous()),
      };
    }
    if (this.match("NUMBER")) {
      const tok = this.previous();
      const unit = this.tryParseUnitSuffix();
      if (unit) {
        return {
          kind: "UnitLiteralExpr",
          value: tok.value as number,
          unit,
          span: this.spanFrom(start, this.previous()),
        };
      }
      return {
        kind: "LiteralExpr",
        value: tok.value as number,
        span: this.spanFrom(start, tok),
      };
    }
    if (this.match("UNIT_LITERAL")) {
      const tok = this.previous();
      return {
        kind: "UnitLiteralExpr",
        value: tok.value as number,
        unit: unitFromLexeme(tok.unit!),
        span: this.spanFrom(start, tok),
      };
    }
    if (this.match("STRING")) {
      return {
        kind: "LiteralExpr",
        value: this.previous().value as string,
        span: this.spanFrom(start, this.previous()),
      };
    }
    if (this.match("IDENT")) {
      const tok = this.previous();
      return {
        kind: "IdentExpr",
        name: tok.lexeme,
        span: this.spanFrom(start, tok),
      };
    }
    if (this.match("LPAREN")) {
      const expr = this.parseExpr();
      const end = this.expect("RPAREN", "Expected ')' after expression");
      return { ...expr, span: this.spanFrom(start, end) };
    }

    const t = this.peek();
    throw new ParseError("Expected expression", t.line, t.column);
  }
}

export { parse as parseTokens };

function isUnitIdent(lexeme: string): boolean {
  return ["m", "s", "ms", "rad", "deg"].includes(lexeme);
}
