/** Shared Control Center solution-domain dashboard shell. @module */

import { useCallback, useEffect, useState } from "react";
import type { ControlCenterTab } from "./controlCenterRbac";
import type { RbacAction } from "./controlCenterRbac";
import { CcEmptyState, CcMiniStats, CcNotice, CcSection } from "./controlCenterUi";
import { ControlCenterDataTable } from "./controlCenterDataTable";
import { useControlCenterDemoMode } from "./useControlCenterDemoMode";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

export type SolutionDomainConfig = {
  id: string;
  title: string;
  hint: string;
  exampleConfig: string;
  exampleProgram: string;
  deepLinks: { tab: ControlCenterTab; label: string }[];
  complianceProfile?: string;
  /** Optional tags/keywords used to highlight domain-relevant missions. */
  domainTags?: string[];
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
  config: SolutionDomainConfig;
  onNavigate: (tab: ControlCenterTab) => void;
  extraLoads?: Array<{
    key: string;
    path: string;
    label: string;
  }>;
};

export function SolutionDomainPanel({
  baseUrl,
  authHeaders,
  can,
  hasToken,
  config,
  onNavigate,
  extraLoads = [],
}: Props) {
  const { demoMode } = useControlCenterDemoMode();
  // Hold composed fleet/mission/readiness/domain payloads for the solution dashboard.
  const [health, setHealth] = useState<Record<string, unknown> | null>(null);
  const [readiness, setReadiness] = useState<Record<string, unknown> | null>(null);
  const [missions, setMissions] = useState<Record<string, unknown>[]>([]);
  const [approvals, setApprovals] = useState<Record<string, unknown>[]>([]);
  const [fleetMap, setFleetMap] = useState<Record<string, unknown> | null>(null);
  const [sre, setSre] = useState<Record<string, unknown> | null>(null);
  const [extras, setExtras] = useState<Record<string, Record<string, unknown> | null>>({});
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    // Compose shared ops APIs plus optional domain-specific endpoints.
    setBusy(true);
    setError(null);
    try {
      const headers = authHeaders();
      const [healthRes, readinessRes, missionsRes, approvalsRes, mapRes, sreRes, ...extraRes] =
        await Promise.all([
          fetch(`${baseUrl}/v1/health/summary`),
          fetch(`${baseUrl}/v1/readiness/impact`),
          fetch(`${baseUrl}/v1/operator/missions`, { headers }),
          fetch(`${baseUrl}/v1/operator/mission/approvals`, { headers }),
          fetch(`${baseUrl}/v1/fleet/map`),
          fetch(`${baseUrl}/v1/sre/summary`),
          ...extraLoads.map((item) => fetch(`${baseUrl}${item.path}`, { headers })),
        ]);
      if (healthRes.ok) setHealth(await healthRes.json());
      if (readinessRes.ok) setReadiness(await readinessRes.json());
      if (missionsRes.ok) {
        const body = await missionsRes.json();
        setMissions(Array.isArray(body.missions) ? body.missions : Array.isArray(body) ? body : []);
      }
      if (approvalsRes.ok) {
        const body = await approvalsRes.json();
        setApprovals(
          Array.isArray(body.approvals) ? body.approvals : Array.isArray(body) ? body : [],
        );
      }
      if (mapRes.ok) setFleetMap(await mapRes.json());
      if (sreRes.ok) setSre(await sreRes.json());
      const nextExtras: Record<string, Record<string, unknown> | null> = {};
      for (let i = 0; i < extraLoads.length; i += 1) {
        const res = extraRes[i];
        nextExtras[extraLoads[i].key] = res?.ok ? await res.json() : null;
      }
      setExtras(nextExtras);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl, extraLoads]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const mapPins = Array.isArray(fleetMap?.pins)
    ? (fleetMap?.pins as Record<string, unknown>[])
    : Array.isArray(fleetMap?.robots)
      ? (fleetMap?.robots as Record<string, unknown>[])
      : [];

  const domainTags = (config.domainTags ?? [config.id]).map((tag) => tag.toLowerCase());

  // Prefer missions that mention the domain tag; fall back to the full live queue.
  const taggedMissions = missions.filter((row) => {
    const haystack = JSON.stringify(row).toLowerCase();
    return domainTags.some((tag) => haystack.includes(tag));
  });
  const missionRows = taggedMissions.length > 0 ? taggedMissions : missions;
  const showingUnfiltered = taggedMissions.length === 0 && missions.length > 0;

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcNotice
        tone={demoMode ? "info" : "warn"}
        title={
          demoMode
            ? `Demo mode — ${config.title} composite showcase`
            : "Composite solution view — shared fleet APIs, not a separate domain product"
        }
      >
        This tab composes live <code>/v1/health</code>, missions, fleet map, and SRE data
        {demoMode ? " for blueprint demos" : ""}. Serve the matching blueprint so tables reflect{" "}
        {config.title} entities:{" "}
        <code>
          spanda control-center serve --config {config.exampleConfig} --program{" "}
          {config.exampleProgram}
        </code>
        . Domain-specific extras below are only present when that config exposes them.
      </CcNotice>

      <CcMiniStats
        items={[
          { label: "Health", value: String(health?.overall_status ?? "—") },
          {
            label: "Mission ready",
            value:
              readiness?.mission_ready === true
                ? "yes"
                : readiness?.mission_ready === false
                  ? "no"
                  : "—",
          },
          { label: "Missions", value: missionRows.length },
          { label: "Approvals", value: approvals.length },
          { label: "Map pins", value: mapPins.length },
          { label: "Incidents", value: Number(sre?.open_incidents ?? sre?.incident_count ?? 0) },
        ]}
      />

      <CcSection title={config.title} hint={config.hint}>
        <div className="cc-action-bar" style={{ flexWrap: "wrap" }}>
          {config.deepLinks.map((link) => (
            <button key={link.tab} type="button" onClick={() => onNavigate(link.tab)}>
              {link.label}
            </button>
          ))}
          <button type="button" onClick={() => void load()} disabled={busy}>
            Refresh
          </button>
        </div>
      </CcSection>

      <CcSection title="Mission approvals" hint="Operator mission approve queue for this domain.">
        {!hasToken && <p className="demo-hint">Sign in for approval mutations.</p>}
        {approvals.length === 0 ? (
          <CcEmptyState title="No pending approvals" />
        ) : (
          <ControlCenterDataTable
            rows={approvals}
            rowKey={(row, index) => String(row.id ?? row.mission_id ?? index)}
            columns={[
              {
                key: "id",
                header: "ID",
                render: (row) => String(row.id ?? row.mission_id ?? "—"),
              },
              {
                key: "status",
                header: "Status",
                render: (row) => String(row.status ?? "—"),
              },
              {
                key: "summary",
                header: "Summary",
                render: (row) => String(row.summary ?? row.mission ?? "—"),
              },
            ]}
          />
        )}
        {can("Approve") && (
          <p className="cc-section-hint">Use Mission / Operator tabs to approve or reject.</p>
        )}
      </CcSection>

      <CcSection
        title="Active missions"
        hint={
          showingUnfiltered
            ? `No missions tagged for ${config.id} — showing full operator queue from the loaded config.`
            : `Missions matching ${domainTags.join(", ")} when present.`
        }
      >
        {missionRows.length === 0 ? (
          <CcEmptyState
            title="No operator missions"
            description={`Load ${config.exampleConfig} to populate this solution view.`}
          />
        ) : (
          <ControlCenterDataTable
            rows={missionRows.slice(0, 20)}
            rowKey={(row, index) => String(row.id ?? row.mission_id ?? index)}
            columns={[
              {
                key: "id",
                header: "Mission",
                render: (row) => String(row.id ?? row.mission_id ?? "—"),
              },
              {
                key: "state",
                header: "State",
                render: (row) => String(row.state ?? row.status ?? "—"),
              },
              {
                key: "robot",
                header: "Robot",
                render: (row) => String(row.robot_id ?? row.robot ?? "—"),
              },
            ]}
          />
        )}
      </CcSection>

      <CcSection title="Fleet map snapshot" hint="Pins from GET /v1/fleet/map — open Fleet map for full view.">
        {mapPins.length === 0 ? (
          <CcEmptyState title="No map pins" />
        ) : (
          <ControlCenterDataTable
            rows={mapPins.slice(0, 20)}
            rowKey={(row, index) => String(row.id ?? row.robot_id ?? index)}
            columns={[
              {
                key: "id",
                header: "ID",
                render: (row) => String(row.id ?? row.robot_id ?? "—"),
              },
              {
                key: "label",
                header: "Label",
                render: (row) => String(row.label ?? row.name ?? "—"),
              },
              {
                key: "x",
                header: "X / lon",
                render: (row) => String(row.x ?? row.lon ?? row.longitude ?? "—"),
              },
              {
                key: "y",
                header: "Y / lat",
                render: (row) => String(row.y ?? row.lat ?? row.latitude ?? "—"),
              },
            ]}
          />
        )}
      </CcSection>

      {extraLoads.map((item) => {
        const data = extras[item.key];
        const rows = Array.isArray(data)
          ? (data as Record<string, unknown>[])
          : Array.isArray(data?.items)
            ? (data.items as Record<string, unknown>[])
            : Array.isArray(data?.sessions)
              ? (data.sessions as Record<string, unknown>[])
              : Array.isArray(data?.humans)
                ? (data.humans as Record<string, unknown>[])
                : null;
        return (
          <CcSection
            key={item.key}
            title={item.label}
            hint={`GET ${item.path} — empty unless the loaded blueprint exposes this endpoint.`}
          >
            {rows ? (
              <ControlCenterDataTable
                rows={rows.slice(0, 20)}
                rowKey={(row, index) => String(row.id ?? row.name ?? index)}
                columns={[
                  {
                    key: "id",
                    header: "ID",
                    render: (row) => String(row.id ?? row.name ?? "—"),
                  },
                  {
                    key: "status",
                    header: "Status",
                    render: (row) => String(row.status ?? row.state ?? row.ready ?? "—"),
                  },
                  {
                    key: "detail",
                    header: "Detail",
                    render: (row) =>
                      String(row.summary ?? row.label ?? row.type ?? row.role ?? "—"),
                  },
                ]}
              />
            ) : data ? (
              <pre className="cc-action-result">{JSON.stringify(data, null, 2)}</pre>
            ) : (
              <CcEmptyState
                title={`No ${item.label.toLowerCase()} data`}
                description={`Serve with ${config.exampleConfig} or open the linked domain tab.`}
              />
            )}
          </CcSection>
        );
      })}

      {config.complianceProfile && (
        <CcSection title="Compliance">
          <p className="cc-section-hint">
            Export profile <code>{config.complianceProfile}</code> from the Compliance tab (
            <code>GET /v1/compliance/export?profile={config.complianceProfile}</code>).
          </p>
        </CcSection>
      )}
    </div>
  );
}
