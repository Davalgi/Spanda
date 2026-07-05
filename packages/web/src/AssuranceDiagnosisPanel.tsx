import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

export type AssuranceDiagnosisFocus = "assurance" | "diagnosis";

type Props = {
  baseUrl: string;
  focus: AssuranceDiagnosisFocus;
};

export function AssuranceDiagnosisPanel({ baseUrl, focus }: Props) {
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
      if (focus === "assurance") {
        const assuranceRes = await fetch(`${baseUrl}/v1/assurance/summary`);
        if (assuranceRes.ok) setAssurance(await assuranceRes.json());
      } else {
        const diagnosisRes = await fetch(`${baseUrl}/v1/diagnosis/summary`);
        if (diagnosisRes.ok) setDiagnosis(await diagnosisRes.json());
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, focus]);

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

  if (focus === "assurance") {
    return (
      <section className="cc-panel">
        {error && <p className="error">{error}</p>}

        <CcSection
          title="Fleet assurance summary"
          hint="Assurance policy from the loaded config — minimum score and recovery/resilience requirements."
        >
          {assurance ? (
            <pre className="cc-action-result">{JSON.stringify(assurance, null, 2)}</pre>
          ) : (
            <CcEmptyState title="Loading assurance summary…" />
          )}
        </CcSection>

        <CcSection
          title="Loaded program — assurance"
          hint="CLI parity via POST /v1/programs/assure on the --program file."
          actions={
            <div className="cc-action-bar">
              <button type="button" onClick={() => void runProgramAssure()} disabled={busy}>
                Run program assure
              </button>
            </div>
          }
        >
          {programAssure ? (
            <pre className="cc-action-result">{JSON.stringify(programAssure, null, 2)}</pre>
          ) : (
            <CcEmptyState
              title="Run program assurance"
              description="Requires control-center serve --program."
            />
          )}
        </CcSection>
      </section>
    );
  }

  return (
    <section className="cc-panel">
      {error && <p className="error">{error}</p>}

      <CcSection
        title="Fleet diagnosis summary"
        hint="Diagnosis policy from the loaded config — mitigation and anomaly-handler requirements."
      >
        {diagnosis ? (
          <pre className="cc-action-result">{JSON.stringify(diagnosis, null, 2)}</pre>
        ) : (
          <CcEmptyState title="Loading diagnosis summary…" />
        )}
      </CcSection>

      <CcSection
        title="Loaded program — diagnosis"
        hint="CLI parity via POST /v1/programs/diagnose on the --program file."
        actions={
          <div className="cc-action-bar">
            <button type="button" onClick={() => void runProgramDiagnose()} disabled={busy}>
              Run program diagnose
            </button>
          </div>
        }
      >
        {programDiagnose ? (
          <pre className="cc-action-result">{JSON.stringify(programDiagnose, null, 2)}</pre>
        ) : (
          <CcEmptyState
            title="Run program diagnosis"
            description="Requires control-center serve --program."
          />
        )}
      </CcSection>
    </section>
  );
}
