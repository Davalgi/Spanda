import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";

type RecoveryPlan = {
  plan_id?: string;
  entity_id: string;
  failure: string;
  risk: string;
};

type RecoveryMetrics = {
  total_recoveries: number;
  success_rate: number;
  recovery_confidence: number;
};

type Playbook = {
  name: string;
  version?: string;
  trigger?: string;
  steps?: unknown[];
};

type HistoryRow = {
  timestamp?: string;
  root_cause?: string;
  status?: string;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function RecoveryPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [plans, setPlans] = useState<RecoveryPlan[]>([]);
  const [metrics, setMetrics] = useState<RecoveryMetrics | null>(null);
  const [playbooks, setPlaybooks] = useState<Playbook[]>([]);
  const [history, setHistory] = useState<HistoryRow[]>([]);
  const [entityId, setEntityId] = useState("");
  const [failure, setFailure] = useState("sensor_fault");
  const [playbook, setPlaybook] = useState("");
  const [actionLog, setActionLog] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [plansRes, metricsRes, playbooksRes, historyRes] = await Promise.all([
        fetch(`${baseUrl}/v1/recovery/plans`),
        fetch(`${baseUrl}/v1/recovery/metrics`),
        fetch(`${baseUrl}/v1/recovery/playbooks`),
        fetch(`${baseUrl}/v1/recovery/history`),
      ]);
      const plansJson = await plansRes.json();
      const metricsJson = await metricsRes.json();
      const playbooksJson = playbooksRes.ok ? await playbooksRes.json() : { playbooks: [] };
      const historyJson = historyRes.ok ? await historyRes.json() : { history: [] };
      setPlans(Array.isArray(plansJson.plans) ? plansJson.plans : []);
      setMetrics(metricsJson.metrics ?? null);
      setPlaybooks(playbooksJson.playbooks ?? []);
      setHistory((historyJson.history ?? []).slice(0, 10));
    } catch (e) {
      setError(e instanceof Error ? e.message : "Failed to load recovery data");
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  const recoveryPost = async (path: string, body: Record<string, unknown>) => {
    if (!hasToken || !can("Recover")) return;
    setBusy(true);
    setActionLog(null);
    try {
      const res = await fetch(`${baseUrl}${path}`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify(body),
      });
      const text = await res.text();
      if (!res.ok) throw new Error(`${path} ${res.status}`);
      setActionLog(text);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const body = {
    entity_id: entityId || undefined,
    failure: failure || undefined,
    playbook: playbook || undefined,
  };

  return (
    <section className="recovery-panel">
      <header>
        <h2>Recovery Orchestrator</h2>
        <button type="button" onClick={() => void load()} disabled={busy}>
          {busy ? "Loading…" : "Refresh"}
        </button>
      </header>
      {error ? <p className="error">{error}</p> : null}
      {metrics ? (
        <div className="recovery-metrics">
          <span>Recoveries: {metrics.total_recoveries}</span>
          <span>Success: {(metrics.success_rate * 100).toFixed(0)}%</span>
          <span>Confidence: {(metrics.recovery_confidence * 100).toFixed(0)}%</span>
          <span>Active plans: {plans.length}</span>
        </div>
      ) : null}

      <div className="digital-thread-filters">
        <label>
          Entity ID
          <input
            value={entityId}
            onChange={(event) => setEntityId(event.target.value)}
            placeholder="robot-001"
          />
        </label>
        <label>
          Failure
          <input value={failure} onChange={(event) => setFailure(event.target.value)} />
        </label>
        <label>
          Playbook
          <input value={playbook} onChange={(event) => setPlaybook(event.target.value)} />
        </label>
      </div>
      <div className="cc-action-bar">
        <button
          type="button"
          disabled={busy || !hasToken || !can("Recover")}
          onClick={() => void recoveryPost("/v1/recovery/plan", body)}
        >
          Plan
        </button>
        <button
          type="button"
          disabled={busy || !hasToken || !can("Recover")}
          onClick={() => void recoveryPost("/v1/recovery/simulate", body)}
        >
          Simulate
        </button>
        <button
          type="button"
          disabled={busy || !hasToken || !can("Recover")}
          onClick={() => void recoveryPost("/v1/recovery/execute", { ...body, force_execute: true })}
        >
          Execute
        </button>
        <button
          type="button"
          disabled={busy || !hasToken || !can("Recover")}
          onClick={() => void recoveryPost("/v1/recovery/validate", body)}
        >
          Validate
        </button>
      </div>
      {actionLog && <pre>{actionLog}</pre>}

      <h3>Recovery Queue / Plans</h3>
      <table>
        <thead>
          <tr>
            <th>Plan</th>
            <th>Entity</th>
            <th>Failure</th>
            <th>Risk</th>
          </tr>
        </thead>
        <tbody>
          {plans.map((plan) => (
            <tr key={plan.plan_id ?? plan.entity_id}>
              <td>{plan.plan_id ?? "—"}</td>
              <td>{plan.entity_id}</td>
              <td>{plan.failure}</td>
              <td>{plan.risk}</td>
            </tr>
          ))}
          {plans.length === 0 && (
            <tr>
              <td colSpan={4}>No active recovery plans</td>
            </tr>
          )}
        </tbody>
      </table>

      <h3>Playbooks</h3>
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Version</th>
            <th>Trigger</th>
            <th>Steps</th>
          </tr>
        </thead>
        <tbody>
          {playbooks.map((pb) => (
            <tr key={pb.name}>
              <td>{pb.name}</td>
              <td>{pb.version ?? "—"}</td>
              <td>{pb.trigger ?? "—"}</td>
              <td>{pb.steps?.length ?? 0}</td>
            </tr>
          ))}
          {playbooks.length === 0 && (
            <tr>
              <td colSpan={4}>none</td>
            </tr>
          )}
        </tbody>
      </table>

      <h3>Recent history</h3>
      <table>
        <thead>
          <tr>
            <th>Time</th>
            <th>Root cause</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {history.map((row, idx) => (
            <tr key={idx}>
              <td>{row.timestamp ?? "—"}</td>
              <td>{row.root_cause ?? "—"}</td>
              <td>{row.status ?? "—"}</td>
            </tr>
          ))}
          {history.length === 0 && (
            <tr>
              <td colSpan={3}>none</td>
            </tr>
          )}
        </tbody>
      </table>
    </section>
  );
}
