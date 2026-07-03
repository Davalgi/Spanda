import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { ControlCenterDataTable } from "./controlCenterDataTable";
import {
  decisionEventClass,
  decisionLayerClass,
  decisionRowClass,
  type DecisionTraceFrame,
} from "./decisionsPanelUi";

type DecisionData = {
  list: Record<string, unknown> | null;
  policies: Record<string, unknown> | null;
  traces: Record<string, unknown> | null;
  cache: Record<string, unknown> | null;
  escalations: Record<string, unknown> | null;
  mesh: Record<string, unknown> | null;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
};

export function DecisionsPanel({ baseUrl, authHeaders }: Props) {
  const [data, setData] = useState<DecisionData | null>(null);
  const [simResult, setSimResult] = useState<Record<string, unknown> | null>(null);
  const [liveTrace, setLiveTrace] = useState(false);
  const [policyJson, setPolicyJson] = useState("");
  const [policyBusy, setPolicyBusy] = useState(false);
  const [simBusy, setSimBusy] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(
    async (quiet = false) => {
      if (!quiet) setBusy(true);
      setError(null);
      try {
        const [listRes, policiesRes, tracesRes, cacheRes, escalationsRes, meshRes] =
          await Promise.all([
            fetch(`${baseUrl}/v1/decisions`),
            fetch(`${baseUrl}/v1/decision-policies`),
            fetch(`${baseUrl}/v1/decisions/traces`),
            fetch(`${baseUrl}/v1/decision-policy-cache`),
            fetch(`${baseUrl}/v1/decisions/escalations`),
            fetch(`${baseUrl}/v1/decisions/mesh`),
          ]);
        setData({
          list: listRes.ok ? await listRes.json() : null,
          policies: policiesRes.ok ? await policiesRes.json() : null,
          traces: tracesRes.ok ? await tracesRes.json() : null,
          cache: cacheRes.ok ? await cacheRes.json() : null,
          escalations: escalationsRes.ok ? await escalationsRes.json() : null,
          mesh: meshRes.ok ? await meshRes.json() : null,
        });
      } catch (err) {
        if (!quiet) setError(String(err));
      } finally {
        if (!quiet) setBusy(false);
      }
    },
    [baseUrl],
  );

  useEffect(() => {
    void load();
  }, [load]);

  useEffect(() => {
    if (!liveTrace) return undefined;
    const timer = window.setInterval(() => void load(true), 3000);
    return () => window.clearInterval(timer);
  }, [liveTrace, load]);

  const approveEscalation = async (escalationId: string) => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/decisions/escalate`, {
        method: "POST",
        headers: { "Content-Type": "application/json", ...authHeaders() },
        body: JSON.stringify({
          escalation_id: escalationId,
          approver: "control_center_operator",
        }),
      });
      if (!res.ok) throw new Error(`escalate ${res.status}`);
      await load(true);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const runSimWithTraces = async () => {
    setSimBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/simulation`, {
        method: "POST",
        headers: { "Content-Type": "application/json", ...authHeaders() },
        body: JSON.stringify({
          execute: true,
          decision_trace: true,
          record_trace: true,
          inject_health_faults: true,
        }),
      });
      if (!res.ok) throw new Error(`simulation ${res.status}`);
      setSimResult(await res.json());
      setLiveTrace(true);
      await load(true);
    } catch (err) {
      setError(String(err));
    } finally {
      setSimBusy(false);
    }
  };

  const pendingRaw = (data?.escalations as Record<string, unknown> | null)?.pending;
  const pending = Array.isArray(pendingRaw)
    ? (pendingRaw as Record<string, unknown>[])
    : [];
  const traceBody = data?.traces as Record<string, unknown> | null;
  const framesRaw = traceBody?.frames;
  const frames = Array.isArray(framesRaw) ? (framesRaw as DecisionTraceFrame[]) : [];
  const meshBody = data?.mesh as Record<string, unknown> | null;

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Decision operations"
        hint="Distributed decision architecture — reflex, local, fleet, and control-center layers."
        actions={
          <div className="cc-action-bar">
            <button type="button" onClick={() => void load()} disabled={busy}>
              Refresh
            </button>
            <button type="button" onClick={() => void runSimWithTraces()} disabled={busy || simBusy}>
              {simBusy ? "Running sim…" : "Run sim with traces"}
            </button>
            <button type="button" onClick={() => setLiveTrace((live) => !live)} disabled={busy}>
              {liveTrace ? "Live trace on (3s)" : "Live trace off"}
            </button>
          </div>
        }
      >
        {simResult && (
          <p className="cc-section-hint">
            Last sim:{" "}
            {String(
              (simResult.simulation as Record<string, unknown> | undefined)?.status ?? "completed",
            )}
          </p>
        )}
      </CcSection>

      <CcSection title="Pending escalations">
        {pending.length === 0 ? (
          <CcEmptyState title="No pending escalations" />
        ) : (
          <ControlCenterDataTable
            rows={pending}
            rowKey={(row) => String(row.escalation_id)}
            columns={[
              { key: "id", header: "ID", render: (row) => String(row.escalation_id ?? "—") },
              { key: "entity", header: "Entity", render: (row) => String(row.entity_id ?? "—") },
              { key: "action", header: "Action", render: (row) => String(row.action ?? "—") },
              { key: "reason", header: "Reason", render: (row) => String(row.reason ?? "—") },
            ]}
          />
        )}
        {pending.length > 0 && (
          <div className="cc-action-bar">
            {pending.map((row) => (
              <button
                key={String(row.escalation_id)}
                type="button"
                onClick={() => void approveEscalation(String(row.escalation_id))}
                disabled={busy}
              >
                Approve {String(row.escalation_id)}
              </button>
            ))}
          </div>
        )}
      </CcSection>

      <CcSection title="Live decision trace (v3)">
        {frames.length === 0 ? (
          <CcEmptyState
            title="No trace frames"
            description={
              traceBody?.error
                ? String(traceBody.error)
                : "Run sim with traces or enable SPANDA_DECISION_TRACE=1."
            }
          />
        ) : (
          <div className="cc-table-wrap">
            <table className="cc-data-table">
              <thead>
                <tr>
                  <th>Time</th>
                  <th>Event</th>
                  <th>Layer</th>
                  <th>Entity</th>
                  <th>Decision</th>
                  <th>Safety</th>
                  <th>Trust</th>
                  <th>Reason</th>
                </tr>
              </thead>
              <tbody>
                {frames.map((frame, index) => {
                  const payload = frame.payload ?? {};
                  const envelope = (payload.security_envelope as Record<string, unknown>) ?? {};
                  const event = String(frame.event ?? "—");
                  const layer = String(payload.layer ?? "—");
                  const safetyOk =
                    envelope.safety_validation_passed ??
                    (payload.safety_validation as Record<string, unknown> | undefined)?.passed;
                  const trustOk =
                    envelope.trust_validation_passed ??
                    (payload.trust_validation as Record<string, unknown> | undefined)?.passed;
                  return (
                    <tr key={index} className={decisionRowClass(event)}>
                      <td>{String(frame.sim_time_ms ?? "—")}</td>
                      <td>
                        <span className={decisionEventClass(event)}>{event}</span>
                      </td>
                      <td>
                        <span className={decisionLayerClass(layer)}>{layer}</span>
                      </td>
                      <td>{String(payload.entity_id ?? "—")}</td>
                      <td>{String(payload.decision ?? "—")}</td>
                      <td>{safetyOk === true ? "pass" : safetyOk === false ? "fail" : "—"}</td>
                      <td>{trustOk === true ? "pass" : trustOk === false ? "fail" : "—"}</td>
                      <td>{String(payload.reason ?? "—")}</td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        )}
      </CcSection>

      <CcSection title="Fleet mesh">
        {!meshBody ? (
          <CcEmptyState title="Fleet mesh unavailable" />
        ) : meshBody.configured !== true ? (
          <CcEmptyState
            title="Fleet mesh not configured"
            description={String(meshBody.hint ?? "Set SPANDA_FLEET_MESH_URL on the host.")}
          />
        ) : (
          <pre className="cc-action-result">{JSON.stringify(meshBody, null, 2)}</pre>
        )}
      </CcSection>

      {data?.policies && (
        <CcSection title="Policies">
          <details className="cc-json-details">
            <summary>Policy bundle</summary>
            <pre className="cc-action-result">{JSON.stringify(data.policies, null, 2)}</pre>
          </details>
          <label className="cc-field">
            Edit policy JSON
            <textarea
              rows={6}
              value={policyJson || JSON.stringify(data.policies, null, 2)}
              onChange={(event) => setPolicyJson(event.target.value)}
            />
          </label>
          <button
            type="button"
            disabled={policyBusy}
            onClick={async () => {
              setPolicyBusy(true);
              try {
                const body = policyJson.trim() || JSON.stringify(data.policies);
                await fetch(`${baseUrl}/v1/decision-policies`, {
                  method: "POST",
                  headers: { "Content-Type": "application/json", ...authHeaders() },
                  body,
                });
                await load();
              } finally {
                setPolicyBusy(false);
              }
            }}
          >
            Save policy draft
          </button>
        </CcSection>
      )}
    </div>
  );
}
