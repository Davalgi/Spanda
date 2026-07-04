import { useCallback, useState } from "react";
import { DEFAULT_SOURCE } from "./examples";
import { checkSource, runSource, type CheckResponse, type RunResponse } from "./spanda-wasm";
import { CcEmptyState, CcSection } from "./controlCenterUi";

type Props = {
  baseUrl?: string;
};

export function PlaygroundPanel({ baseUrl }: Props) {
  const [source, setSource] = useState(DEFAULT_SOURCE);
  const [diagnostics, setDiagnostics] = useState<CheckResponse["diagnostics"]>([]);
  const [runResult, setRunResult] = useState<RunResponse["result"] | null>(null);
  const [serverProgram, setServerProgram] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadServerProgram = useCallback(async () => {
    if (!baseUrl) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/source`);
      if (!res.ok) throw new Error(`programs/source ${res.status}`);
      const body = await res.json();
      const text = String(body.source ?? "");
      setServerProgram(String(body.file ?? body.label ?? "program.sd"));
      setSource(text);
      setDiagnostics([]);
      setRunResult(null);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  const handleCheck = useCallback(async () => {
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
        hint="Check and run Spanda programs in-browser. Load the server program when Control Center was started with --program."
      >
        {baseUrl && (
          <div className="cc-action-bar">
            <button type="button" onClick={() => void loadServerProgram()} disabled={busy}>
              Load server program
            </button>
            {serverProgram && (
              <span className="cc-section-hint">Loaded: {serverProgram}</span>
            )}
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
