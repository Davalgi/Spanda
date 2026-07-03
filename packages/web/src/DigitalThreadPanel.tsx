import { useCallback, useEffect, useState } from "react";
import {
  DigitalThreadGraph,
  type DigitalThreadDeviceLink,
  type DigitalThreadGraphEdge,
  type DigitalThreadGraphNode,
} from "./DigitalThreadGraph";
import { CcEmptyState, CcSection } from "./controlCenterUi";

type Props = {
  baseUrl: string;
};

export function DigitalThreadPanel({ baseUrl }: Props) {
  const [digitalThread, setDigitalThread] = useState<Record<string, unknown> | null>(null);
  const [capabilityFilter, setCapabilityFilter] = useState("");
  const [deviceFilter, setDeviceFilter] = useState("");
  const [lifecycleFilter, setLifecycleFilter] = useState("");
  const [selectedNode, setSelectedNode] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const params = new URLSearchParams();
      if (capabilityFilter.trim()) params.set("capability", capabilityFilter.trim());
      if (deviceFilter.trim()) params.set("device_id", deviceFilter.trim());
      if (lifecycleFilter.trim()) params.set("lifecycle_phase", lifecycleFilter.trim());
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
  }, [baseUrl, capabilityFilter, deviceFilter, lifecycleFilter]);

  useEffect(() => {
    void load();
  }, [load]);

  const lifecycleRows =
    (digitalThread?.lifecycle_rows as { node_id: string; phase: string }[] | undefined) ?? [];
  const graphNodes =
    (digitalThread?.graph as { nodes?: DigitalThreadGraphNode[] })?.nodes ?? [];
  const graphEdges =
    (digitalThread?.graph as { edges?: DigitalThreadGraphEdge[] })?.edges ?? [];
  const deviceLinks =
    (digitalThread?.device_links as DigitalThreadDeviceLink[] | undefined) ?? [];
  const chainSummary = (digitalThread?.chain_summary as string[]) ?? [];

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

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
          <button type="button" onClick={() => void load()} disabled={busy}>
            Query
          </button>
        </div>
      </CcSection>

      {!digitalThread && !busy ? (
        <CcEmptyState title="No digital thread data" />
      ) : digitalThread ? (
        <>
          <p className="cc-section-hint">
            {String(digitalThread.matched_node_count ?? 0)} nodes,{" "}
            {String(digitalThread.matched_edge_count ?? 0)} edges
            {lifecycleRows.length > 0
              ? ` — lifecycle phases: ${Object.keys(
                  (digitalThread.lifecycle_summary as Record<string, number>) ?? {},
                ).join(", ")}`
              : ""}{" "}
            — click a node to highlight neighbors
          </p>

          <div className="digital-thread-legend">
            <span className="legend-mission">Mission</span>
            <span className="legend-robot">Robot</span>
            <span className="legend-capability">Capability</span>
            <span className="legend-hardware">Hardware</span>
            <span className="legend-provider">Provider</span>
            <span className="legend-package">Package</span>
            <span className="legend-safety">Safety</span>
          </div>

          <DigitalThreadGraph
            nodes={graphNodes}
            edges={graphEdges}
            deviceLinks={deviceLinks}
            lifecycleRows={lifecycleRows}
            selectedId={selectedNode}
            onSelectNode={setSelectedNode}
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
