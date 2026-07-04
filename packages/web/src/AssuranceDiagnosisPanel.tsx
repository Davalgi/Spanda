import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type Props = {
  baseUrl: string;
};

export function AssuranceDiagnosisPanel({ baseUrl }: Props) {
  const [assurance, setAssurance] = useState<Record<string, unknown> | null>(null);
  const [diagnosis, setDiagnosis] = useState<Record<string, unknown> | null>(null);
  const [programAssure, setProgramAssure] = useState<Record<string, unknown> | null>(null);
  const [programDiagnose, setProgramDiagnose] = useState<Record<string, unknown> | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadSummaries = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [assuranceRes, diagnosisRes] = await Promise.all([
        fetch(`${baseUrl}/v1/assurance/summary`),
        fetch(`${baseUrl}/v1/diagnosis/summary`),
      ]);
      if (assuranceRes.ok) setAssurance(await assuranceRes.json());
      if (diagnosisRes.ok) setDiagnosis(await diagnosisRes.json());
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  const runProgramAssure = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/assure`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: "{}",
      });
      if (!res.ok) throw new Error(`programs/assure ${res.status}`);
      setProgramAssure(await res.json());
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  const runProgramDiagnose = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/diagnose`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: "{}",
      });
      if (!res.ok) throw new Error(`programs/diagnose ${res.status}`);
      setProgramDiagnose(await res.json());
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void loadSummaries();
  }, [loadSummaries]);

  useRegisterTabRefresh(() => void loadSummaries(), { busy });

  return (
    <section className="cc-panel">
      {error && <p className="error">{error}</p>}

      <CcSection
        title="Fleet assurance summary"
        hint="Aggregated assurance state from Control Center."
      >
        {assurance ? (
          <pre className="cc-action-result">{JSON.stringify(assurance, null, 2)}</pre>
        ) : (
          <CcEmptyState title="Loading assurance summary…" />
        )}
      </CcSection>

      <CcSection title="Fleet diagnosis summary" hint="Aggregated diagnosis state.">
        {diagnosis ? (
          <pre className="cc-action-result">{JSON.stringify(diagnosis, null, 2)}</pre>
        ) : (
          <CcEmptyState title="Loading diagnosis summary…" />
        )}
      </CcSection>

      <CcSection
        title="Loaded program — assurance & diagnosis"
        hint="CLI parity via POST /v1/programs/assure and /v1/programs/diagnose on the --program file."
        actions={
          <div className="cc-action-bar">
            <button type="button" onClick={() => void runProgramAssure()} disabled={busy}>
              Run program assure
            </button>
            <button type="button" onClick={() => void runProgramDiagnose()} disabled={busy}>
              Run program diagnose
            </button>
          </div>
        }
      >
        {programAssure && (
          <details className="cc-json-details" open>
            <summary>Program assurance report</summary>
            <pre className="cc-action-result">{JSON.stringify(programAssure, null, 2)}</pre>
          </details>
        )}
        {programDiagnose && (
          <details className="cc-json-details" open>
            <summary>Program diagnosis report</summary>
            <pre className="cc-action-result">{JSON.stringify(programDiagnose, null, 2)}</pre>
          </details>
        )}
        {!programAssure && !programDiagnose && (
          <CcEmptyState title="Run program-level checks" description="Requires control-center serve --program." />
        )}
      </CcSection>
    </section>
  );
}
