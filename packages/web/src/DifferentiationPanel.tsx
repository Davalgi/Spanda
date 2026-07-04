import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { ControlCenterDataTable } from "./controlCenterDataTable";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type ExplainSection = {
  topic: string;
  summary: string;
  details: string[];
};

type TraceRow = {
  path: string;
  name?: string;
};

type Props = {
  baseUrl: string;
};

const EXPLAIN_MODES = ["program", "readiness", "verify", "safety"] as const;

export function DifferentiationPanel({ baseUrl }: Props) {
  const [contract, setContract] = useState<Record<string, unknown> | null>(null);
  const [explainMode, setExplainMode] = useState<(typeof EXPLAIN_MODES)[number]>("program");
  const [explain, setExplain] = useState<Record<string, unknown> | null>(null);
  const [traces, setTraces] = useState<TraceRow[]>([]);
  const [selectedTrace, setSelectedTrace] = useState("");
  const [audit, setAudit] = useState<Record<string, unknown> | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadTraces = useCallback(async () => {
    try {
      const res = await fetch(`${baseUrl}/v1/programs/traces?limit=20`);
      if (!res.ok) return;
      const body = await res.json();
      const rows = Array.isArray(body.traces)
        ? (body.traces as TraceRow[])
        : [];
      setTraces(rows);
      if (!selectedTrace && rows[0]?.path) {
        setSelectedTrace(String(rows[0].path));
      }
    } catch {
      /* optional */
    }
  }, [baseUrl, selectedTrace]);

  const runContractVerify = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/contract/verify`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: "{}",
      });
      if (!res.ok) throw new Error(`contract verify ${res.status}`);
      setContract(await res.json());
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  const runExplain = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/explain`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ mode: explainMode }),
      });
      if (!res.ok) throw new Error(`explain ${res.status}`);
      setExplain(await res.json());
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, explainMode]);

  const runAudit = useCallback(async () => {
    if (!selectedTrace) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/audit/decisions`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ file: selectedTrace, explain: true }),
      });
      if (!res.ok) throw new Error(`audit decisions ${res.status}`);
      setAudit(await res.json());
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, selectedTrace]);

  const refresh = useCallback(async () => {
    await loadTraces();
    await runContractVerify();
    await runExplain();
    if (selectedTrace) await runAudit();
  }, [loadTraces, runAudit, runContractVerify, runExplain, selectedTrace]);

  useEffect(() => {
    void loadTraces();
    void runContractVerify();
    void runExplain();
  }, [loadTraces, runContractVerify, runExplain]);

  useRegisterTabRefresh(() => void refresh(), { busy });

  const contractReport = (contract?.report ?? null) as Record<string, unknown> | null;
  const contracts = Array.isArray(contractReport?.contracts)
    ? (contractReport.contracts as Record<string, unknown>[])
    : [];
  const checks = Array.isArray(contractReport?.checks)
    ? (contractReport.checks as Record<string, unknown>[])
    : [];
  const explainReport = (explain?.report ?? null) as Record<string, unknown> | null;
  const sections = Array.isArray(explainReport?.sections)
    ? (explainReport.sections as ExplainSection[])
    : [];

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Mission contracts"
        hint="Static contract verification from mission plans and robot declarations — CLI parity with spanda contract verify."
        actions={
          <button type="button" onClick={() => void runContractVerify()} disabled={busy}>
            Re-verify
          </button>
        }
      >
        {contractReport ? (
          <>
            <p className="cc-section-hint">
              {contract?.passed === true ? "Passed" : "Issues found"} —{" "}
              {String(contract?.file ?? "loaded program")}
            </p>
            {contracts.length > 0 ? (
              <ControlCenterDataTable
                rows={contracts}
                rowKey={(row, index) => `${String(row.name ?? "contract")}-${index}`}
                columns={[
                  { key: "name", header: "Mission", render: (row) => String(row.name ?? "—") },
                  { key: "kind", header: "Kind", render: (row) => String(row.kind ?? "—") },
                  {
                    key: "objectives",
                    header: "Objectives",
                    render: (row) => String((row.objectives as string[] | undefined)?.length ?? 0),
                  },
                  {
                    key: "safety",
                    header: "Safety aligned",
                    render: (row) => (row.safety_aligned ? "yes" : "no"),
                  },
                ]}
              />
            ) : (
              <CcEmptyState title="No mission contracts in program" />
            )}
            {checks.length > 0 && (
              <details className="cc-json-details">
                <summary>{checks.length} contract checks</summary>
                <pre className="cc-action-result">{JSON.stringify(checks, null, 2)}</pre>
              </details>
            )}
          </>
        ) : (
          <CcEmptyState title="Run contract verify on the loaded program" />
        )}
      </CcSection>

      <CcSection
        title="Explainability"
        hint="Structured explain reports for the loaded program — spanda explain parity."
        actions={
          <div className="cc-action-bar">
            <label className="cc-field cc-inline-field">
              Mode
              <select
                value={explainMode}
                onChange={(event) =>
                  setExplainMode(event.target.value as (typeof EXPLAIN_MODES)[number])
                }
              >
                {EXPLAIN_MODES.map((mode) => (
                  <option key={mode} value={mode}>
                    {mode}
                  </option>
                ))}
              </select>
            </label>
            <button type="button" onClick={() => void runExplain()} disabled={busy}>
              Explain
            </button>
          </div>
        }
      >
        {sections.length === 0 ? (
          <CcEmptyState title="No explain sections" description="Load a program with --program." />
        ) : (
          sections.map((section) => (
            <details key={section.topic} className="cc-json-details" open>
              <summary>
                {section.topic}: {section.summary}
              </summary>
              <ul className="cc-event-log">
                {section.details.map((detail, index) => (
                  <li key={`${section.topic}-${index}`}>{detail}</li>
                ))}
              </ul>
            </details>
          ))
        )}
      </CcSection>

      <CcSection
        title="Decision audit trail"
        hint="v3 decision records from mission traces — spanda audit decisions parity."
        actions={
          <div className="cc-action-bar">
            <label className="cc-field cc-inline-field">
              Trace
              <select
                value={selectedTrace}
                onChange={(event) => setSelectedTrace(event.target.value)}
              >
                {traces.length === 0 ? (
                  <option value="">No traces found</option>
                ) : (
                  traces.map((trace) => (
                    <option key={trace.path} value={trace.path}>
                      {trace.name ?? trace.path}
                    </option>
                  ))
                )}
              </select>
            </label>
            <button type="button" onClick={() => void runAudit()} disabled={busy || !selectedTrace}>
              Audit + explain
            </button>
          </div>
        }
      >
        {!audit ? (
          <CcEmptyState
            title="Select a trace"
            description="Record a trace with spanda sim --record or run Decisions → Run sim with traces."
          />
        ) : (
          <>
            <p className="cc-section-hint">
              {String(audit.decision_count ?? 0)} decision(s) in {String(audit.file ?? "trace")}
            </p>
            {typeof audit.explanation === "string" && audit.explanation.length > 0 && (
              <pre className="cc-action-result">{audit.explanation}</pre>
            )}
            <details className="cc-json-details">
              <summary>Audit JSON</summary>
              <pre className="cc-action-result">{JSON.stringify(audit.report, null, 2)}</pre>
            </details>
          </>
        )}
      </CcSection>
    </div>
  );
}
