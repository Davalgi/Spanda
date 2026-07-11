/** Control Center assurance and diagnosis panels with structured reports. @module */

import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

export type AssuranceDiagnosisFocus = "assurance" | "diagnosis";

type Props = {
  baseUrl: string;
  focus: AssuranceDiagnosisFocus;
};

function asRecord(value: unknown): Record<string, unknown> | null {
  // Narrow unknown JSON values to plain objects.
  return value && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : null;
}

function asStringList(value: unknown): string[] {
  // Normalize string arrays from nested report fields.
  if (!Array.isArray(value)) return [];
  return value.map((item) => {
    if (typeof item === "string") return item;
    if (item && typeof item === "object") {
      const rec = item as Record<string, unknown>;
      return String(rec.message ?? rec.issue ?? rec.name ?? JSON.stringify(item));
    }
    return String(item);
  });
}

function ReportCards({ report, kind }: { report: Record<string, unknown>; kind: string }) {
  // Render pass/fail, issues, and domain-specific lists from assure/diagnose JSON.
  const nested = asRecord(report.report) ?? asRecord(report.summary) ?? report;
  const passed =
    typeof nested.passed === "boolean"
      ? nested.passed
      : typeof report.passed === "boolean"
        ? report.passed
        : null;
  const issues = [
    ...asStringList(nested.issues),
    ...asStringList(report.issues),
    ...asStringList(nested.gaps),
  ];
  const anomalies = asStringList(nested.anomalies ?? nested.anomaly_findings);
  const prognostics = asStringList(nested.prognostics ?? nested.prognostic_findings);
  const diagnoses = asStringList(nested.diagnoses ?? nested.findings ?? nested.results);
  const causal = asStringList(nested.causal_graph ?? nested.causal_summary ?? nested.causes);

  return (
    <div>
      <CcMiniStats
        items={[
          {
            label: "Result",
            value: passed === null ? "—" : passed ? "passed" : "failed",
          },
          { label: "Issues", value: issues.length },
          {
            label: kind === "assurance" ? "Anomalies" : "Diagnoses",
            value: kind === "assurance" ? anomalies.length : diagnoses.length,
          },
        ]}
      />
      {issues.length > 0 && (
        <CcSection title="Issues">
          <ul className="cc-event-log">
            {issues.map((item, index) => (
              <li key={`${item}-${index}`} className="error">
                {item}
              </li>
            ))}
          </ul>
        </CcSection>
      )}
      {kind === "assurance" && anomalies.length > 0 && (
        <CcSection title="Anomalies">
          <ul className="cc-event-log">
            {anomalies.map((item, index) => (
              <li key={`${item}-${index}`}>{item}</li>
            ))}
          </ul>
        </CcSection>
      )}
      {kind === "assurance" && prognostics.length > 0 && (
        <CcSection title="Prognostics">
          <ul className="cc-event-log">
            {prognostics.map((item, index) => (
              <li key={`${item}-${index}`}>{item}</li>
            ))}
          </ul>
        </CcSection>
      )}
      {kind === "diagnosis" && diagnoses.length > 0 && (
        <CcSection title="Diagnoses">
          <ul className="cc-event-log">
            {diagnoses.map((item, index) => (
              <li key={`${item}-${index}`}>{item}</li>
            ))}
          </ul>
        </CcSection>
      )}
      {causal.length > 0 && (
        <CcSection title="Causal summary">
          <ul className="cc-event-log">
            {causal.map((item, index) => (
              <li key={`${item}-${index}`}>{item}</li>
            ))}
          </ul>
        </CcSection>
      )}
      <details>
        <summary>Raw JSON</summary>
        <pre className="cc-action-result">{JSON.stringify(report, null, 2)}</pre>
      </details>
    </div>
  );
}

function PolicySummary({ data, title }: { data: Record<string, unknown> | null; title: string }) {
  // Show fleet policy summary as key metrics instead of raw JSON only.
  if (!data) return <CcEmptyState title={`Loading ${title.toLowerCase()}…`} />;
  const keys = Object.keys(data).slice(0, 8);
  return (
    <div>
      <CcMiniStats
        items={keys.slice(0, 4).map((key) => ({
          label: key,
          value:
            typeof data[key] === "object" ? JSON.stringify(data[key]).slice(0, 40) : String(data[key]),
        }))}
      />
      <details>
        <summary>Raw policy JSON</summary>
        <pre className="cc-action-result">{JSON.stringify(data, null, 2)}</pre>
      </details>
    </div>
  );
}

export function AssuranceDiagnosisPanel({ baseUrl, focus }: Props) {
  // Hold fleet summaries, program reports, and optional file/trace inputs.
  const [assurance, setAssurance] = useState<Record<string, unknown> | null>(null);
  const [diagnosis, setDiagnosis] = useState<Record<string, unknown> | null>(null);
  const [programAssure, setProgramAssure] = useState<Record<string, unknown> | null>(null);
  const [programDiagnose, setProgramDiagnose] = useState<Record<string, unknown> | null>(null);
  const [assureFile, setAssureFile] = useState("");
  const [diagnoseFile, setDiagnoseFile] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadSummaries = useCallback(async () => {
    // Load fleet assurance or diagnosis policy summary for the focused tab.
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
    // Run program-level assurance with optional file override.
    setBusy(true);
    setError(null);
    try {
      const body: Record<string, string> = {};
      if (assureFile.trim()) body.file = assureFile.trim();
      const res = await fetch(`${baseUrl}/v1/programs/assure`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      });
      if (!res.ok) throw new Error(`programs/assure ${res.status}`);
      setProgramAssure(await res.json());
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, assureFile]);

  const runProgramDiagnose = useCallback(async () => {
    // Run program-level diagnosis with optional file or .trace path.
    setBusy(true);
    setError(null);
    try {
      const body: Record<string, string> = {};
      if (diagnoseFile.trim()) body.file = diagnoseFile.trim();
      const res = await fetch(`${baseUrl}/v1/programs/diagnose`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      });
      if (!res.ok) throw new Error(`programs/diagnose ${res.status}`);
      setProgramDiagnose(await res.json());
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, diagnoseFile]);

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
          <PolicySummary data={assurance} title="Assurance summary" />
        </CcSection>

        <CcSection
          title="Loaded program — assurance"
          hint="CLI parity via POST /v1/programs/assure on the --program file."
          actions={
            <div className="cc-action-bar" style={{ flexWrap: "wrap" }}>
              <input
                type="text"
                placeholder="Optional program file"
                value={assureFile}
                onChange={(event) => setAssureFile(event.target.value)}
                style={{ minWidth: "12rem", flex: "1 1 12rem" }}
              />
              <button type="button" onClick={() => void runProgramAssure()} disabled={busy}>
                Run program assure
              </button>
            </div>
          }
        >
          {programAssure ? (
            <ReportCards report={programAssure} kind="assurance" />
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
        <PolicySummary data={diagnosis} title="Diagnosis summary" />
      </CcSection>

      <CcSection
        title="Loaded program — diagnosis"
        hint="CLI parity via POST /v1/programs/diagnose on the --program file or a .trace path."
        actions={
          <div className="cc-action-bar" style={{ flexWrap: "wrap" }}>
            <input
              type="text"
              placeholder="Optional program or .trace file"
              value={diagnoseFile}
              onChange={(event) => setDiagnoseFile(event.target.value)}
              style={{ minWidth: "12rem", flex: "1 1 12rem" }}
            />
            <button type="button" onClick={() => void runProgramDiagnose()} disabled={busy}>
              Run program diagnose
            </button>
          </div>
        }
      >
        {programDiagnose ? (
          <ReportCards report={programDiagnose} kind="diagnosis" />
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
