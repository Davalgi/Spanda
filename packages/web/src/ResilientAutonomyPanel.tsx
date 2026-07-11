/** Cognitive & Resilience Control Center panel — functional domain live views. @module */

import { useCallback, useEffect, useMemo, useState } from "react";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
};

type MemoryCategoryName = "reflex" | "working" | "episodic" | "semantic" | "procedural";

type CategoryRow = {
  entity_id?: string;
  ref?: string;
  artifact_kind?: string;
  timestamp?: string;
};

type MaintenanceWindow = {
  id: string;
  start: string;
  end: string;
  activities: string[];
};

const MEMORY_CATEGORIES: MemoryCategoryName[] = [
  "reflex",
  "working",
  "episodic",
  "semantic",
  "procedural",
];

function asArray<T>(value: unknown): T[] {
  return Array.isArray(value) ? (value as T[]) : [];
}

async function fetchJson(
  baseUrl: string,
  path: string,
  authHeaders: () => HeadersInit,
): Promise<unknown> {
  const response = await fetch(`${baseUrl}${path}`, { headers: authHeaders() });
  if (!response.ok) {
    throw new Error(`${path} returned ${response.status}`);
  }
  return response.json();
}

export function ResilientAutonomyPanel({ baseUrl, authHeaders }: Props) {
  // Render Cognitive & Resilience domain panels from live Control Center REST.
  //
  // Parameters:
  // - `baseUrl` — Control Center API base URL
  // - `authHeaders` — bearer / API key header factory
  //
  // Returns:
  // React panel element.
  //
  // Options:
  // None.
  //
  // Example:
  // <ResilientAutonomyPanel baseUrl={url} authHeaders={headers} />

  const [reflexes, setReflexes] = useState<unknown[]>([]);
  const [traces, setTraces] = useState<unknown[]>([]);
  const [homeostasis, setHomeostasis] = useState<unknown[]>([]);
  const [quarantined, setQuarantined] = useState<unknown[]>([]);
  const [attention, setAttention] = useState<unknown[]>([]);
  const [memory, setMemory] = useState<unknown[]>([]);
  const [memoryByCategory, setMemoryByCategory] = useState<
    Partial<Record<MemoryCategoryName, CategoryRow[]>>
  >({});
  const [memoryCategory, setMemoryCategory] = useState<MemoryCategoryName>("episodic");
  const [maintenanceWindows, setMaintenanceWindows] = useState<MaintenanceWindow[]>([]);
  const [recoveryConfidence, setRecoveryConfidence] = useState<number | null>(null);
  const [entityAutonomy, setEntityAutonomy] = useState<Record<string, unknown> | null>(null);
  const [strategicPlanning, setStrategicPlanning] = useState<Record<string, unknown> | null>(null);
  const [entityCount, setEntityCount] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [
        reflexPayload,
        tracePayload,
        homeoPayload,
        immunityPayload,
        attentionPayload,
        memoryPayload,
        metricsPayload,
        maintenancePayload,
      ] = await Promise.all([
        fetchJson(baseUrl, "/v1/autonomy/reflex", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/reflex/traces", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/homeostasis", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/immunity", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/attention", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/memory", authHeaders),
        fetchJson(baseUrl, "/v1/recovery/metrics", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/maintenance/windows", authHeaders).catch(() => ({
          windows: [],
        })),
      ]);
      setReflexes(asArray((reflexPayload as { reflexes?: unknown }).reflexes));
      setTraces(asArray((tracePayload as { traces?: unknown }).traces));
      setHomeostasis(asArray((homeoPayload as { reports?: unknown }).reports));
      setQuarantined(asArray((immunityPayload as { quarantined?: unknown }).quarantined));
      const attentionWindow = (attentionPayload as { attention?: { items?: unknown } }).attention;
      setAttention(asArray(attentionWindow?.items));
      setMemory(asArray((memoryPayload as { memory?: unknown }).memory));
      const byCategory = (memoryPayload as { by_category?: Partial<Record<MemoryCategoryName, CategoryRow[]>> })
        .by_category;
      setMemoryByCategory(byCategory ?? {});
      setMaintenanceWindows(
        asArray((maintenancePayload as { windows?: MaintenanceWindow[] }).windows),
      );
      const metrics = metricsPayload as { recovery_confidence?: number };
      setRecoveryConfidence(
        typeof metrics.recovery_confidence === "number" ? metrics.recovery_confidence : null,
      );

      const entitiesPayload = (await fetchJson(baseUrl, "/v1/entities", authHeaders)) as {
        entities?: Array<{ id?: string }>;
      };
      const entities = entitiesPayload.entities ?? [];
      setEntityCount(entities.length);

      const [governancePayload, profilesPayload] = await Promise.all([
        fetchJson(baseUrl, "/v1/governance", authHeaders).catch(() => null),
        fetchJson(baseUrl, "/v1/deployment-profiles", authHeaders).catch(() => null),
      ]);
      setStrategicPlanning({
        governance: governancePayload,
        deployment_profiles: profilesPayload,
        entity_inventory: entities.length,
      });

      const firstId = entities[0]?.id;
      if (firstId) {
        const autonomy = (await fetchJson(
          baseUrl,
          `/v1/entities/${encodeURIComponent(firstId)}/autonomy`,
          authHeaders,
        )) as Record<string, unknown>;
        setEntityAutonomy(autonomy);
      } else {
        setEntityAutonomy(null);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [authHeaders, baseUrl]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  useRegisterTabRefresh("resilient-autonomy", refresh);

  const categoryRows = useMemo(() => {
    return memoryByCategory[memoryCategory] ?? [];
  }, [memoryByCategory, memoryCategory]);

  if (loading && !reflexes.length && !error) {
    return (
      <div className="cc-panel">
        <CcEmptyState
          title="Loading Cognitive & Resilience…"
          description="Fetching functional domain endpoints under /v1/autonomy/*."
        />
      </div>
    );
  }

  const unstableCount = homeostasis.filter(
    (row) => typeof row === "object" && row && (row as { stable?: boolean }).stable === false,
  ).length;

  const autonomyProfile = entityAutonomy?.autonomy as Record<string, unknown> | undefined;
  const damageRisk = autonomyProfile?.damage_risk;
  const preferredStrategy = (
    autonomyProfile?.recovery_confidence as { preferred_strategy?: string } | undefined
  )?.preferred_strategy;

  return (
    <div className="cc-panel">
      <header className="cc-panel-header">
        <h2>Cognitive & Resilience</h2>
        <p className="cc-panel-subtitle">
          Functional architecture view — reflex & safety, attention, homeostasis, platform immunity,
          operational memory, damage risk, and recovery confidence. Organized by responsibility
          domain, not implementation layer.
        </p>
      </header>

      {error ? (
        <CcSection title="Connection" hint="Control Center must be running with a loaded program/config.">
          <p className="cc-muted">{error}</p>
        </CcSection>
      ) : null}

      <CcMiniStats
        items={[
          { label: "Reflex actions", value: reflexes.length },
          { label: "Unstable entities", value: unstableCount, tone: unstableCount > 0 ? "warn" : "ok" },
          { label: "Attention queue", value: attention.length },
          { label: "Quarantined", value: quarantined.length, tone: quarantined.length > 0 ? "danger" : "ok" },
          {
            label: "Recovery confidence",
            value: recoveryConfidence != null ? `${(recoveryConfidence * 100).toFixed(0)}%` : "—",
          },
        ]}
      />

      <CcSection
        title="Strategic Planning"
        hint="Governance, deployment profiles, and entity inventory — mission planning context."
      >
        <pre className="cc-code-block">{JSON.stringify(strategicPlanning, null, 2)}</pre>
        <p className="cc-muted">Entities in registry: {entityCount}</p>
      </CcSection>

      <CcSection
        title="Reflex Events"
        hint="Reflex & Safety domain — layer-0 safety actions from /v1/autonomy/reflex."
      >
        <pre className="cc-code-block">{JSON.stringify(reflexes.slice(0, 5), null, 2)}</pre>
        <p className="cc-muted">Recent traces: {traces.length}</p>
      </CcSection>

      <CcSection
        title="Attention Queue"
        hint="Attention Engine — prioritized signals; mission and safety events surface first."
      >
        <pre className="cc-code-block">{JSON.stringify(attention.slice(0, 8), null, 2)}</pre>
      </CcSection>

      <CcSection
        title="Homeostasis"
        hint="Homeostasis Engine — stability reports from entity health, readiness, and scheduler telemetry."
      >
        <pre className="cc-code-block">{JSON.stringify(homeostasis.slice(0, 5), null, 2)}</pre>
      </CcSection>

      <CcSection
        title="Platform Immunity"
        hint="Trust-boundary violations requiring quarantine or isolation."
      >
        <pre className="cc-code-block">{JSON.stringify(quarantined, null, 2)}</pre>
      </CcSection>

      <CcSection
        title="Operational Memory"
        hint="Browse memory by category — episodic rows link to the replay/trace index."
      >
        <div className="cc-inline-actions" style={{ marginBottom: "0.75rem", flexWrap: "wrap", gap: "0.35rem" }}>
          {MEMORY_CATEGORIES.map((category) => (
            <button
              key={category}
              type="button"
              className={memoryCategory === category ? "primary" : "secondary"}
              aria-pressed={memoryCategory === category}
              onClick={() => setMemoryCategory(category)}
            >
              {category}
            </button>
          ))}
        </div>
        {categoryRows.length === 0 ? (
          <CcEmptyState
            title={`No ${memoryCategory} refs`}
            description={
              memory.length === 0
                ? "Load a program/config so entities receive memory refs."
                : "Switch category or index replay traces under the project root."
            }
          />
        ) : (
          <div className="cc-table-wrap">
            <table className="cc-data-table">
              <thead>
                <tr>
                  <th>Entity</th>
                  <th>Reference</th>
                  {memoryCategory === "episodic" ? <th>Kind</th> : null}
                  {memoryCategory === "episodic" ? <th>Timestamp</th> : null}
                </tr>
              </thead>
              <tbody>
                {categoryRows.slice(0, 40).map((row, index) => (
                  <tr key={`${row.entity_id ?? "e"}-${row.ref ?? index}`}>
                    <td>{row.entity_id ?? "—"}</td>
                    <td>{typeof row.ref === "string" ? row.ref : JSON.stringify(row.ref)}</td>
                    {memoryCategory === "episodic" ? <td>{row.artifact_kind ?? "—"}</td> : null}
                    {memoryCategory === "episodic" ? <td>{row.timestamp ?? "—"}</td> : null}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </CcSection>

      <CcSection
        title="Maintenance Schedule"
        hint="Maintenance windows from GET /v1/autonomy/maintenance/windows (set via CLI or POST with Operate)."
      >
        {maintenanceWindows.length === 0 ? (
          <CcEmptyState
            title="No maintenance windows"
            description="Schedule with: spanda maintenance window set --id nightly --start … --end …"
          />
        ) : (
          <div className="cc-table-wrap">
            <table className="cc-data-table">
              <thead>
                <tr>
                  <th>ID</th>
                  <th>Start</th>
                  <th>End</th>
                  <th>Activities</th>
                </tr>
              </thead>
              <tbody>
                {maintenanceWindows.map((window) => (
                  <tr key={window.id}>
                    <td>{window.id}</td>
                    <td>{window.start}</td>
                    <td>{window.end}</td>
                    <td>{(window.activities ?? []).join(", ") || "—"}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </CcSection>

      <CcSection
        title="Damage Risk"
        hint="Damage Risk Assessment — harm potential from first entity autonomy profile."
      >
        <pre className="cc-code-block">{JSON.stringify(damageRisk ?? {}, null, 2)}</pre>
      </CcSection>

      <CcSection
        title="Recovery Confidence"
        hint="Adaptive Learning — strategy preference feeds mission abort/replan."
      >
        <p className="cc-muted">
          Platform score:{" "}
          {recoveryConfidence != null ? `${(recoveryConfidence * 100).toFixed(0)}%` : "—"}
          {preferredStrategy ? ` · Preferred strategy: ${preferredStrategy}` : ""}
        </p>
        <pre className="cc-code-block">
          {JSON.stringify(
            {
              platform_score: recoveryConfidence,
              preferred_strategy: preferredStrategy ?? null,
              entity_profile: entityAutonomy,
            },
            null,
            2,
          )}
        </pre>
      </CcSection>
    </div>
  );
}
