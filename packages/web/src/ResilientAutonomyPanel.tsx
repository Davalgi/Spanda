/** Cognitive & Resilience Control Center panel — functional domain live views. @module */

import { useCallback, useEffect, useMemo, useState } from "react";
import {
  CcBadge,
  CcEmptyState,
  CcMiniStats,
  CcNotice,
  CcSection,
} from "./controlCenterUi";
import { ControlCenterDataTable } from "./controlCenterDataTable";
import { useControlCenterDemoMode } from "./useControlCenterDemoMode";
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

type TraceRow = {
  reflex_id?: string;
  entity_id?: string;
  trigger?: string;
  action_taken?: string;
  timestamp?: string;
  priority?: number | string;
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

function sourceTone(source: string): "ok" | "warn" | "info" | "neutral" {
  if (source === "runtime" || source === "program") return "ok";
  if (source === "platform_defaults" || source === "catalog" || source === "none") return "warn";
  return "neutral";
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

  const [reflexes, setReflexes] = useState<Record<string, unknown>[]>([]);
  const { demoMode } = useControlCenterDemoMode();
  const [traces, setTraces] = useState<TraceRow[]>([]);
  const [catalogExamples, setCatalogExamples] = useState<TraceRow[]>([]);
  const [traceSource, setTraceSource] = useState<string>("none");
  const [showCatalog, setShowCatalog] = useState(false);
  const [homeostasis, setHomeostasis] = useState<Record<string, unknown>[]>([]);
  const [homeoPolicySource, setHomeoPolicySource] = useState<string>("platform_defaults");
  const [quarantined, setQuarantined] = useState<Record<string, unknown>[]>([]);
  const [attention, setAttention] = useState<Record<string, unknown>[]>([]);
  const [attentionPolicySource, setAttentionPolicySource] = useState<string>("platform_defaults");
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
      const traceBody = tracePayload as {
        traces?: unknown;
        catalog_examples?: unknown;
        source?: string;
      };
      setTraces(asArray<TraceRow>(traceBody.traces));
      setCatalogExamples(asArray<TraceRow>(traceBody.catalog_examples));
      setTraceSource(typeof traceBody.source === "string" ? traceBody.source : "none");
      setHomeostasis(asArray((homeoPayload as { reports?: unknown }).reports));
      setHomeoPolicySource(
        typeof (homeoPayload as { policy_source?: string }).policy_source === "string"
          ? (homeoPayload as { policy_source: string }).policy_source
          : "platform_defaults",
      );
      setQuarantined(asArray((immunityPayload as { quarantined?: unknown }).quarantined));
      const attentionWindow = (attentionPayload as { attention?: { items?: unknown } }).attention;
      setAttention(asArray(attentionWindow?.items));
      setAttentionPolicySource(
        typeof (attentionPayload as { policy_source?: string }).policy_source === "string"
          ? (attentionPayload as { policy_source: string }).policy_source
          : "platform_defaults",
      );
      setMemory(asArray((memoryPayload as { memory?: unknown }).memory));
      const byCategory = (
        memoryPayload as { by_category?: Partial<Record<MemoryCategoryName, CategoryRow[]>> }
      ).by_category;
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

  useRegisterTabRefresh(refresh);

  const categoryRows = useMemo(() => {
    return memoryByCategory[memoryCategory] ?? [];
  }, [memoryByCategory, memoryCategory]);

  // Demo mode (or an explicit toggle) shows catalog examples when the runtime buffer is empty.
  const useCatalog = (demoMode || showCatalog) && traces.length === 0;
  const visibleTraces = useCatalog ? catalogExamples : traces;

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

  const unstableCount = homeostasis.filter((row) => row.stable === false).length;

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
          operational memory, damage risk, and recovery confidence.
        </p>
      </header>

      {error ? (
        <CcSection title="Connection" hint="Control Center must be running with a loaded program/config.">
          <p className="cc-muted">{error}</p>
        </CcSection>
      ) : null}

      {demoMode && catalogExamples.length > 0 && traces.length === 0 ? (
        <CcNotice tone="info" title="Demo mode — catalog reflex examples">
          Showing illustrative catalog traces until runtime events are recorded. Turn Demo mode off
          for an empty runtime-only queue, or load a program and trigger reflexes.
        </CcNotice>
      ) : null}
      {!demoMode &&
        (homeoPolicySource === "platform_defaults" ||
          attentionPolicySource === "platform_defaults" ||
          traceSource !== "runtime") && (
          <CcNotice
            tone="warn"
            title="Showing live APIs — some sections use defaults until a program runs"
          >
            Load <code>--program</code> with autonomy policy blocks and trigger reflexes (or{" "}
            <code>spanda reflex trace …</code>) for runtime traces. Policy badges below mark{" "}
            <code>program</code> vs <code>platform_defaults</code>.
          </CcNotice>
        )}

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
        <p className="cc-muted">Entities in registry: {entityCount}</p>
        {strategicPlanning ? (
          <ControlCenterDataTable
            rows={[
              {
                key: "entities",
                label: "Entity inventory",
                value: String(entityCount),
              },
              {
                key: "governance",
                label: "Governance payload",
                value: strategicPlanning.governance ? "loaded" : "unavailable",
              },
              {
                key: "profiles",
                label: "Deployment profiles",
                value: strategicPlanning.deployment_profiles ? "loaded" : "unavailable",
              },
            ]}
            rowKey={(row) => String(row.key)}
            columns={[
              { key: "label", header: "Signal", render: (row) => String(row.label) },
              { key: "value", header: "Status", render: (row) => String(row.value) },
            ]}
          />
        ) : (
          <CcEmptyState title="No strategic planning data" />
        )}
      </CcSection>

      <CcSection
        title="Reflex catalog"
        hint="Platform reflex actions from GET /v1/autonomy/reflex (definitions, not events)."
      >
        {reflexes.length === 0 ? (
          <CcEmptyState title="No reflex actions registered" />
        ) : (
          <ControlCenterDataTable
            rows={reflexes.slice(0, 20)}
            rowKey={(row, index) => String(row.id ?? index)}
            columns={[
              { key: "id", header: "ID", render: (row) => String(row.id ?? "—") },
              { key: "trigger", header: "Trigger", render: (row) => String(row.trigger ?? "—") },
              { key: "action", header: "Action", render: (row) => String(row.action ?? "—") },
              {
                key: "priority",
                header: "Priority",
                render: (row) => String(row.priority ?? "—"),
              },
            ]}
          />
        )}
      </CcSection>

      <CcSection
        title="Reflex events"
        hint="Runtime traces only — catalog examples stay opt-in."
        actions={
          <>
            <CcBadge tone={useCatalog ? "info" : sourceTone(traceSource)}>
              {useCatalog ? "source: catalog (demo)" : `source: ${traceSource}`}
            </CcBadge>
            {!demoMode && catalogExamples.length > 0 && traces.length === 0 && (
              <button type="button" className="secondary" onClick={() => setShowCatalog((v) => !v)}>
                {showCatalog ? "Hide catalog examples" : "Show catalog examples"}
              </button>
            )}
          </>
        }
      >
        {useCatalog && (
          <p className="demo-hint">
            Catalog examples are illustrative — not recorded runtime events.
          </p>
        )}
        {visibleTraces.length === 0 ? (
          <CcEmptyState
            title="No runtime reflex traces"
            description="Trigger a reflex on a running program, or turn on Demo mode in the header."
          />
        ) : (
          <ControlCenterDataTable
            rows={visibleTraces}
            rowKey={(row, index) => `${row.reflex_id ?? "r"}-${row.timestamp ?? index}`}
            columns={[
              { key: "reflex", header: "Reflex", render: (row) => String(row.reflex_id ?? "—") },
              { key: "entity", header: "Entity", render: (row) => String(row.entity_id ?? "—") },
              { key: "trigger", header: "Trigger", render: (row) => String(row.trigger ?? "—") },
              {
                key: "action",
                header: "Action",
                render: (row) => String(row.action_taken ?? "—"),
              },
              {
                key: "time",
                header: "Timestamp",
                render: (row) => String(row.timestamp ?? "—"),
              },
            ]}
          />
        )}
      </CcSection>

      <CcSection
        title="Attention Queue"
        hint="Attention Engine — prioritized signals; mission and safety events surface first."
        actions={
          <CcBadge tone={sourceTone(attentionPolicySource)}>
            policy: {attentionPolicySource}
          </CcBadge>
        }
      >
        {attention.length === 0 ? (
          <CcEmptyState
            title="Attention queue empty"
            description="Events appear when entities emit attention signals under the active policy."
          />
        ) : (
          <ControlCenterDataTable
            rows={attention.slice(0, 20)}
            rowKey={(row, index) => String(row.id ?? row.event_id ?? index)}
            columns={[
              {
                key: "id",
                header: "Event",
                render: (row) => String(row.id ?? row.event_id ?? row.kind ?? "—"),
              },
              {
                key: "priority",
                header: "Priority",
                render: (row) => String(row.priority ?? row.score ?? "—"),
              },
              {
                key: "summary",
                header: "Summary",
                render: (row) => String(row.summary ?? row.message ?? row.kind ?? "—"),
              },
            ]}
          />
        )}
      </CcSection>

      <CcSection
        title="Homeostasis"
        hint="Stability reports from entity health, readiness, and scheduler telemetry."
        actions={
          <CcBadge tone={sourceTone(homeoPolicySource)}>policy: {homeoPolicySource}</CcBadge>
        }
      >
        {homeostasis.length === 0 ? (
          <CcEmptyState title="No homeostasis reports" description="Register entities via --config." />
        ) : (
          <ControlCenterDataTable
            rows={homeostasis.slice(0, 20)}
            rowKey={(row, index) => String(row.entity_id ?? index)}
            columns={[
              {
                key: "entity",
                header: "Entity",
                render: (row) => String(row.entity_id ?? "—"),
              },
              {
                key: "stable",
                header: "Stable",
                render: (row) => String(row.stable ?? "—"),
              },
              {
                key: "score",
                header: "Score",
                render: (row) => String(row.score ?? row.stability ?? "—"),
              },
            ]}
          />
        )}
      </CcSection>

      <CcSection
        title="Platform Immunity"
        hint="Trust-boundary violations requiring quarantine or isolation."
      >
        {quarantined.length === 0 ? (
          <CcEmptyState title="Nothing quarantined" />
        ) : (
          <ControlCenterDataTable
            rows={quarantined}
            rowKey={(row, index) => String(row.entity_id ?? row.id ?? index)}
            columns={[
              {
                key: "entity",
                header: "Entity",
                render: (row) => String(row.entity_id ?? row.id ?? "—"),
              },
              {
                key: "reason",
                header: "Reason",
                render: (row) => String(row.reason ?? row.cause ?? "—"),
              },
            ]}
          />
        )}
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
        hint="Maintenance windows from GET /v1/autonomy/maintenance/windows."
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
        hint="Harm potential from the first entity autonomy profile (empty until entities exist)."
      >
        {damageRisk == null ? (
          <CcEmptyState title="No entity autonomy profile" description="Register entities via --config." />
        ) : (
          <pre className="cc-code-block">{JSON.stringify(damageRisk, null, 2)}</pre>
        )}
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
        {entityAutonomy == null && recoveryConfidence == null ? (
          <CcEmptyState title="No recovery metrics yet" />
        ) : (
          <pre className="cc-code-block">
            {JSON.stringify(
              {
                platform_score: recoveryConfidence,
                preferred_strategy: preferredStrategy ?? null,
                entity_id: entityAutonomy?.entity_id ?? entityAutonomy?.id ?? null,
              },
              null,
              2,
            )}
          </pre>
        )}
      </CcSection>
    </div>
  );
}
