import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcBadge, CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

const STRATEGIES = ["canary", "staged", "blue_green"] as const;

export function OtaPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [version, setVersion] = useState("1.0");
  const [strategy, setStrategy] = useState<(typeof STRATEGIES)[number]>("canary");
  const [otaState, setOtaState] = useState<Record<string, unknown> | null>(null);
  const [planLog, setPlanLog] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadStatus = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/ota/status`);
      if (!res.ok) throw new Error(`ota status ${res.status}`);
      const body = await res.json();
      setOtaState((body.state as Record<string, unknown>) ?? body);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void loadStatus();
  }, [loadStatus]);

  const postRollout = async (path: string, dryRun: boolean) => {
    if (!hasToken || !can("Deploy")) return;
    setBusy(true);
    setError(null);
    setPlanLog(null);
    try {
      const res = await fetch(`${baseUrl}${path}`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ version, strategy, dry_run: dryRun }),
      });
      const text = await res.text();
      if (!res.ok) throw new Error(`${path} ${res.status}`);
      setPlanLog(text);
      await loadStatus();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const deployVersion = otaState?.version ? String(otaState.version) : "—";
  const assignmentCount = Array.isArray(otaState?.assignments) ? otaState.assignments.length : 0;

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcMiniStats
        items={[
          { label: "Deployed version", value: deployVersion },
          { label: "Assignments", value: assignmentCount },
          { label: "Strategy", value: strategy },
        ]}
      />

      <CcSection
        title="Rollout plan"
        hint="Dry-run plans the rollout; execute applies it to the fleet."
      >
        <div className="cc-filter-bar">
          <label className="cc-field">
            Target version
            <input value={version} onChange={(event) => setVersion(event.target.value)} />
          </label>
          <label className="cc-field">
            Strategy
            <select
              value={strategy}
              onChange={(event) => setStrategy(event.target.value as (typeof STRATEGIES)[number])}
            >
              {STRATEGIES.map((entry) => (
                <option key={entry} value={entry}>
                  {entry}
                </option>
              ))}
            </select>
          </label>
        </div>

        {!hasToken && (
          <CcEmptyState
            title="Sign in to plan or execute rollouts"
            description="OTA plan and execute require Deploy permission."
          />
        )}

        <div className="cc-action-bar">
          <button
            type="button"
            onClick={() => void postRollout("/v1/ota/plan", true)}
            disabled={busy || !hasToken || !can("Deploy")}
          >
            Plan (dry-run)
          </button>
          <button
            type="button"
            className="primary"
            onClick={() => void postRollout("/v1/ota/execute", false)}
            disabled={busy || !hasToken || !can("Deploy")}
          >
            Execute rollout
          </button>
        </div>

        {planLog && <pre className="cc-action-result">{planLog}</pre>}
      </CcSection>

      <CcSection title="Deploy state" hint="Current OTA state from the Control Center store.">
        {!otaState ? (
          <CcEmptyState title={busy ? "Loading OTA status…" : "No deploy state"} />
        ) : (
          <dl className="cc-detail-grid">
            {Object.entries(otaState)
              .filter(([, value]) => typeof value !== "object")
              .slice(0, 12)
              .map(([key, value]) => (
                <div key={key} className="cc-detail-row">
                  <dt>{key}</dt>
                  <dd>{String(value)}</dd>
                </div>
              ))}
          </dl>
        )}
        {Array.isArray(otaState?.assignments) && otaState.assignments.length > 0 && (
          <div className="cc-table-wrap">
            <table className="cc-data-table">
              <thead>
                <tr>
                  <th>Robot</th>
                  <th>Version</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {(otaState.assignments as Record<string, unknown>[]).map((row, index) => (
                  <tr key={index}>
                    <td>{String(row.robot_id ?? row.robot ?? "—")}</td>
                    <td>{String(row.version ?? "—")}</td>
                    <td>
                      <CcBadge tone="neutral">{String(row.status ?? "pending")}</CcBadge>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </CcSection>
    </div>
  );
}
