import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcBadge, CcEmptyState, CcMiniStats, CcPanelToolbar, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";
import { ControlCenterDataTable } from "./controlCenterDataTable";

type TwinRow = {
  twin_id?: string;
  program?: string;
  readiness_score?: number;
  mission_ready?: boolean;
  history_count?: number;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  hasToken: boolean;
  can: (action: RbacAction) => boolean;
};

export function TwinsPanel({ baseUrl, authHeaders, hasToken, can }: Props) {
  const [payload, setPayload] = useState<Record<string, unknown> | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/twins`);
      if (!res.ok) throw new Error(`twins ${res.status}`);
      setPayload(await res.json());
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

  const sync = async () => {
    if (!hasToken || !can("Deploy")) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/twins/sync`, {
        method: "POST",
        headers: { "Content-Type": "application/json", ...authHeaders() },
        body: "{}",
      });
      if (!res.ok) throw new Error(`twin sync ${res.status}`);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const twins = (payload?.twins as TwinRow[] | undefined) ?? [];

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}
      <CcPanelToolbar>
        <button
          type="button"
          className="primary"
          onClick={() => void sync()}
          disabled={busy || !hasToken || !can("Deploy")}
        >
          Sync loaded program
        </button>
      </CcPanelToolbar>
      {twins.length === 0 ? (
          <CcEmptyState
            title="No twins registered"
            description="Push with spanda twin cloud push or sync a loaded program."
          />
        ) : (
          <ControlCenterDataTable
            rows={twins}
            rowKey={(row) => String(row.twin_id ?? row.program)}
            columns={[
              { key: "id", header: "Twin ID", render: (row) => String(row.twin_id ?? "—") },
              { key: "program", header: "Program", render: (row) => String(row.program ?? "—") },
              {
                key: "readiness",
                header: "Readiness",
                render: (row) => String(row.readiness_score ?? "—"),
              },
              {
                key: "ready",
                header: "Mission ready",
                render: (row) => (row.mission_ready ? "yes" : "no"),
              },
              {
                key: "history",
                header: "History",
                render: (row) => String(row.history_count ?? 0),
              },
            ]}
          />
        )}
      {payload && (
        <CcSection title="Twin Cloud usage">
          <CcMiniStats
            items={[
              { label: "Registered twins", value: twins.length },
              {
                label: "Total history entries",
                value: twins.reduce((sum, twin) => sum + (twin.history_count ?? 0), 0),
              },
              {
                label: "Mission-ready",
                value: twins.filter((twin) => twin.mission_ready).length,
                tone: "ok",
              },
            ]}
          />
          <p className="cc-section-hint">
            Billing dimensions: snapshot push, sync, and stored history count per tenant.
          </p>
        </CcSection>
      )}
    </div>
  );
}
