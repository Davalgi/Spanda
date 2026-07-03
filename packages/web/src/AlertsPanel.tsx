import { useCallback, useEffect, useMemo, useState } from "react";
import { CcBadge, CcEmptyState, CcMiniStats, CcSection, formatTimestamp, severityTone } from "./controlCenterUi";

export type AlertRow = {
  id?: string;
  severity?: string;
  message?: string;
  source?: string;
  alert_type?: string;
  timestamp_ms?: number;
};

type Props = {
  baseUrl: string;
  /** Legacy: pass alerts directly when embedding without fetch. */
  alerts?: AlertRow[];
  loading?: boolean;
};

const SEVERITY_ORDER = ["critical", "high", "medium", "warning", "low", "info"];

export function AlertsPanel({ baseUrl, alerts: alertsProp, loading: loadingProp }: Props) {
  const [fetchedAlerts, setFetchedAlerts] = useState<AlertRow[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    if (alertsProp !== undefined) return;
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/alerts`);
      if (!res.ok) throw new Error(`alerts ${res.status}`);
      const body = await res.json();
      setFetchedAlerts(body.alerts ?? []);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [alertsProp, baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  const alerts = alertsProp ?? fetchedAlerts;
  const isLoading = loadingProp ?? loading;
  const [severityFilter, setSeverityFilter] = useState("all");

  const severityOptions = useMemo(() => {
    const levels = new Set(
      alerts.map((alert) => String(alert.severity ?? "unknown").toLowerCase()),
    );
    return ["all", ...Array.from(levels).sort()];
  }, [alerts]);

  const sorted = useMemo(() => {
    return [...alerts].sort((left, right) => {
      const leftSeverity = String(left.severity ?? "").toLowerCase();
      const rightSeverity = String(right.severity ?? "").toLowerCase();
      const leftRank = SEVERITY_ORDER.indexOf(leftSeverity);
      const rightRank = SEVERITY_ORDER.indexOf(rightSeverity);
      const leftScore = leftRank === -1 ? SEVERITY_ORDER.length : leftRank;
      const rightScore = rightRank === -1 ? SEVERITY_ORDER.length : rightRank;
      if (leftScore !== rightScore) return leftScore - rightScore;
      return (right.timestamp_ms ?? 0) - (left.timestamp_ms ?? 0);
    });
  }, [alerts]);

  const filtered = useMemo(() => {
    if (severityFilter === "all") return sorted;
    return sorted.filter(
      (alert) => String(alert.severity ?? "").toLowerCase() === severityFilter,
    );
  }, [severityFilter, sorted]);

  const stats = useMemo(() => {
    let critical = 0;
    let warning = 0;
    for (const alert of alerts) {
      const tone = severityTone(String(alert.severity ?? ""));
      if (tone === "danger") critical += 1;
      else if (tone === "warn") warning += 1;
    }
    return { total: alerts.length, critical, warning };
  }, [alerts]);

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}
      <CcMiniStats
        items={[
          { label: "Open alerts", value: stats.total, tone: stats.total > 0 ? "warn" : "ok" },
          { label: "Critical / high", value: stats.critical, tone: stats.critical > 0 ? "danger" : "ok" },
          { label: "Medium / warning", value: stats.warning, tone: stats.warning > 0 ? "warn" : "ok" },
        ]}
      />

      <CcSection
        title="Alert history"
        hint="Most severe alerts appear first."
        actions={
          <select
            value={severityFilter}
            onChange={(event) => setSeverityFilter(event.target.value)}
            aria-label="Filter by severity"
          >
            {severityOptions.map((level) => (
              <option key={level} value={level}>
                {level === "all" ? "All severities" : level}
              </option>
            ))}
          </select>
        }
      >
        {isLoading && alerts.length === 0 ? (
          <CcEmptyState title="Loading alerts…" />
        ) : filtered.length === 0 ? (
          <CcEmptyState
            title={alerts.length === 0 ? "All clear — no alerts" : "No alerts match this filter"}
            description={
              alerts.length === 0
                ? "Operational alerts from health checks, readiness, and integrations will appear here."
                : "Try selecting a different severity."
            }
          />
        ) : (
          <ul className="cc-alert-list">
            {filtered.map((alert, index) => {
              const severity = String(alert.severity ?? "unknown");
              const key = alert.id ?? `${severity}-${index}`;
              return (
                <li key={key} className="cc-alert-item">
                  <div className="cc-alert-item-header">
                    <CcBadge tone={severityTone(severity)}>{severity}</CcBadge>
                    {alert.alert_type && (
                      <span className="cc-alert-type">{String(alert.alert_type)}</span>
                    )}
                    <span className="cc-alert-time">
                      {formatTimestamp(alert.timestamp_ms)}
                    </span>
                  </div>
                  <p className="cc-alert-message">{String(alert.message ?? "(no message)")}</p>
                  <p className="cc-alert-source">Source: {String(alert.source ?? "unknown")}</p>
                </li>
              );
            })}
          </ul>
        )}
      </CcSection>
    </div>
  );
}
