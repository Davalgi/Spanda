import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type TraceRow = {
  path: string;
  absolute?: string;
  size_bytes?: number;
  modified_ms?: number;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function ReplayPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [traces, setTraces] = useState<TraceRow[]>([]);
  const [selectedTrace, setSelectedTrace] = useState("");
  const [result, setResult] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadTraces = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/traces?limit=50`);
      if (!res.ok) throw new Error(`traces ${res.status}`);
      const body = await res.json();
      const rows = (body.traces as TraceRow[]) ?? [];
      setTraces(rows);
      if (!selectedTrace && rows.length > 0) {
        setSelectedTrace(rows[0].path);
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, selectedTrace]);

  useEffect(() => {
    void loadTraces();
  }, [loadTraces]);

  useRegisterTabRefresh(loadTraces, { busy });

  const replay = async (mode: "inspect" | "deterministic" | "playback") => {
    if (mode !== "inspect" && (!hasToken || !can("Operate"))) return;
    if (!selectedTrace) return;
    setBusy(true);
    setError(null);
    setResult(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/replay`, {
        method: "POST",
        headers: mode === "inspect" ? { "Content-Type": "application/json" } : authHeaders(),
        body: JSON.stringify({
          file: selectedTrace,
          deterministic: mode === "deterministic",
          playback: mode === "playback",
        }),
      });
      const text = await res.text();
      if (!res.ok) throw new Error(`replay ${res.status}: ${text}`);
      setResult(text);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <section className="cc-replay-panel">
      <p className="demo-hint">
        Mission trace library from the project tree. Record with{" "}
        <code>spanda sim program.sd --record</code>.
      </p>
      <label>
        Trace file
        <select
          value={selectedTrace}
          onChange={(event) => setSelectedTrace(event.target.value)}
        >
          {traces.map((trace) => (
            <option key={trace.path} value={trace.path}>
              {trace.path}
              {trace.size_bytes ? ` (${trace.size_bytes} B)` : ""}
            </option>
          ))}
          {traces.length === 0 && <option value="">No traces found</option>}
        </select>
      </label>
      <div className="cc-action-bar">
        <button type="button" onClick={() => void replay("inspect")} disabled={busy || !selectedTrace}>
          Inspect trace
        </button>
        <button
          type="button"
          onClick={() => void replay("deterministic")}
          disabled={busy || !selectedTrace || !hasToken || !can("Operate")}
        >
          Deterministic replay
        </button>
        <button
          type="button"
          onClick={() => void replay("playback")}
          disabled={busy || !selectedTrace || !hasToken || !can("Operate")}
        >
          Playback
        </button>
      </div>
      {error && <p className="error">{error}</p>}
      {result && <pre>{result}</pre>}
    </section>
  );
}
