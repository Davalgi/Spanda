/**
 * SVG neighborhood graph for Autonomous Entity Mesh topology.
 * @module
 */

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

type Props = {
  graph: MeshGraphPayload | null;
  selectedId: string | null;
  onSelect: (id: string) => void;
  coordinatorId?: string | null;
};

function layoutNodes(nodes: MeshGraphNode[]): Map<string, { x: number; y: number }> {
  const positions = new Map<string, { x: number; y: number }>();
  const cols = Math.min(6, Math.max(1, Math.ceil(Math.sqrt(nodes.length))));
  nodes.forEach((node, idx) => {
    positions.set(node.id, {
      x: 48 + (idx % cols) * 96,
      y: 36 + Math.floor(idx / cols) * 72,
    });
  });
  return positions;
}

export function MeshMiniGraph({ graph, selectedId, onSelect, coordinatorId }: Props) {
  if (!graph || graph.nodes.length === 0) {
    return null;
  }

  const focus = selectedId ?? graph.nodes[0]?.id;
  if (!focus) return null;

  const related = new Set<string>([focus]);
  for (const edge of graph.edges) {
    if (edge.from === focus) related.add(edge.to);
    if (edge.to === focus) related.add(edge.from);
  }

  const nodes = graph.nodes.filter((n) => related.has(n.id)).slice(0, 18);
  if (nodes.length === 0) return null;

  const nodeIds = new Set(nodes.map((n) => n.id));
  const edges = graph.edges.filter((e) => nodeIds.has(e.from) && nodeIds.has(e.to));
  const positions = layoutNodes(nodes);
  const rows = Math.ceil(nodes.length / 6);
  const height = Math.max(160, 56 + rows * 72);

  return (
    <div className="mesh-mini-graph">
      <svg viewBox={`0 0 580 ${height}`} role="img" aria-label="Mesh topology graph">
        {edges.map((edge, idx) => {
          const from = positions.get(edge.from);
          const to = positions.get(edge.to);
          if (!from || !to) return null;
          const stroke = edge.trusted === false ? "#ef4444" : edge.active === false ? "#94a3b8" : "#64748b";
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
        {nodes.map((node) => {
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
          return (
            <g key={node.id} onClick={() => onSelect(node.id)} className="mesh-graph-node">
              <circle cx={pos.x} cy={pos.y} r={selected || isCoordinator ? 14 : 10} fill={fill} />
              <text x={pos.x} y={pos.y + 24} textAnchor="middle" fontSize={9} fill="#e2e8f0">
                {node.id.slice(0, 14)}
              </text>
            </g>
          );
        })}
      </svg>
    </div>
  );
}
