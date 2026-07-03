import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";

type ForecastPayload = {
  forecast?: { score?: number; horizon?: string; warnings?: string[] };
  trends?: { factor?: string; slope?: number; volatility?: number }[];
  history?: { samples?: number };
};

type Props = {
  baseUrl: string;
};

export function ReadinessTrendsPanel({ baseUrl }: Props) {
  const [forecast, setForecast] = useState<ForecastPayload | null>(null);
  const [sreTrends, setSreTrends] = useState<Record<string, unknown> | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [forecastRes, sreRes] = await Promise.all([
        fetch(`${baseUrl}/v1/analytics/readiness-forecast?all=1`),
        fetch(`${baseUrl}/v1/sre/summary`),
      ]);
      if (forecastRes.ok) {
        setForecast(await forecastRes.json());
      }
      if (sreRes.ok) {
        const body = await sreRes.json();
        setSreTrends((body.readiness_trends as Record<string, unknown>) ?? null);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  const trends = forecast?.trends ?? [];
  const warnings = forecast?.forecast?.warnings ?? [];

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Readiness trends & forecast"
        hint="Analytics forecast and SRE readiness trend rollup."
        actions={
          <button type="button" onClick={() => void load()} disabled={busy}>
            Refresh
          </button>
        }
      >
        <CcMiniStats
          items={[
            {
              label: "Forecast score",
              value: forecast?.forecast?.score?.toFixed(1) ?? "—",
            },
            { label: "Horizon", value: forecast?.forecast?.horizon ?? "7d" },
            { label: "Samples", value: forecast?.history?.samples ?? sreTrends?.sample_count ?? "—" },
            { label: "Warnings", value: warnings.length, tone: warnings.length > 0 ? "warn" : "ok" },
          ]}
        />

        {trends.length > 0 ? (
          <div className="cc-trend-chart">
            {trends.map((trend) => {
              const slope = trend.slope ?? 0;
              const height = Math.min(100, Math.max(8, Math.abs(slope) * 40 + 20));
              const tone = slope >= 0 ? "ok" : "danger";
              return (
                <div key={trend.factor ?? "factor"} className="cc-trend-bar-wrap">
                  <div
                    className={`cc-trend-bar tone-${tone}`}
                    style={{ height: `${height}%` }}
                    title={`slope ${slope}`}
                  />
                  <span className="cc-trend-label">{trend.factor ?? "—"}</span>
                </div>
              );
            })}
          </div>
        ) : (
          <CcEmptyState
            title="No trend samples yet"
            description="Record readiness with spanda readiness --record or serve with a loaded program."
          />
        )}

        {warnings.length > 0 && (
          <ul className="cc-warning-list">
            {warnings.map((warning) => (
              <li key={warning}>{warning}</li>
            ))}
          </ul>
        )}
      </CcSection>
    </div>
  );
}
