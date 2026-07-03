import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { ControlCenterDataTable } from "./controlCenterDataTable";

type Props = {
  baseUrl: string;
};

export function SmartSpacesPanel({ baseUrl }: Props) {
  const [summary, setSummary] = useState<Record<string, unknown> | null>(null);
  const [readiness, setReadiness] = useState<Record<string, unknown> | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const facility = "tower-demo";
      const [
        summaryRes,
        readinessRes,
        dashboardRes,
        occupancyRes,
        devicesRes,
        healthRes,
        securityRes,
        environmentRes,
        energyRes,
        floorMapRes,
      ] = await Promise.all([
        fetch(`${baseUrl}/v1/smart-spaces/summary`),
        fetch(`${baseUrl}/v1/facilities/${facility}/readiness`).catch(() =>
          fetch(`${baseUrl}/v1/facilities/home-demo/readiness`),
        ),
        fetch(`${baseUrl}/v1/dashboard`),
        fetch(`${baseUrl}/v1/zones/floor-12/occupancy`).catch(() =>
          fetch(`${baseUrl}/v1/zones/room-living/occupancy`),
        ),
        fetch(`${baseUrl}/v1/smart-spaces/devices?facility_id=${facility}`),
        fetch(`${baseUrl}/v1/facilities/${facility}/health`),
        fetch(`${baseUrl}/v1/facilities/${facility}/security`),
        fetch(`${baseUrl}/v1/zones/room-lobby/environment`),
        fetch(`${baseUrl}/v1/energy/systems/battery-001`),
        fetch(`${baseUrl}/v1/facilities/${facility}/floor-map`),
      ]);
      if (!summaryRes.ok) throw new Error(`smart-spaces summary ${summaryRes.status}`);
      const summaryBody = await summaryRes.json();
      setSummary({
        ...summaryBody,
        dashboard: dashboardRes.ok ? await dashboardRes.json() : null,
        occupancy: occupancyRes.ok ? await occupancyRes.json() : null,
        devices: devicesRes.ok ? await devicesRes.json() : null,
        facilityHealth: healthRes.ok ? await healthRes.json() : null,
        facilitySecurity: securityRes.ok ? await securityRes.json() : null,
        environment: environmentRes.ok ? await environmentRes.json() : null,
        energyDetail: energyRes.ok ? await energyRes.json() : null,
        floorMap: floorMapRes.ok ? await floorMapRes.json() : null,
      });
      if (readinessRes.ok) setReadiness(await readinessRes.json());
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  if (!summary && !busy) {
    return (
      <div className="cc-panel">
        {error && <div className="error">{error}</div>}
        <CcEmptyState
          title="Smart Spaces data unavailable"
          description="Serve with examples/solutions/smart-spaces/spanda.toml and smart-building/floor_readiness.sd"
        />
      </div>
    );
  }

  const facilities = (summary?.facilities as Record<string, unknown>) ?? {};
  const energy = (summary?.energy as Record<string, unknown>) ?? {};
  const emergency = (summary?.emergency as Record<string, unknown>) ?? {};
  const dashboard = summary?.dashboard as Record<string, unknown> | null;
  const pool = (dashboard?.device_pool as Record<string, unknown>) ?? {};
  const facilityRows = (facilities.facilities as Record<string, unknown>[]) ?? [];
  const gatewayRows = (facilities.gateways as Record<string, unknown>[]) ?? [];
  const zoneRows = (facilities.zones as Record<string, unknown>[]) ?? [];
  const energyRows = (energy.systems as Record<string, unknown>[]) ?? [];
  const continuity = (emergency.continuity_pairs as Record<string, unknown>[]) ?? [];
  const occupancy = summary?.occupancy as Record<string, unknown> | null;
  const robotRows = (summary?.robots as Record<string, unknown>[]) ?? [];
  const wearableRows = (summary?.wearables as Record<string, unknown>[]) ?? [];
  const trustRows = (summary?.trust_entries as Record<string, unknown>[]) ?? [];
  const deviceRows =
    ((summary?.devices as Record<string, unknown> | null)?.devices as
      | Record<string, unknown>[]
      | undefined) ?? [];
  const facilityHealth = summary?.facilityHealth as Record<string, unknown> | null;
  const facilitySecurity = summary?.facilitySecurity as Record<string, unknown> | null;
  const environment = summary?.environment as Record<string, unknown> | null;
  const energyDetail = summary?.energyDetail as Record<string, unknown> | null;
  const floorMap = summary?.floorMap as Record<string, unknown> | null;
  const floorZones = (floorMap?.zones as Record<string, unknown>[]) ?? [];
  const factorChart =
    (readiness?.factors as Record<string, unknown>[]) ??
    (readiness?.factor_chart as Record<string, unknown>[]) ??
    [];

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Smart building overview"
        hint="Facilities, zones, energy, and occupancy for smart-space programs."
        actions={
          <button type="button" onClick={() => void load()} disabled={busy}>
            Refresh
          </button>
        }
      >
        <CcMiniStats
          items={[
            { label: "Facilities", value: String(facilities.count ?? facilityRows.length) },
            { label: "Gateways", value: gatewayRows.length },
            { label: "Zones", value: zoneRows.length },
            { label: "Energy systems", value: String(energy.count ?? energyRows.length) },
            { label: "Pool healthy", value: String(pool.healthy ?? "—") },
            { label: "Emergency", value: String(emergency.status ?? "normal") },
          ]}
        />
      </CcSection>

      <CcSection title="Buildings">
        <ControlCenterDataTable
          rows={facilityRows}
          rowKey={(row) => String(row.id)}
          columns={[
            { key: "id", header: "ID", render: (row) => String(row.id) },
            { key: "name", header: "Name", render: (row) => String(row.name ?? "—") },
            { key: "type", header: "Type", render: (row) => String(row.facility_type ?? "—") },
          ]}
        />
      </CcSection>

      <CcSection title="Gateways">
        <ControlCenterDataTable
          rows={gatewayRows}
          rowKey={(row) => String(row.id)}
          columns={[
            { key: "id", header: "ID", render: (row) => String(row.id) },
            { key: "role", header: "Role", render: (row) => String(row.role ?? "—") },
            { key: "provider", header: "Provider", render: (row) => String(row.provider ?? "—") },
            { key: "failover", header: "Failover", render: (row) => String(row.failover_from ?? "—") },
          ]}
        />
      </CcSection>

      <CcSection title="Rooms & zones">
        <ControlCenterDataTable
          rows={zoneRows}
          rowKey={(row) => String(row.id)}
          columns={[
            { key: "id", header: "ID", render: (row) => String(row.id) },
            { key: "name", header: "Name", render: (row) => String(row.name ?? "—") },
            { key: "facility", header: "Facility", render: (row) => String(row.facility ?? "—") },
            { key: "type", header: "Type", render: (row) => String(row.type ?? "—") },
          ]}
        />
      </CcSection>

      <CcSection title="Energy systems">
        <ControlCenterDataTable
          rows={energyRows}
          rowKey={(row) => String(row.id)}
          columns={[
            { key: "id", header: "ID", render: (row) => String(row.id) },
            { key: "type", header: "Type", render: (row) => String(row.type ?? "—") },
            { key: "provider", header: "Provider", render: (row) => String(row.provider ?? "—") },
          ]}
        />
      </CcSection>

      <CcSection title="Robots">
        <ControlCenterDataTable
          rows={robotRows}
          rowKey={(row) => String(row.id)}
          columns={[
            { key: "id", header: "ID", render: (row) => String(row.id) },
            { key: "facility", header: "Facility", render: (row) => String(row.facility ?? "—") },
            { key: "type", header: "Type", render: (row) => String(row.type ?? "—") },
            { key: "provider", header: "Provider", render: (row) => String(row.provider ?? "—") },
          ]}
        />
      </CcSection>

      <CcSection title="Wearables">
        <ControlCenterDataTable
          rows={wearableRows}
          rowKey={(row) => String(row.id)}
          columns={[
            { key: "id", header: "ID", render: (row) => String(row.id) },
            { key: "human", header: "Human", render: (row) => String(row.human_id ?? "—") },
            { key: "type", header: "Type", render: (row) => String(row.type ?? "—") },
            { key: "provider", header: "Provider", render: (row) => String(row.provider ?? "—") },
          ]}
        />
      </CcSection>

      {occupancy && (
        <CcSection title="Occupancy & flow">
          <pre className="cc-action-result">{JSON.stringify(occupancy.occupancy, null, 2)}</pre>
          <ControlCenterDataTable
            rows={(occupancy.timeline as Record<string, unknown>[]) ?? []}
            rowKey={(_, index) => String(index)}
            emptyLabel="No timeline data"
            columns={[
              {
                key: "offset",
                header: "Offset (min)",
                render: (row) => String(row.offset_minutes),
              },
              { key: "count", header: "Count", render: (row) => String(row.count) },
              { key: "flow", header: "Flow", render: (row) => String(row.flow) },
            ]}
          />
        </CcSection>
      )}

      <CcSection title="Device inventory">
        <ControlCenterDataTable
          rows={deviceRows}
          rowKey={(row) => String(row.id)}
          columns={[
            { key: "id", header: "ID", render: (row) => String(row.id) },
            { key: "type", header: "Type", render: (row) => String(row.type ?? "—") },
            { key: "zone", header: "Zone", render: (row) => String(row.zone ?? "—") },
            { key: "health", header: "Health", render: (row) => String(row.health_status ?? "—") },
          ]}
        />
      </CcSection>

      {facilityHealth && (
        <CcSection title="Device pool health">
          <p className="cc-section-hint">
            Overall: <strong>{String(facilityHealth.overall_status)}</strong>
          </p>
          <pre className="cc-action-result">
            {JSON.stringify(facilityHealth.device_pool, null, 2)}
          </pre>
        </CcSection>
      )}

      {facilitySecurity && (
        <CcSection title="Security & access">
          <p className="cc-section-hint">
            Lockdown: {String(facilitySecurity.lockdown_active)} · Package trust min:{" "}
            {String(facilitySecurity.package_trust_min ?? "—")}
          </p>
          <pre className="cc-action-result">
            {JSON.stringify(facilitySecurity.locks_and_cameras, null, 2)}
          </pre>
        </CcSection>
      )}

      {environment && (
        <CcSection title="Environmental (room-lobby)">
          <pre className="cc-action-result">{JSON.stringify(environment.readings, null, 2)}</pre>
        </CcSection>
      )}

      {energyDetail && (
        <CcSection title="Energy detail (battery-001)">
          <pre className="cc-action-result">{JSON.stringify(energyDetail.detail, null, 2)}</pre>
        </CcSection>
      )}

      {floorZones.length > 0 && (
        <CcSection title="Floor map">
          <div className="cc-floor-plan">
            <svg viewBox="0 0 100 60" className="cc-floor-plan-svg" role="img" aria-label="Floor plan">
              {floorZones.map((row, index) => {
                const zone = (row.zone as Record<string, unknown>) ?? {};
                const cols = Math.ceil(Math.sqrt(floorZones.length));
                const col = index % cols;
                const rowIdx = Math.floor(index / cols);
                const w = 100 / cols - 2;
                const h = 60 / Math.ceil(floorZones.length / cols) - 2;
                const x = col * (w + 2) + 1;
                const y = rowIdx * (h + 2) + 1;
                const occupancy = Number(row.occupancy_count ?? 0);
                return (
                  <g key={String(zone.id)}>
                    <rect
                      x={x}
                      y={y}
                      width={w}
                      height={h}
                      className={`cc-floor-zone${occupancy > 0 ? " occupied" : ""}`}
                    />
                    <text x={x + 2} y={y + h / 2} className="cc-floor-zone-label">
                      {String(zone.name ?? zone.id).slice(0, 8)}
                    </text>
                  </g>
                );
              })}
            </svg>
          </div>
          <ControlCenterDataTable
            rows={floorZones}
            rowKey={(row) => {
              const zone = (row.zone as Record<string, unknown>) ?? {};
              return String(zone.id);
            }}
            columns={[
              {
                key: "zone",
                header: "Zone",
                render: (row) => {
                  const zone = (row.zone as Record<string, unknown>) ?? {};
                  return String(zone.name ?? zone.id);
                },
              },
              {
                key: "parent",
                header: "Parent",
                render: (row) => {
                  const zone = (row.zone as Record<string, unknown>) ?? {};
                  return String(zone.parent ?? "—");
                },
              },
              {
                key: "devices",
                header: "Devices",
                render: (row) => String(row.device_count ?? 0),
              },
            ]}
          />
        </CcSection>
      )}

      {readiness && (
        <CcSection
          title={`Facility readiness — ${String(readiness.score ?? "—")}/100 (${String(readiness.status ?? "unknown")})`}
        >
          <div className="trust-chart">
            {factorChart.map((factor) => {
              const score = Number(factor.score ?? 0);
              const dimension = String(factor.dimension ?? "factor");
              return (
                <div key={dimension} className="trust-chart-row">
                  <span className="trust-chart-label">{dimension}</span>
                  <div className="trust-chart-bar">
                    <div
                      className="trust-chart-fill"
                      style={{ width: `${Math.min(100, Math.max(0, score))}%` }}
                    />
                  </div>
                  <span className="trust-chart-score">{score}</span>
                </div>
              );
            })}
          </div>
          <ControlCenterDataTable
            rows={trustRows}
            rowKey={(row) => String(row.id)}
            columns={[
              { key: "id", header: "Device", render: (row) => String(row.id) },
              { key: "trust", header: "Trust", render: (row) => String(row.trust_level ?? "—") },
              {
                key: "health",
                header: "Health",
                render: (row) => String(row.health_status ?? "—"),
              },
              { key: "provider", header: "Provider", render: (row) => String(row.provider ?? "—") },
            ]}
          />
        </CcSection>
      )}

      <CcSection title="Emergency & continuity">
        <ControlCenterDataTable
          rows={continuity}
          rowKey={(_, index) => String(index)}
          columns={[
            { key: "primary", header: "Primary", render: (row) => String(row.primary ?? "—") },
            { key: "backup", header: "Backup", render: (row) => String(row.backup ?? "—") },
            {
              key: "failure",
              header: "On failure",
              render: (row) => String(row.on_failure ?? "—"),
            },
          ]}
        />
      </CcSection>
    </div>
  );
}
