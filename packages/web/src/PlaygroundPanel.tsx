/** Control Center WASM playground with optional server program load. @module */

import { useCallback, useEffect, useState } from "react";
import { DEFAULT_SOURCE } from "./examples";
import { checkSource, runSource, type CheckResponse, type RunResponse } from "./spanda-wasm";
import { CcEmptyState, CcSection } from "./controlCenterUi";

type Props = {
  baseUrl?: string;
};

export function PlaygroundPanel({ baseUrl }: Props) {
  // Hold editor source and last check/run results for the WASM playground.
  const [source, setSource] = useState(DEFAULT_SOURCE);
  const [diagnostics, setDiagnostics] = useState<CheckResponse["diagnostics"]>([]);
  const [runResult, setRunResult] = useState<RunResponse["result"] | null>(null);
  const [serverProgram, setServerProgram] = useState<string | null>(null);
  const [fileQuery, setFileQuery] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [autoLoaded, setAutoLoaded] = useState(false);

  const loadServerProgram = useCallback(
    async (opts?: { file?: string; silent?: boolean }) => {
      // Fetch program source from Control Center when --program (or ?file=) is available.
      if (!baseUrl) return false;
      if (!opts?.silent) {
        setBusy(true);
        setError(null);
      }
      try {
        const params = new URLSearchParams();
        const file = (opts?.file ?? fileQuery).trim();
        if (file) params.set("file", file);
        const qs = params.toString();
        const res = await fetch(`${baseUrl}/v1/programs/source${qs ? `?${qs}` : ""}`);
        if (!res.ok) {
          if (opts?.silent && res.status === 400) return false;
          throw new Error(`programs/source ${res.status}`);
        }
        const body = await res.json();
        const text = String(body.source ?? "");
        if (!text.trim()) return false;
        setServerProgram(String(body.file ?? body.label ?? "program.sd"));
        setSource(text);
        setDiagnostics([]);
        setRunResult(null);
        return true;
      } catch (e) {
        if (!opts?.silent) setError(String(e));
        return false;
      } finally {
        if (!opts?.silent) setBusy(false);
      }
    },
    [baseUrl, fileQuery],
  );

  useEffect(() => {
    // Auto-load the server program once when the playground mounts with a base URL.
    if (!baseUrl || autoLoaded) return;
    setAutoLoaded(true);
    void loadServerProgram({ silent: true });
  }, [baseUrl, autoLoaded, loadServerProgram]);

  const handleCheck = useCallback(async () => {
    // Type-check the editor source via the WASM checker.
    setBusy(true);
    setError(null);
    setRunResult(null);
    try {
      const resp = await checkSource(source);
      setDiagnostics(resp.diagnostics);
      if (!resp.ok) setError("Type check failed");
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [source]);

  const handleRun = useCallback(async () => {
    // Execute the editor source via the WASM runner.
    setBusy(true);
    setError(null);
    try {
      const resp = await runSource(source);
      setRunResult(resp.result);
      if (!resp.ok) setError("Run failed");
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [source]);

  return (
    <div className="cc-panel cc-playground">
      {error && <div className="error">{error}</div>}
      <CcSection
        title="WASM playground"
        hint="Check and run Spanda programs in-browser. Auto-loads the server --program when available."
      >
        {baseUrl && (
          <div className="cc-action-bar" style={{ flexWrap: "wrap", gap: "0.5rem" }}>
            <input
              type="text"
              placeholder="Optional file path (?file=)"
              value={fileQuery}
              onChange={(event) => setFileQuery(event.target.value)}
              style={{ minWidth: "12rem", flex: "1 1 12rem" }}
            />
            <button type="button" onClick={() => void loadServerProgram()} disabled={busy}>
              Load server program
            </button>
            {serverProgram && <span className="cc-section-hint">Loaded: {serverProgram}</span>}
          </div>
        )}
        <textarea
          className="cc-playground-source"
          rows={14}
          value={source}
          onChange={(event) => setSource(event.target.value)}
        />
        <div className="cc-action-bar">
          <button type="button" onClick={() => void handleCheck()} disabled={busy}>
            Check
          </button>
          <button type="button" onClick={() => void handleRun()} disabled={busy}>
            Run
          </button>
        </div>
      </CcSection>
      {diagnostics.length > 0 && (
        <CcSection title="Diagnostics">
          <ul className="cc-event-log">
            {diagnostics.map((diag, index) => (
              <li key={`${diag.message}-${index}`}>
                <span className="cc-event-type">{diag.severity}</span> {diag.message}
              </li>
            ))}
          </ul>
        </CcSection>
      )}
      {runResult ? (
        <CcSection title="Run result">
          <pre className="cc-action-result">{JSON.stringify(runResult, null, 2)}</pre>
        </CcSection>
      ) : (
        <CcEmptyState title="Run output appears here" />
      )}
    </div>
  );
}
