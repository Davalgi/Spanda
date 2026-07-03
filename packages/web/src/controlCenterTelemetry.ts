/** WebSocket telemetry stream helpers for Control Center. @module */

export type TelemetryStreamEvent = {
  type?: string;
  channel?: string;
  payload?: unknown;
  timestamp_ms?: number;
};

export type TelemetryStreamState = {
  connected: boolean;
  events: TelemetryStreamEvent[];
  error: string | null;
};

function wsUrl(baseUrl: string): string {
  const url = new URL(baseUrl);
  url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
  url.pathname = "/v1/stream/telemetry";
  return url.toString();
}

/** Open a telemetry WebSocket and push events to the callback until closed. */
export function connectTelemetryStream(
  baseUrl: string,
  onEvent: (event: TelemetryStreamEvent) => void,
  onStatus: (connected: boolean, error?: string) => void,
): () => void {
  let socket: WebSocket | null = null;
  try {
    socket = new WebSocket(wsUrl(baseUrl));
  } catch (error) {
    onStatus(false, String(error));
    return () => undefined;
  }

  socket.onopen = () => {
    onStatus(true);
    socket?.send(JSON.stringify({ type: "resume", telemetry_offset: 0, trace_offset: 0, alert_offset: 0 }));
  };

  socket.onmessage = (message) => {
    try {
      const parsed = JSON.parse(String(message.data)) as TelemetryStreamEvent;
      onEvent(parsed);
    } catch {
      onEvent({ type: "raw", payload: message.data });
    }
  };

  socket.onerror = () => onStatus(false, "WebSocket error");
  socket.onclose = () => onStatus(false);

  return () => {
    socket?.close();
    socket = null;
  };
}
