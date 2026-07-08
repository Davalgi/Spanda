/**
 * SVG mesh topology graph — neighborhood or full force-directed layout.
 * @module
 */
import { useMemo } from "react";
import {
  forceDirectedLayout,
  gridLayout,
  layoutDimensions,
  type LayoutEdge,
} from "./meshGraphLayout";

export type MeshGraphNode = {
  id: string;
  transport?: string;
  reachable?: boolean;
  trust_score?: number;
  role?: string;
};

export type MeshGraphEdge = {
  from: string;
  to: string;
  trusted?: boolean;
  active?: boolean;
};

export type MeshGraphPayload = {
  nodes: MeshGraphNode[];
  edges: MeshGraphEdge[];
};

export type MeshGraphViewMode = "neighborhood" | "full";

type Props = {
  graph: MeshGraphPayload | null;
  selectedId: string | null;
  onSelect: (id: string) => void;
  coordinatorId?: string | null;
  mode?: MeshGraphViewMode;
};

function neighborhoodNodes(graph: MeshGraphPayload, focus: string): MeshGraphNode[] {
  const related = new Set<string>([focus]);
  for (const edge of graph.edges) {
    if (edge.from === focus) related.add(edge.to);
    if (edge.to === focus) related.add(edge.from);
  }
  return graph.nodes.filter((node) => related.has(node.id)).slice(0, 18);
}

export function MeshMiniGraph({
  graph,
  selectedId,
  onSelect,
  coordinatorId,
  mode = "neighborhood",
}: Props) {
  const layout = useMemo(() => {
    if (!graph || graph.nodes.length === 0) {
      return null;
    }

    const focus = selectedId ?? graph.nodes[0]?.id;
    if (!focus) return null;

    const visibleNodes =
      mode === "full" ? graph.nodes : neighborhoodNodes(graph, focus);
    if (visibleNodes.length === 0) return null;

    const nodeIds = new Set(visibleNodes.map((node) => node.id));
    const edges = graph.edges.filter(
      (edge) => nodeIds.has(edge.from) && nodeIds.has(edge.to),
    );
    const { width, height } = layoutDimensions(visibleNodes.length, mode);
    const layoutEdges: LayoutEdge[] = edges.map((edge) => ({
      from: edge.from,
      to: edge.to,
    }));
    const positions =
      mode === "full"
        ? forceDirectedLayout(
            visibleNodes.map((node) => node.id),
            layoutEdges,
            width,
            height,
            coordinatorId,
          )
        : gridLayout(visibleNodes.map((node) => node.id));

    return { visibleNodes, edges, positions, width, height, focus };
  }, [graph, selectedId, coordinatorId, mode]);

  if (!layout) {
    return null;
  }

  const { visibleNodes, edges, positions, width, height, focus } = layout;

  return (
    <div className="mesh-mini-graph">
      <div className="mesh-graph-legend" aria-hidden="true">
        <span className="mesh-legend-item">
          <span className="mesh-legend-dot mesh-legend-coordinator" /> Coordinator
        </span>
        <span className="mesh-legend-item">
          <span className="mesh-legend-dot mesh-legend-selected" /> Selected
        </span>
        <span className="mesh-legend-item">
          <span className="mesh-legend-dot mesh-legend-offline" /> Offline
        </span>
        <span className="mesh-legend-item">
          <span className="mesh-legend-line mesh-legend-untrusted" /> Untrusted link
        </span>
      </div>
      <svg
        viewBox={`0 0 ${width} ${height}`}
        role="img"
        aria-label={mode === "full" ? "Full mesh topology graph" : "Mesh neighborhood graph"}
      >
        {edges.map((edge, idx) => {
          const from = positions.get(edge.from);
          const to = positions.get(edge.to);
          if (!from || !to) return null;
          const stroke =
            edge.trusted === false ? "#ef4444" : edge.active === false ? "#94a3b8" : "#64748b";
          return (
            <line
              key={`${edge.from}-${edge.to}-${idx}`}
              x1={from.x}
              y1={from.y}
              x2={to.x}
              y2={to.y}
              stroke={stroke}
              strokeWidth={edge.active === false ? 1 : 1.5}
              strokeDasharray={edge.active === false ? "4 3" : undefined}
            />
          );
        })}
        {visibleNodes.map((node) => {
          const pos = positions.get(node.id);
          if (!pos) return null;
          const selected = node.id === focus;
          const isCoordinator = coordinatorId != null && node.id === coordinatorId;
          const fill = !node.reachable
            ? "#475569"
            : isCoordinator
              ? "#a855f7"
              : selected
                ? "#6366f1"
                : "#334155";
          const title = [
            node.id,
            node.transport ? `transport: ${node.transport}` : null,
            node.trust_score != null ? `trust: ${node.trust_score.toFixed(2)}` : null,
            node.role ? `role: ${node.role}` : null,
          ]
            .filter(Boolean)
            .join("\n");
          return (
            <g key={node.id} onClick={() => onSelect(node.id)} className="mesh-graph-node">
              <title>{title}</title>
              <circle
                cx={pos.x}
                cy={pos.y}
                r={selected || isCoordinator ? 14 : 10}
                fill={fill}
              />
              <text x={pos.x} y={pos.y + 24} textAnchor="middle" fontSize={9} fill="#e2e8f0">
                {node.id.slice(0, mode === "full" ? 10 : 14)}
              </text>
            </g>
          );
        })}
      </svg>
    </div>
  );
}
