import { readFileSync } from "node:fs";
import { tokenize } from "./lexer/index.js";
import { parse } from "./parser/index.js";
import { typeCheck } from "./types/index.js";
import { Interpreter, type RobotBackend, type RobotState } from "./runtime/index.js";
import type { Program } from "./ast/nodes.js";

export type CompileResult = {
  program: Program;
  source: string;
};

export function compile(source: string): CompileResult {
  const tokens = tokenize(source);
  const program = parse(tokens);
  typeCheck(program);
  return { program, source };
}

export function compileFile(path: string): CompileResult {
  const source = readFileSync(path, "utf-8");
  return compile(source);
}

export type RunOptions = {
  backend: RobotBackend;
  entryBehavior?: string;
  maxLoopIterations?: number;
  onMotionBlocked?: (reason: string) => void;
  onLog?: (message: string) => void;
};

export function run(program: Program, options: RunOptions): RobotState {
  const interpreter = new Interpreter({
    backend: options.backend,
    maxLoopIterations: options.maxLoopIterations,
    onMotionBlocked: options.onMotionBlocked,
    onLog: options.onLog,
  });
  return interpreter.run(program, options.entryBehavior);
}

export function runSource(source: string, options: RunOptions): RobotState {
  const { program } = compile(source);
  return run(program, options);
}

export function runFile(path: string, options: RunOptions): RobotState {
  const { program } = compileFile(path);
  return run(program, options);
}
