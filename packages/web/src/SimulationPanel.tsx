import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
  defaultProgram?: string;
};

export function SimulationPanel({
  baseUrl,
  authHeaders,
  can,
  hasToken,
  defaultProgram,
}: Props) {
  const [programFile, setProgramFile] = useState(defaultProgram ?? "");
  const [injectFaults, setInjectFaults] = useState(false);
  const [recordTrace, setRecordTrace] = useState(true);
  const [decisionTrace, setDecisionTrace] = useState(true);
  const [result, setResult] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (defaultProgram && !programFile) {
      setProgramFile(defaultProgram);
    }
  }, [defaultProgram, programFile]);

  const runSim = async (execute: boolean) => {
    if (execute && (!hasToken || !can("Deploy"))) return;
    setBusy(true);
    setError(null);
    setResult(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/simulation`, {
        method: "POST",
        headers: execute ? authHeaders() : { "Content-Type": "application/json" },
        body: JSON.stringify({
          file: programFile || undefined,
          execute,
          inject_health_faults: injectFaults,
          record_trace: recordTrace,
          decision_trace: decisionTrace,
        }),
      });
      const text = await res.text();
      if (!res.ok) throw new Error(`simulation ${res.status}: ${text}`);
      setResult(text);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <section className="cc-simulation-panel">
      <h3>Simulation</h3>
      <p className="demo-hint">
        Plan or execute program simulation with optional fault injection and decision traces.
      </p>
      <div className="digital-thread-filters">
        <label>
          Program file
          <input
            value={programFile}
            onChange={(event) => setProgramFile(event.target.value)}
            placeholder="src/mission.sd"
          />
        </label>
        <label className="cc-remember-row">
          <input
            type="checkbox"
            checked={injectFaults}
            onChange={(event) => setInjectFaults(event.target.checked)}
          />
          Inject health faults
        </label>
        <label className="cc-remember-row">
          <input
            type="checkbox"
            checked={recordTrace}
            onChange={(event) => setRecordTrace(event.target.checked)}
          />
          Record mission trace
        </label>
        <label className="cc-remember-row">
          <input
            type="checkbox"
            checked={decisionTrace}
            onChange={(event) => setDecisionTrace(event.target.checked)}
          />
          Decision trace (v3)
        </label>
      </div>
      <div className="cc-action-bar">
        <button type="button" onClick={() => void runSim(false)} disabled={busy}>
          Plan (dry-run)
        </button>
        <button
          type="button"
          onClick={() => void runSim(true)}
          disabled={busy || !hasToken || !can("Deploy")}
        >
          Execute simulation
        </button>
      </div>
      {error && <p className="error">{error}</p>}
      {result && <pre>{result}</pre>}
    </section>
  );
}
