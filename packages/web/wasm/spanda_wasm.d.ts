/* tslint:disable */
/* eslint-disable */

export function wasm_check(source: string): any;

export function wasm_fmt(source: string): string;

export function wasm_ir(source: string): any;

export function wasm_run(source: string, max_loop_iterations: number): any;

export function wasm_telemetry_append(line: string): any;

export function wasm_telemetry_clear(): any;

export function wasm_telemetry_otlp(): any;

export function wasm_telemetry_prometheus(): any;

export function wasm_telemetry_stats(): any;

export function wasm_verify(source: string): any;

export function wasm_version(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly wasm_check: (a: number, b: number) => any;
    readonly wasm_fmt: (a: number, b: number) => [number, number];
    readonly wasm_ir: (a: number, b: number) => any;
    readonly wasm_run: (a: number, b: number, c: number) => any;
    readonly wasm_telemetry_append: (a: number, b: number) => any;
    readonly wasm_telemetry_clear: () => any;
    readonly wasm_telemetry_otlp: () => any;
    readonly wasm_telemetry_prometheus: () => any;
    readonly wasm_telemetry_stats: () => any;
    readonly wasm_verify: (a: number, b: number) => any;
    readonly wasm_version: () => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
