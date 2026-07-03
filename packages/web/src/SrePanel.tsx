import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcBadge, CcEmptyState, CcMiniStats, CcSection, severityTone } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

export type SreSummary = {
  availability_percent: number;
  incidents_open?: number;
  mttr_hint_ms?: number | null;
  mtbf_hint_ms?: number | null;
  slo?: { target_percent?: number; met?: boolean };
  health_trends?: {
    degraded_percent?: number;
    failed_percent?: number;
    offline_percent?: number;
  };
  readiness_trends?: { sample_count?: number; warnings?: string[] };
};

export type IncidentRow = {
  id: string;
  title: string;
  status: string;
  severity: string;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

function incidentTone(status: string): "ok" | "warn" | "danger" | "neutral" {
  const normalized = status.toLowerCase();
  if (normalized === "resolved" || normalized === "closed") return "ok";
  if (normalized === "acknowledged" || normalized === "acked") return "warn";
  return "danger";
}

function formatMs(ms: number | null | undefined): string {
  if (ms === null || ms === undefined) return "—";
  if (ms < 1000) return `${ms} ms`;
  const minutes = Math.round(ms / 60000);
  if (minutes < 60) return `${minutes} min`;
  return `${(minutes / 60).toFixed(1)} h`;
}

export function SrePanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [summary, setSummary] = useState<SreSummary | null>(null);
  const [incidents, setIncidents] = useState<IncidentRow[]>([]);
  const [traces, setTraces] = useState<Record<string, unknown>[]>([]);
  const [grafanaUrl, setGrafanaUrl] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const canOperate = can("Operate");

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [summaryRes, incidentsRes, tracesRes, integrationsRes] = await Promise.all([
        fetch(`${baseUrl}/v1/sre/summary`),
        fetch(`${baseUrl}/v1/sre/incidents`),
        fetch(`${baseUrl}/v1/observability/traces`),
        hasToken
          ? fetch(`${baseUrl}/v1/admin/integrations`, { headers: authHeaders() })
          : Promise.resolve(null),
      ]);
      if (!summaryRes.ok) throw new Error(`sre summary ${summaryRes.status}`);
      setSummary(await summaryRes.json());
      if (incidentsRes.ok) {
        const body = await incidentsRes.json();
        setIncidents(body.incidents ?? []);
      }
      if (tracesRes.ok) {
        const body = await tracesRes.json();
        setTraces((body.traces ?? []).slice(-5));
      }
      if (integrationsRes && integrationsRes.ok) {
        const body = await integrationsRes.json();
        const url = (body.observability as { grafana_url?: string } | undefined)?.grafana_url;
        setGrafanaUrl(url?.trim() ? url : null);
      } else {
        setGrafanaUrl(null);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl, hasToken]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const createIncident = async () => {
    if (!hasToken || !canOperate) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/sre/incidents`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({
          title: "Control Center incident",
          description: "Opened from @davalgi-spanda/web panel",
          severity: "warning",
        }),
      });
      if (!res.ok) throw new Error(`create incident ${res.status}`);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const ackIncident = async (incidentId: string) => {
    if (!hasToken || !canOperate) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/sre/incidents/${encodeURIComponent(incidentId)}/ack`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ assignee: "operator" }),
      });
      if (!res.ok) throw new Error(`ack incident ${res.status}`);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const resolveIncident = async (incidentId: string) => {
    if (!hasToken || !canOperate) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(
        `${baseUrl}/v1/sre/incidents/${encodeURIComponent(incidentId)}/resolve`,
        { method: "POST", headers: authHeaders() },
      );
      if (!res.ok) throw new Error(`resolve incident ${res.status}`);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const sloMet = summary?.slo?.met;
  const openIncidents =
    summary?.incidents_open ?? incidents.filter((i) => i.status !== "resolved").length;

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      {summary ? (
        <CcMiniStats
          items={[
            {
              label: "Availability",
              value: `${summary.availability_percent.toFixed(1)}%`,
              tone:
                summary.availability_percent >= (summary.slo?.target_percent ?? 99) ? "ok" : "warn",
            },
            {
              label: "SLO target",
              value: `${(summary.slo?.target_percent ?? 99).toFixed(1)}%`,
            },
            {
              label: "SLO status",
              value: sloMet === undefined ? "—" : sloMet ? "Met" : "Breached",
              tone: sloMet === undefined ? "neutral" : sloMet ? "ok" : "danger",
            },
            {
              label: "Open incidents",
              value: openIncidents,
              tone: openIncidents > 0 ? "danger" : "ok",
            },
            { label: "MTTR hint", value: formatMs(summary.mttr_hint_ms) },
            { label: "MTBF hint", value: formatMs(summary.mtbf_hint_ms) },
          ]}
        />
      ) : busy ? (
        <CcEmptyState title="Loading SRE summary…" />
      ) : (
        <CcEmptyState
          title="SRE data unavailable"
          description="Could not load summary from the Control Center API."
        />
      )}

      {summary?.health_trends && (
        <CcSection title="Health trends" hint="Pool degradation over the observation window.">
          <CcMiniStats
            items={[
              {
                label: "Degraded",
                value: `${summary.health_trends.degraded_percent?.toFixed(1) ?? "0"}%`,
                tone: (summary.health_trends.degraded_percent ?? 0) > 0 ? "warn" : "ok",
              },
              {
                label: "Failed",
                value: `${summary.health_trends.failed_percent?.toFixed(1) ?? "0"}%`,
                tone: (summary.health_trends.failed_percent ?? 0) > 0 ? "danger" : "ok",
              },
              {
                label: "Offline",
                value: `${summary.health_trends.offline_percent?.toFixed(1) ?? "0"}%`,
                tone: (summary.health_trends.offline_percent ?? 0) > 0 ? "warn" : "ok",
              },
            ]}
          />
        </CcSection>
      )}

      <CcSection
        title="Incidents"
        hint="Track and resolve operational incidents."
        actions={
          <div className="cc-action-bar">
            <button
              type="button"
              onClick={() => void createIncident()}
              disabled={busy || !hasToken || !canOperate}
            >
              Open incident
            </button>
          </div>
        }
      >
        {incidents.length === 0 ? (
          <CcEmptyState
            title="No incidents"
            description="Incidents opened from alerts, SRE checks, or manual triage appear here."
          />
        ) : (
          <ul className="cc-incident-list">
            {incidents.map((incident) => (
              <li key={incident.id} className="cc-incident-item">
                <div className="cc-incident-header">
                  <span className="cc-incident-title">{incident.title}</span>
                  <CcBadge tone={incidentTone(incident.status)}>{incident.status}</CcBadge>
                  <CcBadge tone={severityTone(incident.severity)}>{incident.severity}</CcBadge>
                </div>
                <div className="cc-incident-actions">
                  {incident.status === "open" && hasToken && canOperate && (
                    <button
                      type="button"
                      onClick={() => void ackIncident(incident.id)}
                      disabled={busy}
                    >
                      Acknowledge
                    </button>
                  )}
                  {incident.status !== "resolved" && hasToken && canOperate && (
                    <button
                      type="button"
                      onClick={() => void resolveIncident(incident.id)}
                      disabled={busy}
                    >
                      Resolve
                    </button>
                  )}
                </div>
              </li>
            ))}
          </ul>
        )}
      </CcSection>

      <CcSection title="Recent traces" hint="Last five observability traces from the API.">
        {traces.length === 0 ? (
          <CcEmptyState
            title="No traces recorded"
            description="Pass X-Correlation-ID on API requests to populate traces."
          />
        ) : (
          <ul className="cc-trace-list">
            {traces.map((trace, index) => (
              <li key={index} className="cc-trace-item">
                <code>{JSON.stringify(trace)}</code>
              </li>
            ))}
          </ul>
        )}
      </CcSection>

      <CcSection title="Observability bridge" hint="Grafana templates, Jaeger traces, and OTLP export.">
        <ul className="cc-link-list">
          <li>
            <a href="https://grafana.com/grafana/dashboards/" target="_blank" rel="noreferrer noopener">
              Grafana dashboard templates (spanda-grafana-dashboards)
            </a>
          </li>
          <li>
            <a href={`${baseUrl}/v1/observability/traces`} target="_blank" rel="noreferrer">
              Open traces JSON
            </a>
          </li>
          <li>
            <a href={`${baseUrl}/v1/version`} target="_blank" rel="noreferrer">
              API version &amp; gRPC reflection policy
            </a>
          </li>
        </ul>
        {(summary as { burn_rate?: { fast_burn?: boolean } })?.burn_rate?.fast_burn && (
          <p className="cc-burn-banner">Fast-burn SLO alert — investigate incidents immediately.</p>
        )}
        {grafanaUrl && (
          <iframe
            className="cc-grafana-embed"
            title="Grafana dashboard"
            src={grafanaUrl}
            loading="lazy"
            referrerPolicy="no-referrer"
          />
        )}
      </CcSection>
    </div>
  );
}
