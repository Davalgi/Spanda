import { CcSection } from "./controlCenterUi";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
};

type PlaceholderSection = {
  title: string;
  hint: string;
  endpoint: string;
};

const SECTIONS: PlaceholderSection[] = [
  {
    title: "Reflex Events",
    hint: "Immediate local safety responses from distributed decision layer 0.",
    endpoint: "/v1/autonomy/reflex",
  },
  {
    title: "Attention Queue",
    hint: "Prioritized events — critical safety above routine telemetry.",
    endpoint: "/v1/autonomy/attention",
  },
  {
    title: "Homeostasis Status",
    hint: "Stability metrics before failures — CPU, memory, battery, latency.",
    endpoint: "/v1/autonomy/homeostasis",
  },
  {
    title: "Immunity / Quarantine",
    hint: "Untrusted, tampered, or compromised entity isolation.",
    endpoint: "/v1/autonomy/immunity",
  },
  {
    title: "Operational Memory",
    hint: "Reflex, working, episodic, semantic, and procedural memory refs.",
    endpoint: "/v1/entities/{id}/autonomy",
  },
  {
    title: "Recovery Confidence",
    hint: "Rule-based strategy preference from historical outcomes.",
    endpoint: "/v1/recovery/metrics",
  },
  {
    title: "Damage Risk",
    hint: "Harm potential index — overheating, vibration, operator fatigue.",
    endpoint: "/v1/entities/{id}/autonomy",
  },
];

export function ResilientAutonomyPanel({ baseUrl, authHeaders }: Props) {
  return (
    <div className="cc-panel">
      <header className="cc-panel-header">
        <h2>Resilient Autonomy</h2>
        <p className="cc-panel-subtitle">
          Bio-inspired resilience architecture — reflex, attention, homeostasis, immunity, memory,
          recovery confidence, and damage risk. Preview panels; data from REST stubs.
        </p>
      </header>

      {SECTIONS.map((section) => (
        <CcSection key={section.title} title={section.title} hint={section.hint}>
          <div className="cc-placeholder-card">
            <code>
              GET {baseUrl}
              {section.endpoint}
            </code>
            <p className="cc-muted">
              Placeholder — wire to live telemetry and entity autonomy profiles. Use{" "}
              <code>spanda reflex list</code>, <code>spanda homeostasis report</code>,{" "}
              <code>spanda immunity scan</code> for CLI reports today.
            </p>
          </div>
        </CcSection>
      ))}
    </div>
  );
}
