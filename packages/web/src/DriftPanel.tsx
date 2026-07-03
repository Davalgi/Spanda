import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcBadge, CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type DriftData = {
  baselineId: string;
  report: Record<string, unknown>;
};

type Snapshot = {
  id: string;
  label?: string;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function DriftPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [snapshots, setSnapshots] = useState<Snapshot[]>([]);
  const [driftData, setDriftData] = useState<DriftData | null>(null);
  const [baselineId, setBaselineId] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadDrift = useCallback(
    async (selectedBaseline?: string) => {
      setBusy(true);
      setError(null);
      try {
        const snapsRes = await fetch(`${baseUrl}/v1/config/snapshots`);
        if (!snapsRes.ok) throw new Error(`snapshots ${snapsRes.status}`);
        const snapsBody = await snapsRes.json();
        const list: Snapshot[] = snapsBody.snapshots ?? [];
        setSnapshots(list);

        const baseline = selectedBaseline ?? list[0]?.id ?? "";
        setBaselineId(baseline);
        if (!baseline) {
          setDriftData(null);
          return;
        }

        const driftRes = await fetch(
          `${baseUrl}/v1/drift?baseline_id=${encodeURIComponent(baseline)}`,
        );
        if (!driftRes.ok) throw new Error(`drift ${driftRes.status}`);
        const body = await driftRes.json();
        setDriftData({ baselineId: baseline, report: body.report ?? body });
      } catch (err) {
        setError(String(err));
      } finally {
        setBusy(false);
      }
    },
    [baseUrl],
  );

  useEffect(() => {
    void loadDrift();
  }, [loadDrift]);

  useRegisterTabRefresh(() => loadDrift(), { busy });

  const runScan = async () => {
    if (!hasToken || !can("Deploy")) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/drift/scan`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify(baselineId ? { baseline_id: baselineId } : {}),
      });
      if (!res.ok) throw new Error(`drift scan ${res.status}`);
      await loadDrift(baselineId);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const requestApproval = async () => {
    if (!hasToken || !can("Deploy") || !driftData) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/config/approvals`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({
          snapshot_id: driftData.baselineId,
          note: "Control Center drift approval request",
        }),
      });
      if (!res.ok) throw new Error(`approval request ${res.status}`);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const passed = driftData?.report.passed;
  const byDimension =
    (driftData?.report.by_dimension as Record<string, number> | undefined) ?? {};
  const driftCount = Object.values(byDimension).reduce((sum, value) => sum + value, 0);

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      {snapshots.length === 0 && !busy ? (
        <CcEmptyState
          title="No config snapshots"
          description="Save a baseline with POST /v1/config/snapshots before running drift detection."
        />
      ) : (
        <>
          <CcMiniStats
            items={[
              { label: "Snapshots", value: snapshots.length },
              {
                label: "Scan result",
                value: passed === undefined ? "—" : passed ? "Passed" : "Failed",
                tone: passed === undefined ? "neutral" : passed ? "ok" : "danger",
              },
              {
                label: "Drift items",
                value: driftCount,
                tone: driftCount > 0 ? "warn" : "ok",
              },
            ]}
          />

          <CcSection
            title="Baseline & scan"
            hint="Compare live configuration against a saved snapshot."
            actions={
              <div className="cc-filter-bar">
                <select
                  value={baselineId}
                  onChange={(event) => {
                    const value = event.target.value;
                    setBaselineId(value);
                    void loadDrift(value);
                  }}
                  aria-label="Baseline snapshot"
                >
                  {snapshots.map((snapshot) => (
                    <option key={snapshot.id} value={snapshot.id}>
                      {snapshot.label ? `${snapshot.label} (${snapshot.id})` : snapshot.id}
                    </option>
                  ))}
                </select>
                <button
                  type="button"
                  className="primary"
                  onClick={() => void runScan()}
                  disabled={busy || !hasToken || !can("Deploy")}
                >
                  {busy ? "Scanning…" : "Run drift scan"}
                </button>
              </div>
            }
          >
            {driftData && (
              <p className="cc-drift-baseline">
                Baseline <code>{driftData.baselineId}</code>{" "}
                <CcBadge tone={passed ? "ok" : "danger"}>
                  {passed ? "passed" : "failed"}
                </CcBadge>
              </p>
            )}
          </CcSection>

          {driftData && Object.keys(byDimension).length > 0 && (
            <CcSection title="Drift by dimension" hint="Where configuration diverges from baseline.">
              <div className="cc-health-bars">
                {Object.entries(byDimension).map(([name, count]) => {
                  const max = Math.max(...Object.values(byDimension), 1);
                  const width = Math.max(4, (count / max) * 100);
                  return (
                    <div key={name} className="cc-health-bar-row">
                      <span className="cc-health-bar-label">{name}</span>
                      <div className="cc-health-bar-track">
                        <div
                          className={`cc-health-bar-fill tone-${count > 0 ? "warn" : "ok"}`}
                          style={{ width: `${width}%` }}
                        />
                      </div>
                      <span className="cc-health-bar-value">{count}</span>
                    </div>
                  );
                })}
              </div>
            </CcSection>
          )}

          {driftData && hasToken && can("Deploy") && (
            <div className="cc-action-bar">
              <button type="button" onClick={() => void requestApproval()} disabled={busy}>
                Request publish approval
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
}
