import { useMemo, useState } from "react";
import {
  CcBadge,
  CcEmptyState,
  CcMiniStats,
  CcSection,
  lifecycleTone,
  trustTone,
} from "./controlCenterUi";

export type DeviceRow = {
  id: string;
  device_type: string;
  lifecycle_state: string;
  assigned_robot?: string;
  logical_name?: string;
  trust_level?: string;
};

type Props = {
  devices: DeviceRow[];
  loading?: boolean;
  onInspect: (id: string) => void;
  onDiscover?: () => void;
  baseUrl?: string;
  authHeaders?: () => HeadersInit;
  canBulk?: boolean;
  onBulkComplete?: () => void;
};

export function DevicesPanel({
  devices,
  loading,
  onInspect,
  onDiscover,
  baseUrl,
  authHeaders,
  canBulk,
  onBulkComplete,
}: Props) {
  const [query, setQuery] = useState("");
  const [lifecycleFilter, setLifecycleFilter] = useState("all");
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [bulkBusy, setBulkBusy] = useState(false);

  const lifecycleOptions = useMemo(() => {
    const states = new Set(devices.map((device) => device.lifecycle_state));
    return ["all", ...Array.from(states).sort()];
  }, [devices]);

  const filtered = useMemo(() => {
    const normalized = query.trim().toLowerCase();
    return devices.filter((device) => {
      if (lifecycleFilter !== "all" && device.lifecycle_state !== lifecycleFilter) {
        return false;
      }
      if (!normalized) return true;
      return (
        device.id.toLowerCase().includes(normalized) ||
        device.device_type.toLowerCase().includes(normalized) ||
        (device.logical_name ?? "").toLowerCase().includes(normalized) ||
        (device.assigned_robot ?? "").toLowerCase().includes(normalized)
      );
    });
  }, [devices, lifecycleFilter, query]);

  const stats = useMemo(() => {
    let active = 0;
    let attention = 0;
    let quarantined = 0;
    for (const device of devices) {
      const state = device.lifecycle_state.toLowerCase();
      if (state === "active" || state === "ready") active += 1;
      if (state === "discovered" || state === "degraded") attention += 1;
      if (state === "quarantined" || state === "failed") quarantined += 1;
    }
    return { total: devices.length, active, attention, quarantined };
  }, [devices]);

  const toggleSelect = (id: string) => {
    setSelected((current) => {
      const next = new Set(current);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const bulkAction = async (action: "quarantine" | "trust") => {
    if (!baseUrl || !authHeaders || !canBulk || selected.size === 0) return;
    setBulkBusy(true);
    try {
      await Promise.all(
        Array.from(selected).map((id) =>
          fetch(`${baseUrl}/v1/devices/${encodeURIComponent(id)}/${action}`, {
            method: "POST",
            headers: authHeaders(),
            body: "{}",
          }),
        ),
      );
      setSelected(new Set());
      onBulkComplete?.();
    } finally {
      setBulkBusy(false);
    }
  };

  return (
    <div className="cc-panel">
      <CcMiniStats
        items={[
          { label: "Total", value: stats.total },
          { label: "Active", value: stats.active, tone: "ok" },
          { label: "Needs attention", value: stats.attention, tone: stats.attention > 0 ? "warn" : "ok" },
          { label: "Quarantined / failed", value: stats.quarantined, tone: stats.quarantined > 0 ? "danger" : "ok" },
        ]}
      />

      <CcSection
        title="Device pool"
        hint="Click a device ID to open provisioning."
        actions={
          <div className="cc-filter-bar">
            {canBulk && selected.size > 0 && (
              <>
                <button type="button" onClick={() => void bulkAction("quarantine")} disabled={bulkBusy}>
                  Quarantine ({selected.size})
                </button>
                <button type="button" onClick={() => void bulkAction("trust")} disabled={bulkBusy}>
                  Trust ({selected.size})
                </button>
              </>
            )}
            <input
              type="search"
              value={query}
              onChange={(event) => setQuery(event.target.value)}
              placeholder="Search devices…"
              aria-label="Search devices"
            />
            <select
              value={lifecycleFilter}
              onChange={(event) => setLifecycleFilter(event.target.value)}
              aria-label="Filter by lifecycle"
            >
              {lifecycleOptions.map((state) => (
                <option key={state} value={state}>
                  {state === "all" ? "All lifecycles" : state}
                </option>
              ))}
            </select>
          </div>
        }
      >
        {loading && devices.length === 0 ? (
          <CcEmptyState title="Loading devices…" />
        ) : filtered.length === 0 ? (
          <CcEmptyState
            title={devices.length === 0 ? "No devices in the pool" : "No devices match your filters"}
            description={
              devices.length === 0
                ? "Run discovery to find hardware on your network, then provision devices onto robots."
                : "Try clearing the search or lifecycle filter."
            }
            action={
              devices.length === 0 && onDiscover ? (
                <button type="button" className="primary" onClick={onDiscover}>
                  Go to discovery
                </button>
              ) : undefined
            }
          />
        ) : (
          <div className="cc-table-wrap">
            <table className="cc-data-table">
              <thead>
                <tr>
                  {canBulk && <th aria-label="Select" />}
                  <th>Device</th>
                  <th>Type</th>
                  <th>Lifecycle</th>
                  <th>Trust</th>
                  <th>Robot</th>
                  <th>Logical name</th>
                </tr>
              </thead>
              <tbody>
                {filtered.map((device) => (
                  <tr key={device.id}>
                    {canBulk && (
                      <td>
                        <input
                          type="checkbox"
                          checked={selected.has(device.id)}
                          onChange={() => toggleSelect(device.id)}
                          aria-label={`Select ${device.id}`}
                        />
                      </td>
                    )}
                    <td>
                      <button
                        type="button"
                        className="cc-link-btn"
                        onClick={() => onInspect(device.id)}
                      >
                        {device.id}
                      </button>
                    </td>
                    <td>{device.device_type}</td>
                    <td>
                      <CcBadge tone={lifecycleTone(device.lifecycle_state)}>
                        {device.lifecycle_state}
                      </CcBadge>
                    </td>
                    <td>
                      <CcBadge tone={trustTone(device.trust_level)}>
                        {device.trust_level ?? "unknown"}
                      </CcBadge>
                    </td>
                    <td>{device.assigned_robot ?? "—"}</td>
                    <td>{device.logical_name ?? "—"}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </CcSection>
    </div>
  );
}
