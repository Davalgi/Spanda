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
  const [recoveryConfidence, setRecoveryConfidence] = useState<number | null>(null);
  const [entityAutonomy, setEntityAutonomy] = useState<Record<string, unknown> | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [reflexPayload, tracePayload, homeoPayload, immunityPayload, attentionPayload, metricsPayload] =
        await Promise.all([
          fetchJson(baseUrl, "/v1/autonomy/reflex", authHeaders),
          fetchJson(baseUrl, "/v1/autonomy/reflex/traces", authHeaders),
          fetchJson(baseUrl, "/v1/autonomy/homeostasis", authHeaders),
          fetchJson(baseUrl, "/v1/autonomy/immunity", authHeaders),
          fetchJson(baseUrl, "/v1/autonomy/attention", authHeaders),
          fetchJson(baseUrl, "/v1/recovery/metrics", authHeaders),
        ]);
      setReflexes(asArray((reflexPayload as { reflexes?: unknown }).reflexes));
      setTraces(asArray((tracePayload as { traces?: unknown }).traces));
      setHomeostasis(asArray((homeoPayload as { reports?: unknown }).reports));
      setQuarantined(asArray((immunityPayload as { quarantined?: unknown }).quarantined));
      const attentionWindow = (attentionPayload as { attention?: { items?: unknown } }).attention;
      setAttention(asArray(attentionWindow?.items));
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
        <CcEmptyState title="Loading resilient autonomy…" hint="Fetching /v1/autonomy endpoints." />
      </div>
    );
  }

  const unstableCount = homeostasis.filter(
    (row) => typeof row === "object" && row && (row as { stable?: boolean }).stable === false,
  ).length;

  return (
    <div className="cc-panel">
      <header className="cc-panel-header">
        <h2>Resilient Autonomy</h2>
        <p className="cc-panel-subtitle">
          Live bio-inspired resilience — reflex, attention, homeostasis, immunity, memory, recovery
          confidence, and damage risk from REST.
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
          { label: "Reflex traces", value: traces.length },
          { label: "Unstable entities", value: unstableCount, tone: unstableCount > 0 ? "warn" : "ok" },
          { label: "Quarantined", value: quarantined.length, tone: quarantined.length > 0 ? "danger" : "ok" },
          {
            label: "Recovery confidence",
            value: recoveryConfidence != null ? `${(recoveryConfidence * 100).toFixed(0)}%` : "—",
          },
        ]}
      />

      <CcSection title="Reflex Events" hint="Layer-0 safety actions from /v1/autonomy/reflex.">
        <pre className="cc-code-block">{JSON.stringify(reflexes.slice(0, 5), null, 2)}</pre>
      </CcSection>

      <CcSection title="Attention Queue" hint="Prioritized signals from entity health.">
        <pre className="cc-code-block">{JSON.stringify(attention.slice(0, 8), null, 2)}</pre>
      </CcSection>

      <CcSection title="Homeostasis Status" hint="Stability reports from entity health/readiness mapping.">
        <pre className="cc-code-block">{JSON.stringify(homeostasis.slice(0, 5), null, 2)}</pre>
      </CcSection>

      <CcSection title="Immunity / Quarantine" hint="Trust-boundary violations requiring isolation.">
        <pre className="cc-code-block">{JSON.stringify(quarantined, null, 2)}</pre>
      </CcSection>

      <CcSection title="Operational Memory & Damage Risk" hint="First entity autonomy profile.">
        <pre className="cc-code-block">{JSON.stringify(entityAutonomy, null, 2)}</pre>
      </CcSection>
    </div>
  );
}
