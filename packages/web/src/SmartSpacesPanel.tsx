/** Smart Spaces Control Center panel — inventory live; telemetry labeled when simulated. @module */

import { useCallback, useEffect, useMemo, useState } from "react";
import { CcBadge, CcEmptyState, CcMiniStats, CcNotice, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";
import { ControlCenterDataTable } from "./controlCenterDataTable";

type Props = {
  baseUrl: string;
};

function rowId(row: Record<string, unknown> | undefined): string | null {
  const id = row?.id;
  return typeof id === "string" && id.length > 0 ? id : null;
}

function isSimulatedSource(source: unknown): boolean {
  // Treat profile/seed backends as simulated for honesty badges.
  if (typeof source !== "string") return true;
  return source === "simulated" || source === "zone_profile" || source === "twin_profile";
}

export function SmartSpacesPanel({ baseUrl }: Props) {
  // Load facility inventory from summary, then detail endpoints for selected ids.
  //
  // Parameters:
  // - `baseUrl` — Control Center API base URL
  //
  // Returns:
  // Smart Spaces panel element.
  //
  // Options:
  // None.
  //
  // Example:
  // <SmartSpacesPanel baseUrl={url} />

  const [summary, setSummary] = useState<Record<string, unknown> | null>(null);
  const [readiness, setReadiness] = useState<Record<string, unknown> | null>(null);
  const [facilityId, setFacilityId] = useState<string>("");
  const [zoneId, setZoneId] = useState<string>("");
  const [energyId, setEnergyId] = useState<string>("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      // Pull inventory first so facility/zone/energy pickers use real config ids.
      const summaryRes = await fetch(`${baseUrl}/v1/smart-spaces/summary`);
      if (!summaryRes.ok) throw new Error(`smart-spaces summary ${summaryRes.status}`);
      const summaryBody = (await summaryRes.json()) as Record<string, unknown>;
      const facilitiesBlock = (summaryBody.facilities as Record<string, unknown>) ?? {};
      const facilityRows = (facilitiesBlock.facilities as Record<string, unknown>[]) ?? [];
      const zoneRows = (facilitiesBlock.zones as Record<string, unknown>[]) ?? [];
      const energyBlock = (summaryBody.energy as Record<string, unknown>) ?? {};
      const energyRows = (energyBlock.systems as Record<string, unknown>[]) ?? [];

      const nextFacility =
        facilityId && facilityRows.some((row) => rowId(row) === facilityId)
          ? facilityId
          : (rowId(facilityRows[0]) ?? "");
      const zonesForFacility = zoneRows.filter((row) => {
        if (!nextFacility) return true;
        return String(row.facility ?? "") === nextFacility || !row.facility;
      });
      const zonePool = zonesForFacility.length > 0 ? zonesForFacility : zoneRows;
      const nextZone =
        zoneId && zonePool.some((row) => rowId(row) === zoneId)
          ? zoneId
          : (rowId(zonePool[0]) ?? "");
      const nextEnergy =
        energyId && energyRows.some((row) => rowId(row) === energyId)
          ? energyId
          : (rowId(energyRows[0]) ?? "");

      setFacilityId(nextFacility);
      setZoneId(nextZone);
      setEnergyId(nextEnergy);

      // Fetch detail panels only for selected inventory ids (no hardcoded demos).
      const detailFetches: Promise<Response>[] = [
        fetch(`${baseUrl}/v1/dashboard`),
        nextFacility
          ? fetch(`${baseUrl}/v1/facilities/${encodeURIComponent(nextFacility)}/readiness`)
          : Promise.resolve(new Response(null, { status: 404 })),
        nextFacility
          ? fetch(
              `${baseUrl}/v1/smart-spaces/devices?facility_id=${encodeURIComponent(nextFacility)}`,
            )
          : Promise.resolve(new Response(null, { status: 404 })),
        nextFacility
          ? fetch(`${baseUrl}/v1/facilities/${encodeURIComponent(nextFacility)}/health`)
          : Promise.resolve(new Response(null, { status: 404 })),
        nextFacility
          ? fetch(`${baseUrl}/v1/facilities/${encodeURIComponent(nextFacility)}/security`)
          : Promise.resolve(new Response(null, { status: 404 })),
        nextFacility
          ? fetch(`${baseUrl}/v1/facilities/${encodeURIComponent(nextFacility)}/floor-map`)
          : Promise.resolve(new Response(null, { status: 404 })),
        nextZone
          ? fetch(`${baseUrl}/v1/zones/${encodeURIComponent(nextZone)}/occupancy`)
          : Promise.resolve(new Response(null, { status: 404 })),
        nextZone
          ? fetch(`${baseUrl}/v1/zones/${encodeURIComponent(nextZone)}/environment`)
          : Promise.resolve(new Response(null, { status: 404 })),
        nextEnergy
          ? fetch(`${baseUrl}/v1/energy/systems/${encodeURIComponent(nextEnergy)}`)
          : Promise.resolve(new Response(null, { status: 404 })),
      ];
      const [
        dashboardRes,
        readinessRes,
        devicesRes,
        healthRes,
        securityRes,
        floorMapRes,
        occupancyRes,
        environmentRes,
        energyRes,
      ] = await Promise.all(detailFetches);

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
      else setReadiness(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, energyId, facilityId, zoneId]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const facilities = (summary?.facilities as Record<string, unknown>) ?? {};
  const energy = (summary?.energy as Record<string, unknown>) ?? {};
  const emergency = (summary?.emergency as Record<string, unknown>) ?? {};
  const dashboard = summary?.dashboard as Record<string, unknown> | null;
  const pool = (dashboard?.device_pool as Record<string, unknown>) ?? {};
  const facilityRows = useMemo(
    () => (facilities.facilities as Record<string, unknown>[]) ?? [],
    [facilities.facilities],
  );
  const gatewayRows = (facilities.gateways as Record<string, unknown>[]) ?? [];
  const zoneRows = useMemo(
    () => (facilities.zones as Record<string, unknown>[]) ?? [],
    [facilities.zones],
  );
  const energyRows = useMemo(
    () => (energy.systems as Record<string, unknown>[]) ?? [],
    [energy.systems],
  );
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

  const occupancySource =
    ((occupancy?.occupancy as Record<string, unknown> | undefined)?.source as string | undefined) ??
    "zone_profile";
  const environmentSource = (environment?.source as string | undefined) ?? "simulated";
  const energySource = (energyDetail?.source as string | undefined) ?? "simulated";
  const zonesForPicker = zoneRows.filter((row) => {
    if (!facilityId) return true;
    return String(row.facility ?? "") === facilityId || !row.facility;
  });

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

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcNotice tone="warn" title="Inventory is live — detail telemetry may be simulated">
        Facility / zone / device lists come from <code>--config</code>. Occupancy, environment, and
        energy readings are labeled when they use seed / zone-profile backends (not live sensors).
        Live BACnet probes require <code>SPANDA_LIVE_BACNET=1</code>.
      </CcNotice>

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

      <CcSection title="Selection" hint="Detail panels follow the selected facility, zone, and energy system.">
        <div className="cc-facility-picker">
          <label>
            Facility
            <select
              value={facilityId}
              onChange={(e) => {
                setFacilityId(e.target.value);
                setZoneId("");
              }}
              disabled={facilityRows.length === 0}
            >
              {facilityRows.length === 0 ? (
                <option value="">No facilities in config</option>
              ) : (
                facilityRows.map((row) => {
                  const id = rowId(row) ?? "";
                  return (
                    <option key={id} value={id}>
                      {String(row.name ?? id)}
                    </option>
                  );
                })
              )}
            </select>
          </label>
          <label>
            Zone
            <select
              value={zoneId}
              onChange={(e) => setZoneId(e.target.value)}
              disabled={zonesForPicker.length === 0}
            >
              {zonesForPicker.length === 0 ? (
                <option value="">No zones</option>
              ) : (
                zonesForPicker.map((row) => {
                  const id = rowId(row) ?? "";
                  return (
                    <option key={id} value={id}>
                      {String(row.name ?? id)}
                    </option>
                  );
                })
              )}
            </select>
          </label>
          <label>
            Energy system
            <select
              value={energyId}
              onChange={(e) => setEnergyId(e.target.value)}
              disabled={energyRows.length === 0}
            >
              {energyRows.length === 0 ? (
                <option value="">No energy systems</option>
              ) : (
                energyRows.map((row) => {
                  const id = rowId(row) ?? "";
                  return (
                    <option key={id} value={id}>
                      {String(row.type ?? id)} ({id})
                    </option>
                  );
                })
              )}
            </select>
          </label>
          <button type="button" onClick={() => void load()} disabled={busy}>
            Refresh
          </button>
        </div>
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
        <CcSection
          title={`Occupancy & flow — ${zoneId || "—"}`}
          hint="Counts from zone profile / twin seed unless a live occupancy twin is wired."
          actions={
            isSimulatedSource(occupancySource) ? (
              <CcBadge tone="warn">source: {occupancySource}</CcBadge>
            ) : (
              <CcBadge tone="ok">source: {occupancySource}</CcBadge>
            )
          }
        >
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
        <CcSection
          title={`Environmental — ${zoneId || "—"}`}
          actions={
            isSimulatedSource(environmentSource) ? (
              <CcBadge tone="warn">source: {environmentSource}</CcBadge>
            ) : (
              <CcBadge tone="ok">source: {environmentSource}</CcBadge>
            )
          }
        >
          <pre className="cc-action-result">{JSON.stringify(environment.readings, null, 2)}</pre>
        </CcSection>
      )}

      {energyDetail && (
        <CcSection
          title={`Energy detail — ${energyId || "—"}`}
          actions={
            isSimulatedSource(energySource) ? (
              <CcBadge tone="warn">source: {energySource}</CcBadge>
            ) : (
              <CcBadge tone="ok">source: {energySource}</CcBadge>
            )
          }
        >
          <pre className="cc-action-result">{JSON.stringify(energyDetail.detail, null, 2)}</pre>
        </CcSection>
      )}

      {floorZones.length > 0 && (
        <CcSection
          title="Floor map (schematic)"
          hint="Tiled zone schematic from inventory — not a CAD floorplan."
          actions={<CcBadge tone="info">schematic</CcBadge>}
        >
          <div className="cc-floor-plan">
            <svg viewBox="0 0 100 60" className="cc-floor-plan-svg" role="img" aria-label="Floor plan schematic">
              {floorZones.map((row, index) => {
                const zone = (row.zone as Record<string, unknown>) ?? {};
                const cols = Math.ceil(Math.sqrt(floorZones.length));
                const col = index % cols;
                const rowIdx = Math.floor(index / cols);
                const w = 100 / cols - 2;
                const h = 60 / Math.ceil(floorZones.length / cols) - 2;
                const x = col * (w + 2) + 1;
                const y = rowIdx * (h + 2) + 1;
                const occ = Number(row.occupancy_count ?? 0);
                return (
                  <g key={String(zone.id)}>
                    <rect
                      x={x}
                      y={y}
                      width={w}
                      height={h}
                      className={`cc-floor-zone${occ > 0 ? " occupied" : ""}`}
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
