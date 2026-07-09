/**
 * spanda wasm module (spanda-wasm.ts).
 * @module
 */

import init, {
  wasm_check as wasmCheck,
  wasm_run as wasmRun,
  wasm_telemetry_append as wasmTelemetryAppend,
  wasm_telemetry_clear as wasmTelemetryClear,
  wasm_telemetry_otlp as wasmTelemetryOtlp,
  wasm_telemetry_prometheus as wasmTelemetryPrometheus,
  wasm_telemetry_stats as wasmTelemetryStats,
} from "../wasm/spanda_wasm.js";
import wasmUrl from "../wasm/spanda_wasm_bg.wasm?url";

export type Diagnostic = { message: string; line: number; column: number };

export type CheckResponse = { ok: boolean; diagnostics: Diagnostic[] };

export type RunResponse = {
  ok: boolean;
  result?: {
    state: {
      pose: { x: number; y: number; theta: number; z?: number };
      velocity: { linear: number; angular: number };
      emergency_stop: boolean;
    };
    events: string[];
    logs: string[];
  };
  diagnostics?: Diagnostic[];
};

export type TelemetryStats = {
  total_events: number;
  device_events: number;
  sensor_events: number;
  heartbeat_events: number;
  device_heartbeat_events: number;
  health_events: number;
  session_events: number;
  runtime_metrics_events: number;
  tracked_tasks: number;
  tracked_devices: number;
};

export type TelemetryResponse = {
  ok: boolean;
  error?: string;
  stats?: TelemetryStats;
  body?: string;
};

let wasmReady = false;
let wasmLoad: Promise<boolean> | null = null;

export function isWasmLoaded(): boolean {
  return wasmReady;
}

async function ensureWasm(): Promise<boolean> {
  if (wasmReady) return true;
  if (wasmLoad) return wasmLoad;

  wasmLoad = (async () => {
    try {
      await init(wasmUrl);
      wasmReady = true;
      return true;
    } catch (error) {
      wasmReady = false;
      if (import.meta.env.DEV) {
        console.error("Failed to load Spanda WASM module", error);
      }
      return false;
    } finally {
      wasmLoad = null;
    }
  })();

  return wasmLoad;
}

export async function loadWasm(): Promise<boolean> {
  return ensureWasm();
}

export async function checkSource(source: string): Promise<CheckResponse> {
  if (!(await ensureWasm())) {
    return { ok: false, diagnostics: [{ message: "WASM module not loaded", line: 1, column: 1 }] };
  }
  return wasmCheck(source) as CheckResponse;
}

export async function runSource(source: string, maxLoopIterations: number): Promise<RunResponse> {
  if (!(await ensureWasm())) {
    return { ok: false, diagnostics: [{ message: "WASM module not loaded", line: 1, column: 1 }] };
  }
  return wasmRun(source, maxLoopIterations) as RunResponse;
}

function telemetryUnavailable(): TelemetryResponse {
  return { ok: false, error: "WASM module not loaded" };
}

export async function telemetryClear(): Promise<TelemetryResponse> {
  if (!(await ensureWasm())) return telemetryUnavailable();
  return wasmTelemetryClear() as TelemetryResponse;
}

export async function telemetryAppend(line: string): Promise<TelemetryResponse> {
  if (!(await ensureWasm())) return telemetryUnavailable();
  return wasmTelemetryAppend(line) as TelemetryResponse;
}

export async function telemetryStats(): Promise<TelemetryResponse> {
  if (!(await ensureWasm())) return telemetryUnavailable();
  return wasmTelemetryStats() as TelemetryResponse;
}

export async function telemetryPrometheus(): Promise<TelemetryResponse> {
  if (!(await ensureWasm())) return telemetryUnavailable();
  return wasmTelemetryPrometheus() as TelemetryResponse;
}

export async function telemetryOtlp(): Promise<TelemetryResponse> {
  if (!(await ensureWasm())) return telemetryUnavailable();
  return wasmTelemetryOtlp() as TelemetryResponse;
}
