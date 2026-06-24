import { useCallback, useState } from "react";
import {
  telemetryClear,
  telemetryOtlp,
  telemetryPrometheus,
  telemetryStats,
} from "./spanda-wasm";

type Props = {
  refreshKey: number;
};

type TelemetryStatsPayload = {
  total_events?: number;
  device_events?: number;
  sensor_events?: number;
  heartbeat_events?: number;
};

export function TelemetryPanel({ refreshKey }: Props) {
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [stats, setStats] = useState<TelemetryStatsPayload | null>(null);
  const [prometheus, setPrometheus] = useState<string>("");
  const [otlp, setOtlp] = useState<string>("");

  const loadTelemetry = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [statsResp, promResp, otlpResp] = await Promise.all([
        telemetryStats(),
        telemetryPrometheus(),
        telemetryOtlp(),
      ]);
      if (!statsResp.ok) {
        throw new Error(statsResp.error ?? "telemetry stats failed");
      }
      setStats(statsResp.stats as TelemetryStatsPayload);
      setPrometheus(promResp.ok ? (promResp.body ?? "") : "");
      setOtlp(otlpResp.ok ? (otlpResp.body ?? "") : "");
    } catch (e) {
      setError(String(e));
      setStats(null);
      setPrometheus("");
      setOtlp("");
    } finally {
      setBusy(false);
    }
  }, [refreshKey]);

  const handleClear = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const resp = await telemetryClear();
      if (!resp.ok) {
        throw new Error(resp.error ?? "telemetry clear failed");
      }
      setStats(null);
      setPrometheus("");
      setOtlp("");
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, []);

  return (
    <div className="telemetry-panel">
      <div className="toolbar operations-toolbar">
        <button type="button" onClick={() => void loadTelemetry()} disabled={busy}>
          Load WASM telemetry
        </button>
        <button type="button" onClick={() => void handleClear()} disabled={busy}>
          Clear buffer
        </button>
      </div>
      {error && <div className="error">{error}</div>}
      {stats && (
        <div className="panel">
          <h2>In-memory telemetry</h2>
          <dl>
            <dt>Total events</dt>
            <dd>{stats.total_events ?? 0}</dd>
            <dt>Device events</dt>
            <dd>{stats.device_events ?? 0}</dd>
            <dt>Sensor events</dt>
            <dd>{stats.sensor_events ?? 0}</dd>
            <dt>Heartbeat events</dt>
            <dd>{stats.heartbeat_events ?? 0}</dd>
          </dl>
        </div>
      )}
      {prometheus && (
        <div className="panel">
          <h2>Prometheus export</h2>
          <pre>{prometheus}</pre>
        </div>
      )}
      {otlp && (
        <div className="panel">
          <h2>OTLP/JSON export</h2>
          <pre>{otlp.slice(0, 4000)}{otlp.length > 4000 ? "\n…" : ""}</pre>
        </div>
      )}
    </div>
  );
}
