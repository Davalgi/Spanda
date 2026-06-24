/**
 * Optional SQLite telemetry backend via Node.js built-in `node:sqlite`.
 * @module
 */

import { createRequire } from "node:module";
import { existsSync, mkdirSync } from "node:fs";
import { dirname } from "node:path";
import type { TelemetryEvent } from "./telemetry-store.js";

type SqliteDatabase = {
  exec(sql: string): void;
  prepare(sql: string): {
    all(...params: unknown[]): unknown[];
    run(...params: unknown[]): void;
  };
};

type DatabaseSyncCtor = new (path: string) => SqliteDatabase;

const require = createRequire(import.meta.url);

let databaseSyncCtor: DatabaseSyncCtor | null = null;

try {
  databaseSyncCtor = require("node:sqlite").DatabaseSync as DatabaseSyncCtor;
} catch {
  databaseSyncCtor = null;
}

export function sqliteBackendAvailable(): boolean {
  return databaseSyncCtor !== null;
}

export function envBackendSqlite(): boolean {
  return process.env.SPANDA_TELEMETRY_BACKEND?.toLowerCase() === "sqlite";
}

export function defaultSqliteStorePath(): string {
  return ".spanda/telemetry-store.db";
}

export function resolveSqlitePath(): string {
  return process.env.SPANDA_TELEMETRY_STORE_PATH ?? defaultSqliteStorePath();
}

function openDatabase(path: string): SqliteDatabase {
  if (!databaseSyncCtor) {
    throw new Error(
      "SPANDA_TELEMETRY_BACKEND=sqlite requires Node.js 22+ with node:sqlite, or use the native Rust CLI",
    );
  }
  const parent = dirname(path);
  if (!existsSync(parent)) {
    mkdirSync(parent, { recursive: true });
  }
  const db = new databaseSyncCtor(path);
  db.exec(`
    CREATE TABLE IF NOT EXISTS telemetry_events (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      kind TEXT NOT NULL,
      timestamp_ms REAL NOT NULL,
      session_id TEXT,
      device_id TEXT,
      sensor_id TEXT,
      task_name TEXT,
      metric TEXT,
      robot_id TEXT,
      payload TEXT NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_events_kind ON telemetry_events(kind);
    CREATE INDEX IF NOT EXISTS idx_events_session ON telemetry_events(session_id);
    CREATE INDEX IF NOT EXISTS idx_events_timestamp ON telemetry_events(timestamp_ms);
    CREATE TABLE IF NOT EXISTS heartbeat_liveness (
      target_kind TEXT NOT NULL,
      target_id TEXT NOT NULL,
      timestamp_ms REAL NOT NULL,
      PRIMARY KEY (target_kind, target_id)
    );
  `);
  return db;
}

function indexFields(event: TelemetryEvent): {
  deviceId?: string;
  sensorId?: string;
  taskName?: string;
  metric?: string;
  sessionId?: string;
  robotId?: string;
} {
  switch (event.kind) {
    case "device":
      return {
        deviceId: event.device_id,
        metric: event.metric,
        sessionId: event.session_id,
        robotId: event.robot_id,
      };
    case "sensor":
      return {
        sensorId: event.sensor_id,
        sessionId: event.session_id,
        robotId: event.robot_id,
      };
    case "heartbeat":
      return {
        taskName: event.task_name,
        sessionId: event.session_id,
        robotId: event.robot_id,
      };
    case "device_heartbeat":
      return {
        deviceId: event.device_id,
        sessionId: event.session_id,
        robotId: event.robot_id,
      };
    case "health":
      return { sessionId: event.session_id };
    case "session":
    case "runtime_metrics":
      return { sessionId: event.session_id };
    default:
      return {};
  }
}

export function sqliteAppendEvent(path: string, event: TelemetryEvent): void {
  const db = openDatabase(path);
  const fields = indexFields(event);
  db.prepare(
    `INSERT INTO telemetry_events
     (kind, timestamp_ms, session_id, device_id, sensor_id, task_name, metric, robot_id, payload)
     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`,
  ).run(
    event.kind,
    event.timestamp_ms,
    fields.sessionId ?? null,
    fields.deviceId ?? null,
    fields.sensorId ?? null,
    fields.taskName ?? null,
    fields.metric ?? null,
    fields.robotId ?? null,
    JSON.stringify(event),
  );
}

export function sqliteReadAll(path: string): TelemetryEvent[] {
  if (!existsSync(path)) {
    return [];
  }
  const db = openDatabase(path);
  const rows = db.prepare("SELECT payload FROM telemetry_events ORDER BY id ASC").all() as Array<{
    payload: string;
  }>;
  return rows.map((row) => JSON.parse(row.payload) as TelemetryEvent);
}

export function sqliteUpsertHeartbeat(
  path: string,
  targetKind: string,
  targetId: string,
  timestampMs: number,
): void {
  const db = openDatabase(path);
  db.prepare(
    `INSERT INTO heartbeat_liveness (target_kind, target_id, timestamp_ms)
     VALUES (?, ?, ?)
     ON CONFLICT(target_kind, target_id) DO UPDATE SET timestamp_ms = excluded.timestamp_ms`,
  ).run(targetKind, targetId, timestampMs);
}

export function sqliteReadHeartbeatIndex(path: string): {
  tasks: Record<string, number>;
  devices: Record<string, number>;
} {
  if (!existsSync(path)) {
    return { tasks: {}, devices: {} };
  }
  const db = openDatabase(path);
  const rows = db
    .prepare("SELECT target_kind, target_id, timestamp_ms FROM heartbeat_liveness")
    .all() as Array<{ target_kind: string; target_id: string; timestamp_ms: number }>;
  const tasks: Record<string, number> = {};
  const devices: Record<string, number> = {};
  for (const row of rows) {
    if (row.target_kind === "task") {
      tasks[row.target_id] = row.timestamp_ms;
    } else if (row.target_kind === "device") {
      devices[row.target_id] = row.timestamp_ms;
    }
  }
  return { tasks, devices };
}

export function sqliteCompact(path: string, maxEvents: number): void {
  if (!existsSync(path)) {
    return;
  }
  const db = openDatabase(path);
  db.prepare(
    `DELETE FROM telemetry_events WHERE id NOT IN (
      SELECT id FROM telemetry_events ORDER BY id DESC LIMIT ?
    )`,
  ).run(maxEvents);
}
