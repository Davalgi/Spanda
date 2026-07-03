import { useCallback, useEffect, useState } from "react";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type Props = {
  baseUrl: string;
};

export function AssuranceDiagnosisPanel({ baseUrl }: Props) {
  const [assurance, setAssurance] = useState<Record<string, unknown> | null>(null);
  const [diagnosis, setDiagnosis] = useState<Record<string, unknown> | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
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

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  return (
    <section className="cc-panel">
      {error && <p className="error">{error}</p>}
      <h4>Assurance</h4>
      {assurance ? <pre>{JSON.stringify(assurance, null, 2)}</pre> : <p>Loading…</p>}
      <h4>Diagnosis</h4>
      {diagnosis ? <pre>{JSON.stringify(diagnosis, null, 2)}</pre> : <p>Loading…</p>}
      <p className="demo-hint">
        Program APIs: <code>POST /v1/programs/assure</code> ·{" "}
        <code>POST /v1/programs/diagnose</code>
      </p>
    </section>
  );
}
