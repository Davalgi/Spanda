import { CcMiniStats, CcSection } from "./controlCenterUi";

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
};

export function HealthPanel({ pool }: Props) {
  const active = pool.active ?? pool.healthy;
  const quarantined = pool.quarantined ?? 0;
  const healthyPercent =
    pool.total > 0 ? Math.round((active / pool.total) * 100) : 100;

  return (
    <div className="cc-panel">
      <div className="cc-health-hero">
        <span className="cc-health-percent">{healthyPercent}%</span>
        <span className="cc-health-label">devices healthy</span>
      </div>

      <CcMiniStats
        items={[
          { label: "Healthy / active", value: active, tone: "ok" },
          { label: "Degraded", value: pool.degraded, tone: pool.degraded > 0 ? "warn" : "ok" },
          { label: "Failed", value: pool.failed, tone: pool.failed > 0 ? "danger" : "ok" },
          { label: "Quarantined", value: quarantined, tone: quarantined > 0 ? "danger" : "ok" },
          { label: "Discovered", value: pool.discovered, tone: pool.discovered > 0 ? "warn" : "ok" },
          { label: "Total", value: pool.total },
        ]}
      />

      <CcSection title="Pool breakdown" hint="Rollup from the device health engine.">
        <div className="cc-health-bars">
          {[
            { label: "Healthy", value: active, tone: "ok" },
            { label: "Degraded", value: pool.degraded, tone: "warn" },
            { label: "Failed", value: pool.failed, tone: "danger" },
            { label: "Quarantined", value: quarantined, tone: "danger" },
            { label: "Discovered", value: pool.discovered, tone: "warn" },
          ].map((row) => {
            const width = pool.total > 0 ? Math.max(2, (row.value / pool.total) * 100) : 0;
            return (
              <div key={row.label} className="cc-health-bar-row">
                <span className="cc-health-bar-label">{row.label}</span>
                <div className="cc-health-bar-track">
                  <div
                    className={`cc-health-bar-fill tone-${row.tone}`}
                    style={{ width: `${width}%` }}
                  />
                </div>
                <span className="cc-health-bar-value">{row.value}</span>
              </div>
            );
          })}
        </div>
      </CcSection>
    </div>
  );
}
