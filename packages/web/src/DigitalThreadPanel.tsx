/**
 * Control Center Digital Thread panel — lifecycle graph query and export.
 * @module
 */
import { useCallback, useEffect, useState } from "react";
import {
  DigitalThreadGraph,
  exportDigitalThreadJson,
  type DigitalThreadDeviceLink,
  type DigitalThreadGraphEdge,
  type DigitalThreadGraphNode,
  type DigitalThreadLifecycleEdge,
} from "./DigitalThreadGraph";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type Props = {
  baseUrl: string;
};

export function DigitalThreadPanel({ baseUrl }: Props) {
  const [digitalThread, setDigitalThread] = useState<Record<string, unknown> | null>(null);
  const [capabilityFilter, setCapabilityFilter] = useState("");
  const [deviceFilter, setDeviceFilter] = useState("");
  const [lifecycleFilter, setLifecycleFilter] = useState("");
  const [phasePathFilter, setPhasePathFilter] = useState("");
  const [nodeSearch, setNodeSearch] = useState("");
  const [highlightPhasePath, setHighlightPhasePath] = useState(true);
  const [selectedNode, setSelectedNode] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    // Query the digital thread API with capability, device, phase, and path filters.
    //
    // Parameters:
    // None (reads panel filter state and `baseUrl`).
    //
    // Returns:
    // Promise that resolves when the report is stored or an error is shown.
    //
    // Options:
    // None.
    //
    // Example:
    // await load();

    setBusy(true);
    setError(null);
    try {
      const params = new URLSearchParams();
      if (capabilityFilter.trim()) params.set("capability", capabilityFilter.trim());
      if (deviceFilter.trim()) params.set("device_id", deviceFilter.trim());
      if (lifecycleFilter.trim()) params.set("lifecycle_phase", lifecycleFilter.trim());
      if (phasePathFilter.trim()) params.set("phase_path", phasePathFilter.trim());
      const query = params.toString();
      const res = await fetch(
        `${baseUrl}/v1/digital-thread/query${query ? `?${query}` : ""}`,
      );
      if (!res.ok) throw new Error(`digital-thread ${res.status}`);
      const body = await res.json();
      setDigitalThread(body.digital_thread ?? body);
      setSelectedNode(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, capabilityFilter, deviceFilter, lifecycleFilter, phasePathFilter]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const lifecycleRows =
    (digitalThread?.lifecycle_rows as { node_id: string; phase: string; kind?: string }[] | undefined) ?? [];
  const lifecycleEdges =
    (digitalThread?.lifecycle_edges as DigitalThreadLifecycleEdge[] | undefined) ?? [];
  const graphNodes =
    (digitalThread?.graph as { nodes?: DigitalThreadGraphNode[] })?.nodes ?? [];
  const graphEdges =
    (digitalThread?.graph as { edges?: DigitalThreadGraphEdge[] })?.edges ?? [];
  const deviceLinks =
    (digitalThread?.device_links as DigitalThreadDeviceLink[] | undefined) ?? [];
  const chainSummary = (digitalThread?.chain_summary as string[]) ?? [];
  const phasePathNodes =
    (digitalThread?.phase_path_nodes as string[] | undefined) ??
    lifecycleRows.map((row) => row.node_id);

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <p className="cc-section-hint">
        Full requirement → retirement lifecycle graph. For capability/hardware matrices and entity
        chain summaries, open the Traceability tab.
      </p>

      <CcSection title="Query filters">
        <div className="digital-thread-filters">
          <label>
            Capability
            <input
              value={capabilityFilter}
              onChange={(event) => setCapabilityFilter(event.target.value)}
              placeholder="e.g. navigate"
            />
          </label>
          <label>
            Device id
            <input
              value={deviceFilter}
              onChange={(event) => setDeviceFilter(event.target.value)}
              placeholder="e.g. gps-001"
            />
          </label>
          <label>
            Lifecycle phase
            <select
              value={lifecycleFilter}
              onChange={(event) => setLifecycleFilter(event.target.value)}
            >
              <option value="">All phases</option>
              <option value="requirement">Requirement</option>
              <option value="design">Design</option>
              <option value="deploy">Deploy</option>
              <option value="operate">Operate</option>
              <option value="retire">Retire</option>
            </select>
          </label>
          <label>
            Phase path
            <input
              value={phasePathFilter}
              onChange={(event) => setPhasePathFilter(event.target.value)}
              placeholder="requirement->deploy"
            />
          </label>
          <label>
            Search nodes
            <input
              value={nodeSearch}
              onChange={(event) => setNodeSearch(event.target.value)}
              placeholder="Filter by id / label / kind"
            />
          </label>
          <label className="digital-thread-filters__checkbox">
            <input
              type="checkbox"
              checked={highlightPhasePath}
              onChange={(event) => setHighlightPhasePath(event.target.checked)}
            />
            Highlight phase path
          </label>
          <button type="button" onClick={() => void load()} disabled={busy}>
            Query
          </button>
          <button
            type="button"
            onClick={() => digitalThread && exportDigitalThreadJson(digitalThread)}
            disabled={!digitalThread}
          >
            Export JSON
          </button>
        </div>
      </CcSection>

      {!digitalThread && !busy ? (
        <CcEmptyState title="No digital thread data" />
      ) : digitalThread ? (
        <>
          <p className="cc-section-hint">
            {String(digitalThread.matched_node_count ?? 0)} nodes,{" "}
            {String(digitalThread.matched_edge_count ?? 0)} edges,{" "}
            {lifecycleEdges.length} lifecycle edges
            {lifecycleRows.length > 0
              ? ` — phases: ${Object.keys(
                  (digitalThread.lifecycle_summary as Record<string, number>) ?? {},
                ).join(", ")}`
              : ""}
            {deviceLinks.length > 0 ? ` — ${deviceLinks.length} device link(s)` : ""}{" "}
            — drag to pan, scroll to zoom, click a node to highlight neighbors
          </p>

          <div className="digital-thread-legend">
            <span className="legend-requirement">Requirement</span>
            <span className="legend-design">Design</span>
            <span className="legend-deploy">Deploy</span>
            <span className="legend-operate">Operate</span>
            <span className="legend-retire">Retire</span>
            <span className="legend-device">Device</span>
          </div>

          <DigitalThreadGraph
            nodes={graphNodes}
            edges={graphEdges}
            deviceLinks={deviceLinks}
            lifecycleRows={lifecycleRows}
            lifecycleEdges={lifecycleEdges}
            selectedId={selectedNode}
            onSelectNode={setSelectedNode}
            searchQuery={nodeSearch}
            highlightPhasePath={highlightPhasePath}
            phasePathNodes={phasePathNodes}
          />

          {chainSummary.length > 0 && (
            <CcSection title="Chain summary">
              <ul className="cc-chain-list">
                {chainSummary.map((step) => (
                  <li key={step}>{step}</li>
                ))}
              </ul>
            </CcSection>
          )}

          <details className="cc-json-details">
            <summary>Raw report JSON</summary>
            <pre className="cc-action-result">{JSON.stringify(digitalThread, null, 2)}</pre>
          </details>
        </>
      ) : null}
    </div>
  );
}
