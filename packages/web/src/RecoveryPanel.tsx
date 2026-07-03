import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import {
  CcBadge,
  CcEmptyState,
  CcMiniStats,
  CcSection,
  severityTone,
} from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

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

type GraphNode = { id: string; kind?: string; display_name?: string; recoverable?: boolean };
type GraphEdge = { from: string; to: string; relationship?: string };

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

function asArray<T>(value: unknown): T[] {
  return Array.isArray(value) ? (value as T[]) : [];
}

function formatPercent(value: unknown): string {
  const numeric = typeof value === "number" ? value : Number(value);
  if (!Number.isFinite(numeric)) return "—";
  return `${(numeric * 100).toFixed(0)}%`;
}

function displayCell(value: unknown): string {
  if (value == null) return "—";
  if (typeof value === "string" || typeof value === "number" || typeof value === "boolean") {
    return String(value);
  }
  return JSON.stringify(value);
}

function riskTone(risk: string | undefined): "ok" | "warn" | "danger" | "neutral" {
  const normalized = (risk ?? "unknown").toLowerCase();
  if (normalized === "low") return "ok";
  if (normalized === "medium") return "warn";
  if (normalized === "high" || normalized === "critical") return "danger";
  return "neutral";
}

export function RecoveryPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [plans, setPlans] = useState<RecoveryPlan[]>([]);
  const [metrics, setMetrics] = useState<RecoveryMetrics | null>(null);
  const [playbooks, setPlaybooks] = useState<Playbook[]>([]);
  const [history, setHistory] = useState<HistoryRow[]>([]);
  const [graphNodes, setGraphNodes] = useState<GraphNode[]>([]);
  const [graphEdges, setGraphEdges] = useState<GraphEdge[]>([]);
  const [entityId, setEntityId] = useState("");
  const [failure, setFailure] = useState("sensor_fault");
  const [playbook, setPlaybook] = useState("");
  const [actionResult, setActionResult] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [plansRes, metricsRes, playbooksRes, historyRes, graphRes] = await Promise.all([
        fetch(`${baseUrl}/v1/recovery/plans`),
        fetch(`${baseUrl}/v1/recovery/metrics`),
        fetch(`${baseUrl}/v1/recovery/playbooks`),
        fetch(`${baseUrl}/v1/recovery/history`),
        fetch(
          `${baseUrl}/v1/recovery/graph${entityId ? `?entity_id=${encodeURIComponent(entityId)}` : ""}`,
        ),
      ]);
      const plansJson = plansRes.ok ? await plansRes.json() : { plans: [] };
      const metricsJson = metricsRes.ok ? await metricsRes.json() : { metrics: null };
      const playbooksJson = playbooksRes.ok ? await playbooksRes.json() : { playbooks: [] };
      const historyJson = historyRes.ok ? await historyRes.json() : { history: [] };
      const graphJson = graphRes.ok ? await graphRes.json() : { graph: {} };
      const graph = (graphJson.graph as Record<string, unknown> | undefined) ?? {};
      setPlans(asArray<RecoveryPlan>(plansJson.plans));
      setMetrics((metricsJson.metrics as RecoveryMetrics | null) ?? null);
      setPlaybooks(asArray<Playbook>(playbooksJson.playbooks));
      setHistory(asArray<HistoryRow>(historyJson.history).slice(0, 10));
      setGraphNodes(asArray<GraphNode>(graph.nodes));
      setGraphEdges([
        ...asArray<GraphEdge>(graph.edges),
        ...asArray<GraphEdge>(graph.dependency_edges),
        ...asArray<GraphEdge>(graph.impact_edges),
        ...asArray<GraphEdge>(graph.recovery_edges),
      ]);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load recovery data");
    } finally {
      setBusy(false);
    }
  }, [baseUrl, entityId]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const recoveryPost = async (path: string, body: Record<string, unknown>) => {
    if (!hasToken || !can("Recover")) return;
    setBusy(true);
    setActionResult(null);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}${path}`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify(body),
      });
      const text = await res.text();
      if (!res.ok) throw new Error(`${path} ${res.status}`);
      setActionResult(text);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const requestBody = {
    entity_id: entityId || undefined,
    failure: failure || undefined,
    playbook: playbook || undefined,
  };

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      {busy && !metrics && plans.length === 0 && playbooks.length === 0 && (
        <p className="cc-section-hint">Loading recovery data…</p>
      )}

      {metrics && (
        <CcMiniStats
          items={[
            { label: "Total recoveries", value: metrics.total_recoveries },
            {
              label: "Success rate",
              value: formatPercent(metrics.success_rate),
              tone:
                typeof metrics.success_rate === "number" && metrics.success_rate >= 0.9
                  ? "ok"
                  : "warn",
            },
            {
              label: "Confidence",
              value: formatPercent(metrics.recovery_confidence),
            },
            { label: "Active plans", value: plans.length, tone: plans.length > 0 ? "warn" : "ok" },
          ]}
        />
      )}

      <CcSection
        title="Plan recovery"
        hint="Plan, simulate, or execute rollback for a failed entity."
      >
        <div className="cc-filter-bar">
          <label className="cc-field">
            Entity ID
            <input
              value={entityId}
              onChange={(event) => setEntityId(event.target.value)}
              placeholder="robot-001"
            />
          </label>
          <label className="cc-field">
            Failure type
            <input value={failure} onChange={(event) => setFailure(event.target.value)} />
          </label>
          <label className="cc-field">
            Playbook
            <input
              value={playbook}
              onChange={(event) => setPlaybook(event.target.value)}
              placeholder="optional"
            />
          </label>
        </div>

        {!hasToken && (
          <CcEmptyState
            title="Sign in to run recovery actions"
            description="Plan, simulate, execute, and validate require Recover permission."
          />
        )}

        <div className="cc-action-bar">
          <button
            type="button"
            disabled={busy || !hasToken || !can("Recover")}
            onClick={() => void recoveryPost("/v1/recovery/plan", requestBody)}
          >
            Plan
          </button>
          <button
            type="button"
            disabled={busy || !hasToken || !can("Recover")}
            onClick={() => void recoveryPost("/v1/recovery/simulate", requestBody)}
          >
            Simulate
          </button>
          <button
            type="button"
            className="primary"
            disabled={busy || !hasToken || !can("Recover")}
            onClick={() =>
              void recoveryPost("/v1/recovery/execute", { ...requestBody, force_execute: true })
            }
          >
            Execute
          </button>
          <button
            type="button"
            disabled={busy || !hasToken || !can("Recover")}
            onClick={() => void recoveryPost("/v1/recovery/validate", requestBody)}
          >
            Validate
          </button>
        </div>

        {actionResult && <pre className="cc-action-result">{actionResult}</pre>}
      </CcSection>

      <CcSection title="Active plans" hint="Recovery orchestrator queue.">
        {plans.length === 0 ? (
          <CcEmptyState title="No active recovery plans" />
        ) : (
          <div className="cc-table-wrap">
            <table className="cc-data-table">
              <thead>
                <tr>
                  <th>Plan</th>
                  <th>Entity</th>
                  <th>Failure</th>
                  <th>Risk</th>
                </tr>
              </thead>
              <tbody>
                {plans.map((plan, index) => (
                  <tr key={plan.plan_id ?? `${plan.entity_id}-${index}`}>
                    <td>{displayCell(plan.plan_id)}</td>
                    <td>{displayCell(plan.entity_id)}</td>
                    <td>{displayCell(plan.failure)}</td>
                    <td>
                      <CcBadge tone={riskTone(typeof plan.risk === "string" ? plan.risk : undefined)}>
                        {displayCell(plan.risk)}
                      </CcBadge>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </CcSection>

      <div className="cc-panel-grid">
        <CcSection title="Playbooks" hint="Automated recovery procedures.">
          {playbooks.length === 0 ? (
            <CcEmptyState title="No playbooks loaded" />
          ) : (
            <ul className="cc-card-list">
              {playbooks.map((entry) => (
                <li key={entry.name} className="cc-card-item">
                  <span className="cc-card-item-title">{entry.name}</span>
                  <span className="cc-card-item-meta">
                    v{entry.version ?? "1"} · {entry.trigger ?? "manual"} ·{" "}
                    {entry.steps?.length ?? 0} steps
                  </span>
                </li>
              ))}
            </ul>
          )}
        </CcSection>

        <CcSection title="Recovery graph" hint="Entities and relationships for impact analysis.">
          {graphNodes.length === 0 ? (
            <CcEmptyState title="No graph nodes" description="Filter by entity ID to narrow the graph." />
          ) : (
            <>
              <div className="cc-table-wrap">
                <table className="cc-data-table">
                  <thead>
                    <tr>
                      <th>Entity</th>
                      <th>Kind</th>
                      <th>Recoverable</th>
                    </tr>
                  </thead>
                  <tbody>
                    {graphNodes.map((node, index) => (
                      <tr key={node.id ?? `node-${index}`}>
                        <td>{displayCell(node.display_name ?? node.id)}</td>
                        <td>{displayCell(node.kind)}</td>
                        <td>
                          <CcBadge tone={node.recoverable ? "ok" : "neutral"}>
                            {node.recoverable ? "yes" : "no"}
                          </CcBadge>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
              {graphEdges.length > 0 && (
                <ul className="cc-edge-list">
                  {graphEdges.slice(0, 12).map((edge, index) => (
                    <li key={`${edge.from}-${edge.to}-${index}`}>
                      {edge.from} → {edge.to}{" "}
                      <span className="cc-edge-rel">({edge.relationship ?? "relates"})</span>
                    </li>
                  ))}
                </ul>
              )}
            </>
          )}
        </CcSection>
      </div>

      <CcSection title="Recent history" hint="Last ten recovery events.">
        {history.length === 0 ? (
          <CcEmptyState title="No recovery history" />
        ) : (
          <ul className="cc-incident-list">
            {history.map((row, index) => (
              <li key={index} className="cc-incident-item">
                <div className="cc-incident-header">
                  <span className="cc-incident-title">{row.root_cause ?? "Recovery event"}</span>
                  {row.status != null && row.status !== "" && (
                    <CcBadge tone={severityTone(String(row.status))}>{String(row.status)}</CcBadge>
                  )}
                </div>
                <p className="cc-alert-source">{row.timestamp ?? "—"}</p>
              </li>
            ))}
          </ul>
        )}
      </CcSection>
    </div>
  );
}
