import { useMemo, useState } from "react";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import type { DeviceRow } from "./DevicesPanel";

const TRANSPORTS = [
  { id: "mdns", label: "mDNS", hint: "Local network broadcast" },
  { id: "subnet", label: "Subnet scan", hint: "IP range sweep" },
  { id: "ble", label: "BLE", hint: "Bluetooth Low Energy" },
  { id: "usb", label: "USB", hint: "Connected peripherals" },
  { id: "can", label: "CAN", hint: "Vehicle / industrial bus" },
  { id: "mqtt", label: "MQTT", hint: "Broker-attached devices" },
  { id: "ros2", label: "ROS 2", hint: "Robot middleware" },
] as const;

type Props = {
  busy?: boolean;
  hasToken: boolean;
  devices: DeviceRow[];
  discoveryResult: Record<string, unknown> | null;
  onDiscover: (transports: string[]) => void;
  onSelectDevice: (id: string) => void;
  onSignIn?: () => void;
};

type RegisteredDevice = {
  id: string;
  transport?: string;
  address?: string;
};

function parseRegistered(result: Record<string, unknown> | null): RegisteredDevice[] {
  if (!result) return [];
  const registered = result.registered;
  if (!Array.isArray(registered)) return [];
  return registered.map((entry) => {
    if (typeof entry === "string") return { id: entry };
    if (entry && typeof entry === "object") {
      const row = entry as Record<string, unknown>;
      return {
        id: String(row.id ?? row.device_id ?? "unknown"),
        transport: row.transport ? String(row.transport) : undefined,
        address: row.address ? String(row.address) : undefined,
      };
    }
    return { id: "unknown" };
  });
}

export function DiscoveryPanel({
  busy,
  hasToken,
  devices,
  discoveryResult,
  onDiscover,
  onSelectDevice,
  onSignIn,
}: Props) {
  const [selectedTransports, setSelectedTransports] = useState<string[]>(
    TRANSPORTS.map((transport) => transport.id),
  );

  const registered = useMemo(() => parseRegistered(discoveryResult), [discoveryResult]);
  const resultCount = Array.isArray(discoveryResult?.results)
    ? discoveryResult.results.length
    : 0;
  const discoveredInPool = devices.filter(
    (device) => device.lifecycle_state.toLowerCase() === "discovered",
  );

  const toggleTransport = (id: string) => {
    setSelectedTransports((current) =>
      current.includes(id) ? current.filter((value) => value !== id) : [...current, id],
    );
  };

  return (
    <div className="cc-panel">
      <CcMiniStats
        items={[
          { label: "Transports selected", value: selectedTransports.length },
          {
            label: "Last scan hits",
            value: resultCount,
            tone: resultCount > 0 ? "ok" : "neutral",
          },
          {
            label: "Registered",
            value: registered.length,
            tone: registered.length > 0 ? "ok" : "neutral",
          },
          {
            label: "Awaiting provision",
            value: discoveredInPool.length,
            tone: discoveredInPool.length > 0 ? "warn" : "ok",
          },
        ]}
      />

      <CcSection
        title="Scan transports"
        hint="Select protocols to scan, then run discovery. Requires a Bearer token with Provision permission."
        actions={
          <button
            type="button"
            className="primary"
            onClick={() => onDiscover(selectedTransports)}
            disabled={busy || !hasToken || selectedTransports.length === 0}
          >
            {busy ? "Scanning…" : "Discover devices"}
          </button>
        }
      >
        {!hasToken && (
          <CcEmptyState
            title="Sign in to run discovery"
            description="Discovery registers new hardware in the device pool. Paste an API key with Provision permission."
            action={
              onSignIn ? (
                <button type="button" onClick={onSignIn}>
                  Sign in
                </button>
              ) : undefined
            }
          />
        )}

        <div className="cc-transport-grid">
          {TRANSPORTS.map((transport) => {
            const checked = selectedTransports.includes(transport.id);
            return (
              <label
                key={transport.id}
                className={`cc-transport-card${checked ? " selected" : ""}`}
              >
                <input
                  type="checkbox"
                  checked={checked}
                  onChange={() => toggleTransport(transport.id)}
                  disabled={!hasToken}
                />
                <span className="cc-transport-label">{transport.label}</span>
                <span className="cc-transport-hint">{transport.hint}</span>
              </label>
            );
          })}
        </div>
      </CcSection>

      <CcSection
        title="Newly registered"
        hint="Devices added to the pool from the last scan."
      >
        {registered.length === 0 ? (
          <CcEmptyState
            title="No devices registered yet"
            description="Run a discovery scan to find hardware on your selected transports."
          />
        ) : (
          <ul className="cc-card-list">
            {registered.map((device) => (
              <li key={device.id} className="cc-card-item cc-card-item--action">
                <div>
                  <span className="cc-card-item-title">{device.id}</span>
                  <span className="cc-card-item-meta">
                    {[device.transport, device.address].filter(Boolean).join(" · ") || "Registered"}
                  </span>
                </div>
                <button type="button" onClick={() => onSelectDevice(device.id)}>
                  Provision
                </button>
              </li>
            ))}
          </ul>
        )}
      </CcSection>

      {discoveredInPool.length > 0 && (
        <CcSection title="Discovered in pool" hint="Devices waiting for provisioning.">
          <ul className="cc-card-list">
            {discoveredInPool.map((device) => (
              <li key={device.id} className="cc-card-item cc-card-item--action">
                <div>
                  <span className="cc-card-item-title">{device.id}</span>
                  <span className="cc-card-item-meta">{device.device_type}</span>
                </div>
                <button type="button" onClick={() => onSelectDevice(device.id)}>
                  Provision
                </button>
              </li>
            ))}
          </ul>
        </CcSection>
      )}
    </div>
  );
}
