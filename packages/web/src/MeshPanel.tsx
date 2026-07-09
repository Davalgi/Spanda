import { useCallback, useEffect, useMemo, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import {
  CcBadge,
  CcEmptyState,
  CcMiniStats,
  CcSection,
} from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";
import { MeshMiniGraph, type MeshGraphPayload, type MeshGraphViewMode } from "./MeshMiniGraph";

const DISCOVERY_SOURCES = [
  { id: "local_runtime", label: "Local runtime", hint: "Entity registry projection" },
  { id: "entity_graph", label: "Entity graph", hint: "CommunicatesWith edges" },
  { id: "mqtt", label: "MQTT", hint: "Live when SPANDA_LIVE_MQTT=1" },
  { id: "ros2", label: "ROS 2", hint: "Live when SPANDA_LIVE_ROS2=1" },
  { id: "dds", label: "DDS", hint: "ROS2 probe shim when live" },
] as const;

const TRANSPORT_FILTERS = [
  { id: "all", label: "All transports" },
  { id: "local_runtime", label: "Local runtime" },
  { id: "mqtt", label: "MQTT" },
  { id: "ros2", label: "ROS 2" },
  { id: "dds", label: "DDS" },
] as const;

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

type MeshRouteHop = {
  entity_id?: string;
};

type MeshRoute = {
  hops?: MeshRouteHop[];
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

function parseMeshGraph(value: Record<string, unknown> | null): MeshGraphPayload | null {
  if (!value) return null;
  const nodes = Array.isArray(value.nodes) ? value.nodes : [];
  const edges = Array.isArray(value.edges) ? value.edges : [];
  return {
    nodes: nodes.filter(
      (node): node is MeshGraphPayload["nodes"][number] =>
        typeof node === "object" && node != null && typeof (node as { id?: unknown }).id === "string",
    ),
    edges: edges.filter(
      (edge): edge is MeshGraphPayload["edges"][number] =>
        typeof edge === "object" &&
        edge != null &&
        typeof (edge as { from?: unknown }).from === "string" &&
        typeof (edge as { to?: unknown }).to === "string",
    ),
  };
}

function partitionMemberIds(partitions: unknown[]): string[] {
  const ids = new Set<string>();
  for (const entry of partitions) {
    if (typeof entry !== "object" || entry == null) continue;
    const members = (entry as { members?: unknown }).members;
    if (!Array.isArray(members)) continue;
    for (const member of members) {
      if (typeof member === "string") ids.add(member);
    }
  }
  return [...ids];
}

export function MeshPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [nodes, setNodes] = useState<MeshNode[]>([]);
  const [health, setHealth] = useState<MeshHealth | null>(null);
  const [topology, setTopology] = useState<MeshTopology | null>(null);
  const [graph, setGraph] = useState<Record<string, unknown> | null>(null);
  const [partitions, setPartitions] = useState<unknown[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [actionResult, setActionResult] = useState<string | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [graphView, setGraphView] = useState<MeshGraphViewMode>("full");
  const [selectedSources, setSelectedSources] = useState<string[]>([
    "local_runtime",
    "entity_graph",
  ]);
  const [transportFilter, setTransportFilter] = useState<string>("all");
  const [capabilityQuery, setCapabilityQuery] = useState("");
  const [routeTarget, setRouteTarget] = useState("");
  const [routeHighlight, setRouteHighlight] = useState<string[]>([]);
  const [partitionSelection, setPartitionSelection] = useState<Set<string>>(new Set());
  const [mergeReport, setMergeReport] = useState<string | null>(null);

  const meshGraph = useMemo(() => parseMeshGraph(graph), [graph]);
  const partitionNodeIds = useMemo(() => partitionMemberIds(partitions), [partitions]);

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
        const nextNodes = Array.isArray(body.nodes) ? body.nodes : [];
        setNodes(nextNodes);
        setSelectedId((current) => {
          if (current && nextNodes.some((node: MeshNode) => node.entity_id === current)) {
            return current;
          }
          return nextNodes[0]?.entity_id ?? null;
        });
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

  const toggleSource = (sourceId: string) => {
    setSelectedSources((current) =>
      current.includes(sourceId)
        ? current.filter((value) => value !== sourceId)
        : [...current, sourceId],
    );
  };

  const togglePartitionEntity = (entityId: string) => {
    setPartitionSelection((current) => {
      const next = new Set(current);
      if (next.has(entityId)) next.delete(entityId);
      else next.add(entityId);
      return next;
    });
  };

  const discover = async () => {
    setBusy(true);
    setActionResult(null);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/mesh/discover`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ sources: selectedSources }),
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

  const findCapability = async () => {
    if (!capabilityQuery.trim()) return;
    setBusy(true);
    setActionResult(null);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/mesh/find-capability`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ capability: capabilityQuery.trim() }),
      });
      const text = await res.text();
      if (!res.ok) throw new Error(`find-capability ${res.status}`);
      setActionResult(text);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const inspectRoute = async () => {
    const source = selectedId;
    const target = routeTarget.trim();
    if (!source || !target) return;
    setBusy(true);
    setActionResult(null);
    setError(null);
    try {
      const query = new URLSearchParams({ source, target });
      const res = await fetch(`${baseUrl}/v1/mesh/routes?${query.toString()}`, {
        headers: authHeaders(),
      });
      const text = await res.text();
      if (!res.ok) throw new Error(`routes ${res.status}`);
      setActionResult(text);
      try {
        const body = JSON.parse(text) as { route?: MeshRoute };
        const hops = body.route?.hops ?? [];
        setRouteHighlight(hops.map((hop) => hop.entity_id).filter((id): id is string => !!id));
      } catch {
        setRouteHighlight([]);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const simulatePartition = async () => {
    const entityIds = [...partitionSelection];
    if (entityIds.length === 0) return;
    setBusy(true);
    setActionResult(null);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/mesh/simulate-partition`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ entity_ids: entityIds }),
      });
      const text = await res.text();
      if (!res.ok) throw new Error(`simulate-partition ${res.status}`);
      setActionResult(text);
      setPartitionSelection(new Set());
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const loadMergeReport = async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/mesh/merge-report`, { headers: authHeaders() });
      const text = await res.text();
      if (!res.ok) throw new Error(`merge-report ${res.status}`);
      setMergeReport(text);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const coordinatorId = topology?.coordinator?.entity_id ?? null;
  const canSimulate = hasToken && can("mesh");

  return (
    <div className="cc-panel">
      <header className="cc-panel-header">
        <h2>Autonomous Entity Mesh</h2>
        <p className="cc-panel-subtitle">
          Trust-aware inter-entity communication — topology, reachability, routes, and partitions.
        </p>
      </header>

      {error ? <CcEmptyState title="Mesh unavailable" description={error} /> : null}

      <CcMiniStats
        items={[
          { label: "Nodes", value: String(health?.total_nodes ?? nodes.length) },
          { label: "Reachable", value: String(health?.reachable_nodes ?? "—") },
          { label: "Coordinator", value: coordinatorId ?? "—" },
          { label: "Partitions", value: String(health?.active_partitions ?? 0) },
          { label: "Components", value: String(health?.topology_components ?? 0) },
          {
            label: "Avg trust",
            value:
              health?.average_trust_score != null
                ? health.average_trust_score.toFixed(2)
                : "—",
          },
        ]}
      />

      <CcSection title="Mesh actions" hint="Discovery honors selected sources (live MQTT/ROS2 when env-gated).">
        <div className="cc-transport-grid mesh-source-grid">
          {DISCOVERY_SOURCES.map((source) => {
            const checked = selectedSources.includes(source.id);
            return (
              <label
                key={source.id}
                className={`cc-transport-card${checked ? " selected" : ""}`}
              >
                <input
                  type="checkbox"
                  checked={checked}
                  onChange={() => toggleSource(source.id)}
                />
                <span className="cc-transport-label">{source.label}</span>
                <span className="cc-transport-hint">{source.hint}</span>
              </label>
            );
          })}
        </div>
        <div className="cc-action-bar">
          <button type="button" disabled={busy} onClick={() => void load()}>
            Refresh
          </button>
          <button type="button" className="primary" disabled={busy} onClick={() => void discover()}>
            Discover
          </button>
          <button type="button" disabled={busy} onClick={() => void loadMergeReport()}>
            Merge report
          </button>
        </div>
        {actionResult ? <pre className="cc-action-result">{actionResult}</pre> : null}
        {mergeReport ? (
          <details open className="mesh-graph-raw">
            <summary>Partition merge report</summary>
            <pre className="cc-action-result">{mergeReport}</pre>
          </details>
        ) : null}
      </CcSection>

      <CcSection title="Route and capability tools">
        <div className="cc-action-bar">
          <label>
            Capability{" "}
            <input
              value={capabilityQuery}
              onChange={(event) => setCapabilityQuery(event.target.value)}
              placeholder="thermal_camera"
            />
          </label>
          <button type="button" disabled={busy || !capabilityQuery.trim()} onClick={() => void findCapability()}>
            Find capability
          </button>
        </div>
        <div className="cc-action-bar">
          <label>
            Route target{" "}
            <input
              value={routeTarget}
              onChange={(event) => setRouteTarget(event.target.value)}
              placeholder={selectedId ?? "entity-id"}
            />
          </label>
          <button
            type="button"
            disabled={busy || !selectedId || !routeTarget.trim()}
            onClick={() => void inspectRoute()}
          >
            Inspect route
          </button>
          {routeHighlight.length > 0 ? (
            <button type="button" disabled={busy} onClick={() => setRouteHighlight([])}>
              Clear route overlay
            </button>
          ) : null}
        </div>
      </CcSection>

      <CcSection title="Entity reachability">
        {nodes.length === 0 ? (
          <CcEmptyState title="No mesh nodes" description="Run Discover or load a project with entities." />
        ) : (
          <>
            <div className="cc-action-bar">
              <button
                type="button"
                className={canSimulate ? "primary" : undefined}
                disabled={busy || partitionSelection.size === 0 || !canSimulate}
                onClick={() => void simulatePartition()}
                title={canSimulate ? undefined : "Sign in with mesh simulate permission"}
              >
                Simulate partition ({partitionSelection.size})
              </button>
            </div>
            <table className="cc-table">
              <thead>
                <tr>
                  <th>Select</th>
                  <th>Entity</th>
                  <th>Transport</th>
                  <th>Reachable</th>
                  <th>Trust</th>
                  <th>Capabilities</th>
                </tr>
              </thead>
              <tbody>
                {nodes.map((node) => (
                  <tr
                    key={node.entity_id}
                    className={selectedId === node.entity_id ? "selected" : undefined}
                    onClick={() => setSelectedId(node.entity_id)}
                  >
                    <td onClick={(event) => event.stopPropagation()}>
                      <input
                        type="checkbox"
                        checked={partitionSelection.has(node.entity_id)}
                        onChange={() => togglePartitionEntity(node.entity_id)}
                      />
                    </td>
                    <td>{node.entity_id}</td>
                    <td>
                      <CcBadge tone={node.transport && node.transport !== "local_runtime" ? "info" : undefined}>
                        {node.transport ?? "local_runtime"}
                      </CcBadge>
                    </td>
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
          </>
        )}
      </CcSection>

      <CcSection
        title="Topology graph"
        hint="Filter by transport, overlay computed routes, and highlight partition members."
      >
        {meshGraph && meshGraph.nodes.length > 0 ? (
          <>
            <div className="cc-action-bar mesh-graph-view-toggle">
              <button
                type="button"
                className={graphView === "full" ? "primary" : undefined}
                disabled={busy}
                onClick={() => setGraphView("full")}
              >
                Full mesh ({meshGraph.nodes.length})
              </button>
              <button
                type="button"
                className={graphView === "neighborhood" ? "primary" : undefined}
                disabled={busy}
                onClick={() => setGraphView("neighborhood")}
              >
                Neighborhood
              </button>
              {TRANSPORT_FILTERS.map((filter) => (
                <button
                  key={filter.id}
                  type="button"
                  className={transportFilter === filter.id ? "primary" : undefined}
                  disabled={busy}
                  onClick={() => setTransportFilter(filter.id)}
                >
                  {filter.label}
                </button>
              ))}
            </div>
            <MeshMiniGraph
              graph={meshGraph}
              selectedId={selectedId}
              onSelect={setSelectedId}
              coordinatorId={coordinatorId}
              mode={graphView}
              highlightPath={routeHighlight}
              transportFilter={transportFilter}
              partitionNodeIds={partitionNodeIds}
            />
            <details className="mesh-graph-raw">
              <summary>Raw graph JSON</summary>
              <pre className="cc-action-result">{JSON.stringify(meshGraph, null, 2)}</pre>
            </details>
          </>
        ) : (
          <CcEmptyState title="No graph data" />
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
