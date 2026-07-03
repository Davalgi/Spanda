import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcMiniStats } from "./controlCenterUi";
import { scalarEntries } from "./controlCenterDataTable";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type Props = {
  baseUrl: string;
};

export function ExecutivePanel({ baseUrl }: Props) {
  const [scorecard, setScorecard] = useState<Record<string, unknown> | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/executive/scorecard`);
      if (!res.ok) throw new Error(`scorecard ${res.status}`);
      const body = await res.json();
      setScorecard((body.scorecard ?? body) as Record<string, unknown>);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const stats = scalarEntries(scorecard).slice(0, 6);

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}
      {!scorecard && !busy ? (
          <CcEmptyState title="Scorecard unavailable" />
        ) : busy && !scorecard ? (
          <CcEmptyState title="Loading scorecard…" />
        ) : (
          <>
            {stats.length > 0 && (
              <CcMiniStats
                items={stats.map(([label, value]) => ({
                  label: label.replace(/_/g, " "),
                  value,
                }))}
              />
            )}
            <dl className="cc-detail-grid">
              {scalarEntries(scorecard).map(([key, value]) => (
                <div key={key} className="cc-detail-row">
                  <dt>{key}</dt>
                  <dd>{value}</dd>
                </div>
              ))}
            </dl>
            <details className="cc-json-details">
              <summary>Raw scorecard JSON</summary>
              <pre className="cc-action-result">{JSON.stringify(scorecard, null, 2)}</pre>
            </details>
          </>
        )}
    </div>
  );
}
