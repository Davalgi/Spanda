/**
 * Interactive SVG graph for digital thread capability-to-device traces.
 * Supports pan/zoom, search filtering, lifecycle phase-path highlight, device
 * overlay nodes, and JSON/SVG export.
 * @module
 */
import { useMemo, useRef, useState, type PointerEvent as ReactPointerEvent, type WheelEvent as ReactWheelEvent } from "react";

export type DigitalThreadGraphNode = {
  id: string;
  label: string;
  kind: string;
};

export type DigitalThreadGraphEdge = {
  from: string;
  to: string;
  relation: string;
};

export type DigitalThreadDeviceLink = {
  device_id: string;
  device_type: string;
  assigned_robot?: string | null;
  lifecycle_state?: string | null;
  related_capabilities: string[];
};

export type DigitalThreadLifecycleRow = {
  node_id: string;
  phase: string;
  kind?: string;
  label?: string;
};

export type DigitalThreadLifecycleEdge = {
  from: string;
  to: string;
  phase_from: string;
  phase_to: string;
  relation: string;
};

type Props = {
  nodes: DigitalThreadGraphNode[];
  edges: DigitalThreadGraphEdge[];
  deviceLinks?: DigitalThreadDeviceLink[];
  lifecycleRows?: DigitalThreadLifecycleRow[];
  lifecycleEdges?: DigitalThreadLifecycleEdge[];
  selectedId?: string | null;
  onSelectNode?: (id: string | null) => void;
  searchQuery?: string;
  highlightPhasePath?: boolean;
  phasePathNodes?: string[];
};

const LIFECYCLE_COLUMN: Record<string, number> = {
  requirement: 0,
  design: 1,
  deploy: 2,
  operate: 3,
  retire: 4,
};

const LIFECYCLE_COLORS: Record<string, string> = {
  requirement: "#a78bfa",
  design: "#6366f1",
  deploy: "#3b82f6",
  operate: "#22c55e",
  retire: "#94a3b8",
};

const KIND_COLUMN: Record<string, number> = {
  mission: 0,
  robot: 1,
  capability: 2,
  hardware: 3,
  device: 3,
  sensor: 3,
  actuator: 3,
  provider: 4,
  package: 5,
  safety: 6,
};

const KIND_COLORS: Record<string, string> = {
  mission: "#6366f1",
  robot: "#3b82f6",
  capability: "#eab308",
  hardware: "#94a3b8",
  device: "#14b8a6",
  sensor: "#22d3ee",
  actuator: "#f97316",
  provider: "#22c55e",
  package: "#a3e635",
  safety: "#f87171",
};

const NODE_W = 128;
const NODE_H = 36;
const COL_GAP = 168;
const ROW_GAP = 52;
const PAD = 28;
const MIN_ZOOM = 0.4;
const MAX_ZOOM = 2.5;

function deviceNodeId(deviceId: string) {
  return `device:${deviceId}`.toLowerCase();
}

function mergeDeviceNodes(
  nodes: DigitalThreadGraphNode[],
  deviceLinks: DigitalThreadDeviceLink[],
  lifecycleRows: DigitalThreadLifecycleRow[],
): DigitalThreadGraphNode[] {
  // Start from program graph nodes and overlay configured devices.
  const byId = new Map(nodes.map((node) => [node.id, { ...node }]));
  const phaseKind = new Map(
    lifecycleRows.map((row) => [row.node_id, row.kind ?? "device"]),
  );

  // Prefer lifecycle kind labels (including device) when present.
  for (const node of byId.values()) {
    const kind = phaseKind.get(node.id);
    if (kind) {
      node.kind = kind;
    }
  }

  for (const link of deviceLinks) {
    const id = deviceNodeId(link.device_id);
    if (byId.has(id)) {
      const existing = byId.get(id)!;
      existing.kind = "device";
      existing.label = link.device_id;
      continue;
    }
    byId.set(id, {
      id,
      label: link.device_id,
      kind: "device",
    });
  }
  return [...byId.values()];
}

function mergeDeviceEdges(
  edges: DigitalThreadGraphEdge[],
  deviceLinks: DigitalThreadDeviceLink[],
  nodeIds: Set<string>,
): DigitalThreadGraphEdge[] {
  // Keep dependency edges and add device overlay links for visible nodes.
  const merged = [...edges];
  const seen = new Set(edges.map((edge) => `${edge.from}|${edge.to}|${edge.relation}`));
  for (const link of deviceLinks) {
    const to = deviceNodeId(link.device_id);
    if (!nodeIds.has(to)) {
      continue;
    }
    const candidates: string[] = [];
    if (link.assigned_robot) {
      candidates.push(`robot:${link.assigned_robot}`.toLowerCase());
    }
    for (const capability of link.related_capabilities) {
      candidates.push(`capability:${capability}`.toLowerCase());
    }
    for (const from of candidates) {
      if (!nodeIds.has(from)) {
        continue;
      }
      const key = `${from}|${to}|device_overlay`;
      if (seen.has(key)) {
        continue;
      }
      seen.add(key);
      merged.push({ from, to, relation: "device_overlay" });
    }
  }
  return merged;
}

function layoutGraph(
  nodes: DigitalThreadGraphNode[],
  lifecycleRows: DigitalThreadLifecycleRow[],
) {
  const phaseByNode = new Map(lifecycleRows.map((row) => [row.node_id, row.phase]));
  const useLifecycle = lifecycleRows.length > 0;
  const columnRows = new Map<number, number>();
  const positions = new Map<string, { x: number; y: number }>();
  const sorted = [...nodes].sort((left, right) => {
    const leftCol = useLifecycle
      ? (LIFECYCLE_COLUMN[phaseByNode.get(left.id) ?? ""] ?? 2)
      : (KIND_COLUMN[left.kind] ?? 3);
    const rightCol = useLifecycle
      ? (LIFECYCLE_COLUMN[phaseByNode.get(right.id) ?? ""] ?? 2)
      : (KIND_COLUMN[right.kind] ?? 3);
    return leftCol - rightCol || left.label.localeCompare(right.label);
  });
  for (const node of sorted) {
    const column = useLifecycle
      ? (LIFECYCLE_COLUMN[phaseByNode.get(node.id) ?? ""] ?? 2)
      : (KIND_COLUMN[node.kind] ?? 3);
    const row = columnRows.get(column) ?? 0;
    columnRows.set(column, row + 1);
    positions.set(node.id, {
      x: PAD + column * COL_GAP,
      y: PAD + row * ROW_GAP,
    });
  }
  let maxRow = 0;
  for (const row of columnRows.values()) {
    maxRow = Math.max(maxRow, row);
  }
  const width = PAD * 2 + (useLifecycle ? 5 : 6) * COL_GAP + NODE_W;
  const height = PAD * 2 + Math.max(maxRow, 1) * ROW_GAP + NODE_H;
  return { positions, width, height, phaseByNode, useLifecycle };
}

function shortLabel(label: string) {
  return label.length > 16 ? `${label.slice(0, 14)}…` : label;
}

function matchesSearch(node: DigitalThreadGraphNode, query: string) {
  if (!query.trim()) {
    return true;
  }
  const needle = query.trim().toLowerCase();
  return (
    node.id.toLowerCase().includes(needle) ||
    node.label.toLowerCase().includes(needle) ||
    node.kind.toLowerCase().includes(needle)
  );
}

/**
 * Download a UTF-8 text blob as a file in the browser.
 */
function downloadText(filename: string, contents: string, mime: string) {
  // Create a temporary object URL and trigger a download click.
  //
  // Parameters:
  // - `filename` — suggested download name
  // - `contents` — file body
  // - `mime` — MIME type for the blob
  //
  // Returns:
  // Nothing.
  //
  // Options:
  // None.
  //
  // Example:
  // downloadText("thread.json", "{}", "application/json");

  const blob = new Blob([contents], { type: mime });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

export function exportDigitalThreadJson(payload: unknown) {
  // Serialize the digital thread report (or graph slice) as a JSON download.
  //
  // Parameters:
  // - `payload` — JSON-serializable report or graph object
  //
  // Returns:
  // Nothing; triggers a browser download.
  //
  // Options:
  // None.
  //
  // Example:
  // exportDigitalThreadJson(report);

  downloadText(
    "digital-thread.json",
    JSON.stringify(payload, null, 2),
    "application/json",
  );
}

export function exportDigitalThreadSvg(svg: SVGSVGElement | null) {
  // Serialize the live SVG graph markup as a downloadable file.
  //
  // Parameters:
  // - `svg` — root SVG element rendered by the graph
  //
  // Returns:
  // Nothing; triggers a browser download when `svg` is present.
  //
  // Options:
  // None.
  //
  // Example:
  // exportDigitalThreadSvg(svgRef.current);

  if (!svg) {
    return;
  }
  const clone = svg.cloneNode(true) as SVGSVGElement;
  clone.setAttribute("xmlns", "http://www.w3.org/2000/svg");
  downloadText(
    "digital-thread.svg",
    `<?xml version="1.0" encoding="UTF-8"?>\n${clone.outerHTML}`,
    "image/svg+xml",
  );
}

export function DigitalThreadGraph({
  nodes,
  edges,
  deviceLinks = [],
  lifecycleRows = [],
  lifecycleEdges = [],
  selectedId: selectedIdProp,
  onSelectNode,
  searchQuery = "",
  highlightPhasePath = false,
  phasePathNodes = [],
}: Props) {
  const [internalSelected, setInternalSelected] = useState<string | null>(null);
  const selectedId = selectedIdProp ?? internalSelected;
  const [zoom, setZoom] = useState(1);
  const [pan, setPan] = useState({ x: 0, y: 0 });
  const dragRef = useRef<{
    pointerId: number;
    startX: number;
    startY: number;
    originX: number;
    originY: number;
  } | null>(null);
  const svgRef = useRef<SVGSVGElement | null>(null);

  const mergedNodes = useMemo(
    () => mergeDeviceNodes(nodes, deviceLinks, lifecycleRows),
    [nodes, deviceLinks, lifecycleRows],
  );

  const visibleNodes = useMemo(
    () => mergedNodes.filter((node) => matchesSearch(node, searchQuery)),
    [mergedNodes, searchQuery],
  );

  const visibleIds = useMemo(
    () => new Set(visibleNodes.map((node) => node.id)),
    [visibleNodes],
  );

  const mergedEdges = useMemo(
    () => mergeDeviceEdges(edges, deviceLinks, visibleIds),
    [edges, deviceLinks, visibleIds],
  );

  const visibleEdges = useMemo(
    () =>
      mergedEdges.filter(
        (edge) => visibleIds.has(edge.from) && visibleIds.has(edge.to),
      ),
    [mergedEdges, visibleIds],
  );

  const visibleLifecycleEdges = useMemo(
    () =>
      lifecycleEdges.filter(
        (edge) => visibleIds.has(edge.from) && visibleIds.has(edge.to),
      ),
    [lifecycleEdges, visibleIds],
  );

  const { positions, width, height, phaseByNode, useLifecycle } = useMemo(
    () => layoutGraph(visibleNodes, lifecycleRows),
    [visibleNodes, lifecycleRows],
  );

  const phasePathSet = useMemo(() => new Set(phasePathNodes), [phasePathNodes]);

  const neighborIds = useMemo(() => {
    if (!selectedId) {
      return new Set<string>();
    }
    const linked = new Set<string>([selectedId]);
    for (const edge of visibleEdges) {
      if (edge.from === selectedId) {
        linked.add(edge.to);
      }
      if (edge.to === selectedId) {
        linked.add(edge.from);
      }
    }
    for (const edge of visibleLifecycleEdges) {
      if (edge.from === selectedId) {
        linked.add(edge.to);
      }
      if (edge.to === selectedId) {
        linked.add(edge.from);
      }
    }
    return linked;
  }, [visibleEdges, visibleLifecycleEdges, selectedId]);

  const selectNode = (id: string | null) => {
    if (onSelectNode) {
      onSelectNode(id);
    } else {
      setInternalSelected(id);
    }
  };

  const selectedNode = visibleNodes.find((node) => node.id === selectedId);
  const selectedDevice = deviceLinks.find(
    (link) => deviceNodeId(link.device_id) === selectedId || link.device_id === selectedId,
  );

  const onWheel = (event: ReactWheelEvent<HTMLDivElement>) => {
    // Zoom toward the pointer while keeping pan stable enough for exploration.
    event.preventDefault();
    const delta = event.deltaY > 0 ? 0.9 : 1.1;
    setZoom((current) => Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, current * delta)));
  };

  const onPointerDown = (event: ReactPointerEvent<HTMLDivElement>) => {
    // Start a pan drag when the user presses on empty canvas space.
    if (event.button !== 0) {
      return;
    }
    const target = event.target as Element;
    if (target.closest("[data-node='true']")) {
      return;
    }
    dragRef.current = {
      pointerId: event.pointerId,
      startX: event.clientX,
      startY: event.clientY,
      originX: pan.x,
      originY: pan.y,
    };
    event.currentTarget.setPointerCapture(event.pointerId);
  };

  const onPointerMove = (event: ReactPointerEvent<HTMLDivElement>) => {
    // Update pan offsets while a drag is active.
    const drag = dragRef.current;
    if (!drag || drag.pointerId !== event.pointerId) {
      return;
    }
    setPan({
      x: drag.originX + (event.clientX - drag.startX),
      y: drag.originY + (event.clientY - drag.startY),
    });
  };

  const onPointerUp = (event: ReactPointerEvent<HTMLDivElement>) => {
    // End the active pan drag.
    if (dragRef.current?.pointerId === event.pointerId) {
      dragRef.current = null;
    }
  };

  if (mergedNodes.length === 0) {
    return <p className="demo-hint">No graph nodes — load a program with <code>--program</code>.</p>;
  }

  if (visibleNodes.length === 0) {
    return <p className="demo-hint">No nodes match the current search filter.</p>;
  }

  return (
    <div className="digital-thread-graph">
      <div className="digital-thread-graph__toolbar">
        <button type="button" onClick={() => setZoom((z) => Math.min(MAX_ZOOM, z * 1.15))}>
          Zoom in
        </button>
        <button type="button" onClick={() => setZoom((z) => Math.max(MIN_ZOOM, z / 1.15))}>
          Zoom out
        </button>
        <button
          type="button"
          onClick={() => {
            setZoom(1);
            setPan({ x: 0, y: 0 });
          }}
        >
          Reset view
        </button>
        <span className="digital-thread-graph__zoom-label">{Math.round(zoom * 100)}%</span>
        <button type="button" onClick={() => exportDigitalThreadSvg(svgRef.current)}>
          Export SVG
        </button>
      </div>
      <div
        className="digital-thread-graph__canvas"
        style={{ maxHeight: 480 }}
        onWheel={onWheel}
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerCancel={onPointerUp}
      >
        <div
          className="digital-thread-graph__viewport"
          style={{
            transform: `translate(${pan.x}px, ${pan.y}px) scale(${zoom})`,
            transformOrigin: "0 0",
            width,
            height,
          }}
        >
          <svg
            ref={svgRef}
            width={width}
            height={height}
            viewBox={`0 0 ${width} ${height}`}
            role="img"
            aria-label="Digital thread lifecycle graph"
          >
            {highlightPhasePath &&
              visibleLifecycleEdges.map((edge) => {
                const from = positions.get(edge.from);
                const to = positions.get(edge.to);
                if (!from || !to) {
                  return null;
                }
                const x1 = from.x + NODE_W;
                const y1 = from.y + NODE_H / 2;
                const x2 = to.x;
                const y2 = to.y + NODE_H / 2;
                const midX = (x1 + x2) / 2;
                return (
                  <path
                    key={`life-${edge.from}-${edge.to}-${edge.relation}`}
                    d={`M ${x1} ${y1} C ${midX} ${y1}, ${midX} ${y2}, ${x2} ${y2}`}
                    fill="none"
                    stroke="#f59e0b"
                    strokeWidth={3}
                    opacity={0.95}
                    strokeDasharray="6 4"
                  />
                );
              })}
            {visibleEdges.map((edge) => {
              const from = positions.get(edge.from);
              const to = positions.get(edge.to);
              if (!from || !to) {
                return null;
              }
              const x1 = from.x + NODE_W;
              const y1 = from.y + NODE_H / 2;
              const x2 = to.x;
              const y2 = to.y + NODE_H / 2;
              const midX = (x1 + x2) / 2;
              const onPhasePath =
                highlightPhasePath &&
                phasePathSet.size > 0 &&
                phasePathSet.has(edge.from) &&
                phasePathSet.has(edge.to);
              const active =
                !selectedId || (neighborIds.has(edge.from) && neighborIds.has(edge.to));
              return (
                <path
                  key={`${edge.from}-${edge.to}-${edge.relation}`}
                  d={`M ${x1} ${y1} C ${midX} ${y1}, ${midX} ${y2}, ${x2} ${y2}`}
                  fill="none"
                  stroke={onPhasePath ? "#f59e0b" : active ? "#58a6ff" : "#30363d"}
                  strokeWidth={onPhasePath ? 2.5 : active ? 2 : 1}
                  opacity={active || onPhasePath ? 0.9 : 0.35}
                />
              );
            })}
            {visibleNodes.map((node) => {
              const pos = positions.get(node.id);
              if (!pos) {
                return null;
              }
              const fill = useLifecycle
                ? (LIFECYCLE_COLORS[phaseByNode.get(node.id) ?? ""] ?? "#64748b")
                : (KIND_COLORS[node.kind] ?? "#64748b");
              const isSelected = node.id === selectedId;
              const onPath = highlightPhasePath && phasePathSet.has(node.id);
              const dimmed =
                selectedId !== null
                  ? !neighborIds.has(node.id)
                  : highlightPhasePath && phasePathSet.size > 0
                    ? !onPath
                    : false;
              return (
                <g
                  key={node.id}
                  data-node="true"
                  transform={`translate(${pos.x}, ${pos.y})`}
                  style={{ cursor: "pointer", opacity: dimmed ? 0.3 : 1 }}
                  onClick={() => selectNode(isSelected ? null : node.id)}
                >
                  <rect
                    width={NODE_W}
                    height={NODE_H}
                    rx={6}
                    fill={fill}
                    stroke={isSelected || onPath ? "#f0f6fc" : "#0f1419"}
                    strokeWidth={isSelected || onPath ? 2.5 : 1}
                  />
                  <text
                    x={NODE_W / 2}
                    y={NODE_H / 2 + 4}
                    textAnchor="middle"
                    fontSize={11}
                    fill="#0f1419"
                    fontFamily="system-ui, sans-serif"
                  >
                    {shortLabel(node.label)}
                  </text>
                  <title>{`${useLifecycle ? phaseByNode.get(node.id) ?? node.kind : node.kind}: ${node.label} (${node.id})`}</title>
                </g>
              );
            })}
          </svg>
        </div>
      </div>
      {selectedNode && (
        <dl className="digital-thread-graph__detail">
          <dt>Selected</dt>
          <dd>
            {useLifecycle ? phaseByNode.get(selectedNode.id) ?? selectedNode.kind : selectedNode.kind} — {selectedNode.label}
          </dd>
          <dt>Id</dt>
          <dd>{selectedNode.id}</dd>
        </dl>
      )}
      {selectedDevice && (
        <dl className="digital-thread-graph__detail">
          <dt>Device</dt>
          <dd>{selectedDevice.device_type}</dd>
          <dt>Robot</dt>
          <dd>{selectedDevice.assigned_robot ?? "—"}</dd>
          <dt>Lifecycle</dt>
          <dd>{selectedDevice.lifecycle_state ?? "—"}</dd>
          <dt>Capabilities</dt>
          <dd>{selectedDevice.related_capabilities.join(", ") || "—"}</dd>
        </dl>
      )}
    </div>
  );
}
