import { useCallback, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import type { ProvisionReport } from "./ProvisioningPanel";

type Options = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  robotId: string;
  onRefresh: () => Promise<void>;
  onOpenProvisioning: () => void;
  setError: (message: string | null) => void;
  setBusy: (busy: boolean) => void;
};

export function useDeviceWorkflow({
  baseUrl,
  authHeaders,
  robotId,
  onRefresh,
  onOpenProvisioning,
  setError,
  setBusy,
}: Options) {
  const [selectedDevice, setSelectedDevice] = useState<string | null>(null);
  const [deviceDetail, setDeviceDetail] = useState<Record<string, unknown> | null>(null);
  const [discoveryResult, setDiscoveryResult] = useState<Record<string, unknown> | null>(null);
  const [provisionReport, setProvisionReport] = useState<ProvisionReport | null>(null);

  const inspectDevice = useCallback(
    async (id: string) => {
      setSelectedDevice(id);
      onOpenProvisioning();
      setProvisionReport(null);
      try {
        const res = await fetch(`${baseUrl}/v1/devices/${encodeURIComponent(id)}`);
        if (res.ok) {
          const body = await res.json();
          setDeviceDetail(body.device ?? null);
        }
      } catch (err) {
        setError(String(err));
      }
    },
    [baseUrl, onOpenProvisioning, setError],
  );

  const runDiscovery = useCallback(
    async (transports: string[]) => {
      setBusy(true);
      setDiscoveryResult(null);
      setError(null);
      try {
        const res = await fetch(`${baseUrl}/v1/devices/discover`, {
          method: "POST",
          headers: authHeaders(),
          body: JSON.stringify({
            transports,
            timeout_ms: 2000,
          }),
        });
        if (!res.ok) throw new Error(`discover ${res.status}`);
        setDiscoveryResult(await res.json());
        await onRefresh();
      } catch (err) {
        setError(String(err));
      } finally {
        setBusy(false);
      }
    },
    [authHeaders, baseUrl, onRefresh, setBusy, setError],
  );

  const provisionDevice = useCallback(async () => {
    if (!selectedDevice) return;
    setBusy(true);
    setProvisionReport(null);
    setError(null);
    try {
      const res = await fetch(
        `${baseUrl}/v1/devices/${encodeURIComponent(selectedDevice)}/provision`,
        {
          method: "POST",
          headers: authHeaders(),
          body: JSON.stringify({ robot_id: robotId }),
        },
      );
      const body = await res.json();
      setProvisionReport((body.report ?? body) as ProvisionReport);
      await onRefresh();
      await inspectDevice(selectedDevice);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl, inspectDevice, onRefresh, robotId, selectedDevice, setBusy, setError]);

  const quarantineDevice = useCallback(async () => {
    if (!selectedDevice) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(
        `${baseUrl}/v1/devices/${encodeURIComponent(selectedDevice)}/quarantine`,
        { method: "POST", headers: authHeaders() },
      );
      if (!res.ok) throw new Error(`quarantine ${res.status}`);
      await onRefresh();
      await inspectDevice(selectedDevice);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl, inspectDevice, onRefresh, selectedDevice, setBusy, setError]);

  const assignDevice = useCallback(async () => {
    if (!selectedDevice) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(
        `${baseUrl}/v1/devices/${encodeURIComponent(selectedDevice)}/assign`,
        {
          method: "POST",
          headers: authHeaders(),
          body: JSON.stringify({ robot_id: robotId }),
        },
      );
      if (!res.ok) throw new Error(`assign ${res.status}`);
      await onRefresh();
      await inspectDevice(selectedDevice);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl, inspectDevice, onRefresh, robotId, selectedDevice, setBusy, setError]);

  const trustDevice = useCallback(async () => {
    if (!selectedDevice) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(
        `${baseUrl}/v1/devices/${encodeURIComponent(selectedDevice)}/trust`,
        { method: "POST", headers: authHeaders() },
      );
      if (!res.ok) throw new Error(`trust ${res.status}`);
      await onRefresh();
      await inspectDevice(selectedDevice);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl, inspectDevice, onRefresh, selectedDevice, setBusy, setError]);

  return {
    selectedDevice,
    deviceDetail,
    discoveryResult,
    provisionReport,
    inspectDevice,
    runDiscovery,
    provisionDevice,
    quarantineDevice,
    assignDevice,
    trustDevice,
  };
}
