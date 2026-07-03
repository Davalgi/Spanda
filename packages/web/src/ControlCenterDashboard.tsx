import { type ControlCenterTab } from "./controlCenterRbac";
import { tabLabel } from "./controlCenterNavConfig";

type Pool = {
  total: number;
  healthy: number;
  active?: number;
  degraded: number;
  discovered: number;
  quarantined?: number;
  failed: number;
};

type Props = {
  pool: Pool;
  fleetAgentCount: number;
  alertCount: number;
  missionReady?: boolean;
  onNavigate: (tab: ControlCenterTab) => void;
};

type StatCard = {
  label: string;
  value: number | string;
  hint?: string;
  tone?: "ok" | "warn" | "danger" | "neutral";
  tab?: ControlCenterTab;
};

export function ControlCenterDashboard({
  pool,
  fleetAgentCount,
  alertCount,
  missionReady,
  onNavigate,
}: Props) {
  const healthy = pool.active ?? pool.healthy;
  const quarantined = pool.quarantined ?? 0;

  const cards: StatCard[] = [
    {
      label: "Devices",
      value: pool.total,
      hint: `${healthy} healthy`,
      tone: pool.failed > 0 ? "warn" : "ok",
      tab: "devices",
    },
    {
      label: "Discovered",
      value: pool.discovered,
      hint: "Awaiting provision",
      tone: pool.discovered > 0 ? "warn" : "neutral",
      tab: "discovery",
    },
    {
      label: "Quarantined",
      value: quarantined,
      hint: quarantined > 0 ? "Needs review" : "None",
      tone: quarantined > 0 ? "danger" : "ok",
      tab: "devices",
    },
    {
      label: "Fleet agents",
      value: fleetAgentCount,
      hint: "Connected robots",
      tone: "neutral",
      tab: "fleet",
    },
    {
      label: "Open alerts",
      value: alertCount,
      hint: alertCount > 0 ? "Review incidents" : "All clear",
      tone: alertCount > 0 ? "danger" : "ok",
      tab: "alerts",
    },
    {
      label: "Mission readiness",
      value: missionReady === undefined ? "—" : missionReady ? "Ready" : "Blocked",
      hint: missionReady === false ? "Check readiness tab" : undefined,
      tone:
        missionReady === undefined ? "neutral" : missionReady ? "ok" : "danger",
      tab: "readiness",
    },
  ];

  const quickLinks: { tab: ControlCenterTab; reason: string }[] = [
    { tab: "fleet-map", reason: "Live fleet positions" },
    { tab: "telemetry", reason: "WebSocket signal stream" },
    { tab: "trends", reason: "Readiness forecast" },
    { tab: "continuity", reason: "Takeover and handoff" },
    { tab: "reports", reason: "Scheduled exports" },
    { tab: "recovery", reason: "Playbooks and rollback" },
  ];

  return (
    <div className="cc-dashboard">
      <div className="cc-stat-grid">
        {cards.map((card) => (
          <button
            key={card.label}
            type="button"
            className={`cc-stat-card${card.tone ? ` tone-${card.tone}` : ""}`}
            onClick={() => card.tab && onNavigate(card.tab)}
            disabled={!card.tab}
          >
            <span className="cc-stat-label">{card.label}</span>
            <span className="cc-stat-value">{card.value}</span>
            {card.hint && <span className="cc-stat-hint">{card.hint}</span>}
          </button>
        ))}
      </div>

      <section className="cc-quick-links">
        <h3>Quick actions</h3>
        <div className="cc-quick-link-grid">
          {quickLinks.map((link) => (
            <button
              key={link.tab}
              type="button"
              className="cc-quick-link"
              onClick={() => onNavigate(link.tab)}
            >
              <span className="cc-quick-link-title">{tabLabel(link.tab)}</span>
              <span className="cc-quick-link-reason">{link.reason}</span>
            </button>
          ))}
        </div>
      </section>
    </div>
  );
}
