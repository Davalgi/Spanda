import { useCallback, useEffect, useState } from "react";
import type {
  DashboardData,
  DeviceEntry,
  FleetAgent,
  FleetEntry,
  PluginPanelEntry,
  ReadinessImpact,
  RobotEntry,
} from "./controlCenterTypes";

type Options = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
};

export function useControlCenterCoreData({ baseUrl, authHeaders }: Options) {
  const [dashboard, setDashboard] = useState<DashboardData | null>(null);
  const [agents, setAgents] = useState<FleetAgent[]>([]);
  const [devices, setDevices] = useState<DeviceEntry[]>([]);
  const [robots, setRobots] = useState<RobotEntry[]>([]);
  const [fleets, setFleets] = useState<FleetEntry[]>([]);
  const [readiness, setReadiness] = useState<ReadinessImpact | null>(null);
  const [selectedRobot, setSelectedRobot] = useState("");
  const [pluginPanels, setPluginPanels] = useState<PluginPanelEntry[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [dashRes, fleetRes, devRes, robotRes, fleetListRes] = await Promise.all([
        fetch(`${baseUrl}/v1/dashboard`),
        fetch(`${baseUrl}/v1/fleet/agents`),
        fetch(`${baseUrl}/v1/devices`),
        fetch(`${baseUrl}/v1/robots`),
        fetch(`${baseUrl}/v1/fleets`),
      ]);
      if (!dashRes.ok) throw new Error(`dashboard ${dashRes.status}`);
      setDashboard(await dashRes.json());
      if (fleetRes.ok) {
        const fleetBody = await fleetRes.json();
        setAgents(fleetBody.agents ?? []);
      }
      if (devRes.ok) {
        const devBody = await devRes.json();
        setDevices(devBody.devices ?? []);
      }
      if (robotRes.ok) {
        const robotBody = await robotRes.json();
        const nextRobots = robotBody.robots ?? [];
        setRobots(nextRobots);
        setSelectedRobot((current) => current || nextRobots[0]?.id || "");
      }
      if (fleetListRes.ok) {
        const fleetBody = await fleetListRes.json();
        setFleets(fleetBody.fleets ?? []);
      }
      const pluginsRes = await fetch(`${baseUrl}/v1/plugins/control-center`, {
        headers: authHeaders(),
      });
      if (pluginsRes.ok) {
        const pluginsBody = await pluginsRes.json();
        const panels: PluginPanelEntry[] = [];
        for (const entry of pluginsBody.plugins ?? []) {
          const pluginName =
            entry.installed?.name ?? entry.manifest?.plugin?.name ?? "plugin";
          for (const panel of entry.manifest?.control_center?.panels ?? []) {
            panels.push({
              plugin: pluginName,
              id: panel.id,
              title: panel.title,
              component: panel.component,
            });
          }
        }
        setPluginPanels(panels);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  const runReadiness = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/readiness/run`, { method: "POST" });
      if (!res.ok) throw new Error(`readiness ${res.status}`);
      setReadiness(await res.json());
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  const robotId = selectedRobot || robots[0]?.id || "rover-001";
  const pool = dashboard?.device_pool ?? null;

  return {
    dashboard,
    pool,
    agents,
    devices,
    robots,
    fleets,
    readiness,
    selectedRobot,
    setSelectedRobot,
    robotId,
    pluginPanels,
    busy,
    setBusy,
    error,
    setError,
    load,
    runReadiness,
  };
}
