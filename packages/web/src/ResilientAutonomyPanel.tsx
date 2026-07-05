import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
};

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
  const [reflexes, setReflexes] = useState<unknown[]>([]);
  const [traces, setTraces] = useState<unknown[]>([]);
  const [homeostasis, setHomeostasis] = useState<unknown[]>([]);
  const [quarantined, setQuarantined] = useState<unknown[]>([]);
  const [attention, setAttention] = useState<unknown[]>([]);
  const [memory, setMemory] = useState<unknown[]>([]);
  const [recoveryConfidence, setRecoveryConfidence] = useState<number | null>(null);
  const [entityAutonomy, setEntityAutonomy] = useState<Record<string, unknown> | null>(null);
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
      ] = await Promise.all([
        fetchJson(baseUrl, "/v1/autonomy/reflex", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/reflex/traces", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/homeostasis", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/immunity", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/attention", authHeaders),
        fetchJson(baseUrl, "/v1/autonomy/memory", authHeaders),
        fetchJson(baseUrl, "/v1/recovery/metrics", authHeaders),
      ]);
      setReflexes(asArray((reflexPayload as { reflexes?: unknown }).reflexes));
      setTraces(asArray((tracePayload as { traces?: unknown }).traces));
      setHomeostasis(asArray((homeoPayload as { reports?: unknown }).reports));
      setQuarantined(asArray((immunityPayload as { quarantined?: unknown }).quarantined));
      const attentionWindow = (attentionPayload as { attention?: { items?: unknown } }).attention;
      setAttention(asArray(attentionWindow?.items));
      setMemory(asArray((memoryPayload as { memory?: unknown }).memory));
      const metrics = metricsPayload as { recovery_confidence?: number };
      setRecoveryConfidence(
        typeof metrics.recovery_confidence === "number" ? metrics.recovery_confidence : null,
      );

      const entitiesPayload = (await fetchJson(baseUrl, "/v1/entities", authHeaders)) as {
        entities?: Array<{ id?: string }>;
      };
      const firstId = entitiesPayload.entities?.[0]?.id;
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

  if (loading && !reflexes.length && !error) {
    return (
      <div className="cc-panel">
        <CcEmptyState
          title="Loading Cognitive & Resilience…"
          hint="Fetching functional domain endpoints under /v1/autonomy/*."
        />
      </div>
    );
  }

  const unstableCount = homeostasis.filter(
    (row) => typeof row === "object" && row && (row as { stable?: boolean }).stable === false,
  ).length;

  const autonomyProfile = entityAutonomy?.autonomy as Record<string, unknown> | undefined;
  const damageRisk = autonomyProfile?.damage_risk;

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
        hint="Memory category refs across entities — working, episodic, semantic, procedural, reflex."
      >
        <pre className="cc-code-block">{JSON.stringify(memory.slice(0, 5), null, 2)}</pre>
      </CcSection>

      <CcSection
        title="Damage Risk"
        hint="Damage Risk Assessment — harm potential from first entity autonomy profile."
      >
        <pre className="cc-code-block">{JSON.stringify(damageRisk ?? {}, null, 2)}</pre>
      </CcSection>

      <CcSection
        title="Recovery Confidence"
        hint="Adaptive Learning domain — strategy confidence from recovery history."
      >
        <pre className="cc-code-block">
          {JSON.stringify(
            {
              platform_score: recoveryConfidence,
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
