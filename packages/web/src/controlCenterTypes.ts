/** Shared Control Center data shapes. @module */

export type DashboardData = {
  device_pool: {
    total: number;
    healthy: number;
    active?: number;
    degraded: number;
    discovered: number;
    quarantined?: number;
    failed: number;
  };
  fleet_agent_count: number;
  alert_count: number;
};

export type FleetAgent = {
  robot_name: string;
  url: string;
  token?: string;
};

export type DeviceEntry = {
  id: string;
  device_type: string;
  lifecycle_state: string;
  assigned_robot?: string;
  logical_name?: string;
  trust_level?: string;
};

export type RobotEntry = {
  id: string;
  model?: string;
  hardware_profile?: string;
};

export type FleetEntry = {
  id: string;
  robot_count: number;
};

export type ReadinessImpact = {
  mission_ready: boolean;
  impact: {
    blocked_count: number;
    total_devices: number;
  };
};

export type PluginPanelEntry = {
  plugin: string;
  id: string;
  title: string;
  component: string;
};
