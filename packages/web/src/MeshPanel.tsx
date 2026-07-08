import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import {
  CcBadge,
  CcEmptyState,
  CcMiniStats,
  CcSection,
} from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type MeshNode = {
  entity_id: string;
  reachable: boolean;
  trust_score: number;
  transport?: string;
  capabilities?: string[];
};

type MeshHealth = {
  total_nodes?: number;
  reachable_nodes?: number;
  active_partitions?: number;
  average_trust_score?: number;
  issues?: string[];
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function MeshPanel({ baseUrl, authHeaders }: Props) {
  const [nodes, setNodes] = useState<MeshNode[]>([]);
  const [health, setHealth] = useState<MeshHealth | null>(null);
  const [graph, setGraph] = useState<Record<string, unknown> | null>(null);
  const [partitions, setPartitions] = useState<unknown[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [nodesRes, healthRes, graphRes, partitionsRes] = await Promise.all([
        fetch(`${baseUrl}/v1/mesh/nodes`, { headers: authHeaders() }),
        fetch(`${baseUrl}/v1/mesh/health`, { headers: authHeaders() }),
        fetch(`${baseUrl}/v1/mesh/graph`, { headers: authHeaders() }),
        fetch(`${baseUrl}/v1/mesh/partitions`, { headers: authHeaders() }),
      ]);
      if (nodesRes.ok) {
        const body = await nodesRes.json();
        setNodes(Array.isArray(body.nodes) ? body.nodes : []);
      }
      if (healthRes.ok) {
        const body = await healthRes.json();
        setHealth((body.health as MeshHealth) ?? null);
      }
      if (graphRes.ok) {
        const body = await graphRes.json();
        setGraph((body.graph as Record<string, unknown>) ?? null);
      }
      if (partitionsRes.ok) {
        const body = await partitionsRes.json();
        setPartitions(Array.isArray(body.partitions) ? body.partitions : []);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, busy);

  return (
    <div className="cc-panel">
      <header className="cc-panel-header">
        <h2>Autonomous Entity Mesh</h2>
        <p className="cc-panel-subtitle">
          Trust-aware inter-entity communication — topology, reachability, and partitions (not packet routing).
        </p>
      </header>

      {error ? <CcEmptyState title="Mesh unavailable" description={error} /> : null}

      <CcMiniStats
        items={[
          { label: "Nodes", value: String(health?.total_nodes ?? nodes.length) },
          { label: "Reachable", value: String(health?.reachable_nodes ?? "—") },
          { label: "Partitions", value: String(health?.active_partitions ?? 0) },
          {
            label: "Avg trust",
            value:
              health?.average_trust_score != null
                ? health.average_trust_score.toFixed(2)
                : "—",
          },
        ]}
      />

      <CcSection title="Entity reachability">
        {nodes.length === 0 ? (
          <CcEmptyState title="No mesh nodes" description="Run spanda mesh discover or load a project with entities." />
        ) : (
          <table className="cc-table">
            <thead>
              <tr>
                <th>Entity</th>
                <th>Transport</th>
                <th>Reachable</th>
                <th>Trust</th>
                <th>Capabilities</th>
              </tr>
            </thead>
            <tbody>
              {nodes.map((node) => (
                <tr key={node.entity_id}>
                  <td>{node.entity_id}</td>
                  <td>{node.transport ?? "—"}</td>
                  <td>
                    <CcBadge tone={node.reachable ? "ok" : "danger"}>
                      {node.reachable ? "yes" : "no"}
                    </CcBadge>
                  </td>
                  <td>{node.trust_score.toFixed(2)}</td>
                  <td>{(node.capabilities ?? []).join(", ") || "—"}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </CcSection>

      <CcSection title="Topology graph">
        {!graph ? (
          <CcEmptyState title="No graph data" />
        ) : (
          <pre className="cc-action-result">{JSON.stringify(graph, null, 2)}</pre>
        )}
      </CcSection>

      <CcSection title="Partitions">
        {partitions.length === 0 ? (
          <CcEmptyState title="No active partitions" />
        ) : (
          <pre className="cc-action-result">{JSON.stringify(partitions, null, 2)}</pre>
        )}
      </CcSection>

      {health?.issues && health.issues.length > 0 ? (
        <CcSection title="Mesh health issues">
          <ul>
            {health.issues.map((issue) => (
              <li key={issue}>{issue}</li>
            ))}
          </ul>
        </CcSection>
      ) : null}
    </div>
  );
}
