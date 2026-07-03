export type DecisionTraceFrame = {
  sim_time_ms?: number;
  event?: string;
  payload?: Record<string, unknown>;
};

export function decisionLayerClass(layer: string): string {
  const normalized = layer.toLowerCase();
  if (normalized.includes("reflex")) return "decision-layer reflex";
  if (normalized.includes("fleet") || normalized.includes("group")) {
    return "decision-layer fleet";
  }
  if (normalized.includes("control")) return "decision-layer control";
  return "decision-layer local";
}

export function decisionEventClass(event: string): string {
  if (event.includes("blocked")) return "decision-event blocked";
  if (event.includes("escalation")) return "decision-event escalation";
  return "decision-event";
}

export function decisionRowClass(event: string): string {
  if (event.includes("blocked")) return "decision-row-blocked";
  if (event.includes("escalation")) return "decision-row-escalation";
  return "";
}
