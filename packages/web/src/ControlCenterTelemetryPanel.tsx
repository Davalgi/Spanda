import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { connectTelemetryStream, type TelemetryStreamEvent } from "./controlCenterTelemetry";

type Props = {
  baseUrl: string;
};

export function ControlCenterTelemetryPanel({ baseUrl }: Props) {
  const [connected, setConnected] = useState(false);
  const [events, setEvents] = useState<TelemetryStreamEvent[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [streaming, setStreaming] = useState(false);

  const startStream = useCallback(() => {
    setStreaming(true);
    setEvents([]);
    setError(null);
    return connectTelemetryStream(
      baseUrl,
      (event) => {
        setEvents((current) => [...current.slice(-199), event]);
      },
      (isConnected, streamError) => {
        setConnected(isConnected);
        if (streamError) setError(streamError);
      },
    );
  }, [baseUrl]);

  useEffect(() => {
    if (!streaming) return undefined;
    const disconnect = startStream();
    return disconnect;
  }, [streaming, startStream]);

  const telemetryCount = events.filter((event) => event.channel === "telemetry" || event.type === "telemetry").length;
  const traceCount = events.filter((event) => event.channel === "trace" || event.type === "trace").length;
  const alertCount = events.filter((event) => event.channel === "alert" || event.type === "alert").length;

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcMiniStats
        items={[
          { label: "Stream", value: connected ? "live" : "offline", tone: connected ? "ok" : "warn" },
          { label: "Telemetry", value: telemetryCount },
          { label: "Traces", value: traceCount },
          { label: "Alerts", value: alertCount },
        ]}
      />

      <CcSection
        title="Live telemetry stream"
        hint="WebSocket /v1/stream/telemetry — resume offsets on reconnect."
        actions={
          <button type="button" onClick={() => setStreaming((value) => !value)}>
            {streaming ? "Stop stream" : "Start stream"}
          </button>
        }
      >
        {events.length === 0 ? (
          <CcEmptyState
            title={streaming ? "Waiting for events…" : "Stream stopped"}
            description="Start the stream to receive live telemetry, traces, and alerts from the Control Center API."
          />
        ) : (
          <ul className="cc-event-log">
            {events
              .slice()
              .reverse()
              .map((event, index) => (
                <li key={`${index}-${event.type ?? "event"}`}>
                  <span className="cc-event-type">{event.type ?? event.channel ?? "event"}</span>
                  <pre className="cc-event-payload">{JSON.stringify(event.payload ?? event, null, 2)}</pre>
                </li>
              ))}
          </ul>
        )}
      </CcSection>
    </div>
  );
}
