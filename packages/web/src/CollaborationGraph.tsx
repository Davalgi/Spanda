type Node = { id?: string; label?: string; kind?: string };
type Edge = { from?: string; to?: string; from_id?: string; to_id?: string };

type Props = {
  graph: Record<string, unknown> | null;
};

export function CollaborationGraph({ graph }: Props) {
  if (!graph) return null;
  const nodes = ((graph.nodes as Node[]) ?? (graph.participants as Node[]) ?? []).slice(0, 16);
  const edges = ((graph.edges as Edge[]) ?? []).slice(0, 24);
  if (nodes.length === 0) {
    return <pre className="cc-action-result">{JSON.stringify(graph, null, 2)}</pre>;
  }

  const positions: Record<string, { x: number; y: number }> = {};
  nodes.forEach((node, index) => {
    const angle = (index / nodes.length) * Math.PI * 2;
    positions[String(node.id ?? index)] = {
      x: 50 + Math.cos(angle) * 35,
      y: 50 + Math.sin(angle) * 35,
    };
  });

  return (
    <svg viewBox="0 0 100 100" className="cc-collab-graph" role="img" aria-label="Collaboration graph">
      {edges.map((edge, index) => {
        const from = String(edge.from ?? edge.from_id ?? "");
        const to = String(edge.to ?? edge.to_id ?? "");
        const start = positions[from];
        const end = positions[to];
        if (!start || !end) return null;
        return (
          <line
            key={`${from}-${to}-${index}`}
            x1={start.x}
            y1={start.y}
            x2={end.x}
            y2={end.y}
            className="cc-collab-edge"
          />
        );
      })}
      {nodes.map((node, index) => {
        const id = String(node.id ?? index);
        const pos = positions[id];
        return (
          <g key={id} transform={`translate(${pos.x}, ${pos.y})`}>
            <circle r="3" className="cc-collab-node" />
            <title>{node.label ?? id}</title>
          </g>
        );
      })}
    </svg>
  );
}
