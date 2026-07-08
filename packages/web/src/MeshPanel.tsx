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
  topology_components?: number;
  average_trust_score?: number;
  coordinator_status?: string;
  issues?: string[];
};

type MeshCoordinator = {
  entity_id?: string;
  status?: string;
};

type MeshTopology = {
  coordinator?: MeshCoordinator | null;
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
  const [topology, setTopology] = useState<MeshTopology | null>(null);
  const [graph, setGraph] = useState<Record<string, unknown> | null>(null);
  const [partitions, setPartitions] = useState<unknown[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [actionResult, setActionResult] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [nodesRes, healthRes, topologyRes, graphRes, partitionsRes] = await Promise.all([
        fetch(`${baseUrl}/v1/mesh/nodes`, { headers: authHeaders() }),
        fetch(`${baseUrl}/v1/mesh/health`, { headers: authHeaders() }),
        fetch(`${baseUrl}/v1/mesh/topology`, { headers: authHeaders() }),
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
      if (topologyRes.ok) {
        const body = await topologyRes.json();
        setTopology((body.topology as MeshTopology) ?? null);
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

  useRegisterTabRefresh(load, { busy });

  const discover = async () => {
    setBusy(true);
    setActionResult(null);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/mesh/discover`, {
        method: "POST",
        headers: authHeaders(),
        body: "{}",
      });
      const text = await res.text();
      if (!res.ok) throw new Error(`discover ${res.status}`);
      setActionResult(text);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const coordinatorId = topology?.coordinator?.entity_id ?? null;

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
          {
            label: "Coordinator",
            value: coordinatorId ?? "—",
          },
          { label: "Partitions", value: String(health?.active_partitions ?? 0) },
          {
            label: "Components",
            value: String(health?.topology_components ?? 0),
          },
          {
            label: "Avg trust",
            value:
              health?.average_trust_score != null
                ? health.average_trust_score.toFixed(2)
                : "—",
          },
        ]}
      />

      <CcSection title="Mesh actions" hint="Discovery refreshes entity reachability from the registry and entity graph.">
        <div className="cc-action-bar">
          <button type="button" disabled={busy} onClick={() => void load()}>
            Refresh
          </button>
          <button type="button" className="primary" disabled={busy} onClick={() => void discover()}>
            Discover
          </button>
        </div>
        {actionResult ? <pre className="cc-action-result">{actionResult}</pre> : null}
      </CcSection>

      <CcSection title="Entity reachability">
        {nodes.length === 0 ? (
          <CcEmptyState title="No mesh nodes" description="Run Discover or load a project with entities." />
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
