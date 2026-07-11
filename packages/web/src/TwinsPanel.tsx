/** Twin Cloud registry panel with per-tenant usage meters. @module */

import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcEmptyState, CcMiniStats, CcPanelToolbar, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";
import { ControlCenterDataTable } from "./controlCenterDataTable";

type TwinRow = {
  twin_id?: string;
  program?: string;
  readiness_score?: number;
  mission_ready?: boolean;
  history_count?: number;
};

type TwinUsage = {
  tenant_id?: string;
  twin_count?: number;
  snapshot_count?: number;
  push_count?: number;
  sync_count?: number;
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
  const [usage, setUsage] = useState<TwinUsage | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [twinsRes, usageRes] = await Promise.all([
        fetch(`${baseUrl}/v1/twins`, { headers: authHeaders() }),
        fetch(`${baseUrl}/v1/twins/usage`, { headers: authHeaders() }),
      ]);
      if (!twinsRes.ok) throw new Error(`twins ${twinsRes.status}`);
      setPayload(await twinsRes.json());
      if (usageRes.ok) {
        setUsage((await usageRes.json()) as TwinUsage);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl]);

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
      {(usage || payload) && (
        <CcSection title="Twin Cloud usage">
          <CcMiniStats
            items={[
              {
                label: "Registered twins",
                value: usage?.twin_count ?? twins.length,
              },
              {
                label: "Stored snapshots",
                value:
                  usage?.snapshot_count ??
                  twins.reduce((sum, twin) => sum + (twin.history_count ?? 0), 0),
              },
              {
                label: "Push calls",
                value: usage?.push_count ?? 0,
              },
              {
                label: "Sync calls",
                value: usage?.sync_count ?? 0,
              },
              {
                label: "History reads",
                value: usage?.history_count ?? 0,
              },
              {
                label: "Mission-ready",
                value: twins.filter((twin) => twin.mission_ready).length,
                tone: "ok",
              },
            ]}
          />
          <p className="cc-section-hint">
            Tenant {usage?.tenant_id ?? "—"} · meters from{" "}
            <code>GET /v1/twins/usage</code> (store counts + in-process API counters).
          </p>
        </CcSection>
      )}
    </div>
  );
}
