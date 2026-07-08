/**
 * Layout helpers for mesh topology SVG graphs.
 * @module
 */

export type LayoutEdge = {
  from: string;
  to: string;
};

export type LayoutPoint = {
  x: number;
  y: number;
};

/**
 * Grid layout for small neighborhood views.
 */
export function gridLayout(
  nodeIds: string[],
  cols = 6,
  cellW = 96,
  cellH = 72,
  padX = 48,
  padY = 36,
): Map<string, LayoutPoint> {
  const positions = new Map<string, LayoutPoint>();
  const columnCount = Math.min(cols, Math.max(1, Math.ceil(Math.sqrt(nodeIds.length))));
  nodeIds.forEach((id, idx) => {
    positions.set(id, {
      x: padX + (idx % columnCount) * cellW,
      y: padY + Math.floor(idx / columnCount) * cellH,
    });
  });
  return positions;
}

/**
 * Force-directed layout for full mesh topology (coordinator anchored at center).
 */
export function forceDirectedLayout(
  nodeIds: string[],
  edges: LayoutEdge[],
  width: number,
  height: number,
  anchorId?: string | null,
): Map<string, LayoutPoint> {
  if (nodeIds.length === 0) {
    return new Map();
  }

  const cx = width / 2;
  const cy = height / 2;
  const positions = new Map<
    string,
    { x: number; y: number; vx: number; vy: number }
  >();

  nodeIds.forEach((id, index) => {
    const angle = (2 * Math.PI * index) / nodeIds.length;
    const radius = Math.min(width, height) * 0.34;
    positions.set(id, {
      x: cx + radius * Math.cos(angle),
      y: cy + radius * Math.sin(angle),
      vx: 0,
      vy: 0,
    });
  });

  const iterations = Math.min(120, 40 + nodeIds.length * 2);
  for (let step = 0; step < iterations; step += 1) {
    for (const [idA, posA] of positions) {
      for (const [idB, posB] of positions) {
        if (idA === idB) continue;
        let dx = posA.x - posB.x;
        let dy = posA.y - posB.y;
        const dist = Math.hypot(dx, dy) || 1;
        const repulsion = 9000 / (dist * dist);
        dx /= dist;
        dy /= dist;
        posA.vx += dx * repulsion;
        posA.vy += dy * repulsion;
      }
    }

    for (const edge of edges) {
      const posA = positions.get(edge.from);
      const posB = positions.get(edge.to);
      if (!posA || !posB) continue;
      let dx = posB.x - posA.x;
      let dy = posB.y - posA.y;
      const dist = Math.hypot(dx, dy) || 1;
      const spring = (dist - 100) * 0.06;
      dx /= dist;
      dy /= dist;
      posA.vx += dx * spring;
      posA.vy += dy * spring;
      posB.vx -= dx * spring;
      posB.vy -= dy * spring;
    }

    if (anchorId && positions.has(anchorId)) {
      const anchor = positions.get(anchorId)!;
      anchor.vx += (cx - anchor.x) * 0.12;
      anchor.vy += (cy - anchor.y) * 0.12;
    }

    for (const [id, pos] of positions) {
      if (id === anchorId) {
        pos.x = cx;
        pos.y = cy;
        pos.vx = 0;
        pos.vy = 0;
        continue;
      }
      pos.x += pos.vx * 0.04;
      pos.y += pos.vy * 0.04;
      pos.vx *= 0.82;
      pos.vy *= 0.82;
      pos.x = Math.max(28, Math.min(width - 28, pos.x));
      pos.y = Math.max(28, Math.min(height - 28, pos.y));
    }
  }

  return new Map(
    [...positions.entries()].map(([id, pos]) => [id, { x: pos.x, y: pos.y }]),
  );
}

export function layoutDimensions(nodeCount: number, mode: "neighborhood" | "full"): {
  width: number;
  height: number;
} {
  if (mode === "neighborhood") {
    const cols = Math.min(6, Math.max(1, Math.ceil(Math.sqrt(nodeCount))));
    const rows = Math.ceil(nodeCount / cols);
    return {
      width: 580,
      height: Math.max(160, 56 + rows * 72),
    };
  }
  const side = Math.max(420, Math.min(920, 180 + nodeCount * 28));
  return { width: side, height: side };
}
