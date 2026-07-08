import type { RbacAction } from "./controlCenterRbac";
import type { ControlCenterTab } from "./controlCenterRbac";
import type { DeviceEntry, PluginPanelEntry } from "./controlCenterTypes";
import { AdministrationPanel } from "./AdministrationPanel";
import { AdasPanel } from "./AdasPanel";
import { AlertsPanel } from "./AlertsPanel";
import { AnalyticsPanel } from "./AnalyticsPanel";
import { AssuranceDiagnosisPanel } from "./AssuranceDiagnosisPanel";
import { AuditPanel } from "./AuditPanel";
import { GovernancePanel } from "./GovernancePanel";
import { CompliancePanel } from "./CompliancePanel";
import { ConfigPanel } from "./ConfigPanel";
import { ControlCenterDashboard } from "./ControlCenterDashboard";
import { DifferentiationPanel } from "./DifferentiationPanel";
import { DecisionsPanel } from "./DecisionsPanel";
import { DevicesPanel } from "./DevicesPanel";
import { DigitalThreadPanel } from "./DigitalThreadPanel";
import { DiscoveryPanel } from "./DiscoveryPanel";
import { DriftPanel } from "./DriftPanel";
import { EntitiesPanel } from "./EntitiesPanel";
import { ExecutivePanel } from "./ExecutivePanel";
import { FleetPanel } from "./FleetPanel";
import { HealthPanel } from "./HealthPanel";
import { HumansPanel } from "./HumansPanel";
import { MappingPanel } from "./MappingPanel";
import { MissionViewPanel } from "./MissionViewPanel";
import { OperatorPanel } from "./OperatorPanel";
import { OtaPanel } from "./OtaPanel";
import { ProvisioningPanel } from "./ProvisioningPanel";
import { ReadinessPanel } from "./ReadinessPanel";
import { RecoveryPanel } from "./RecoveryPanel";
import { MeshPanel } from "./MeshPanel";
import { ReplayPanel } from "./ReplayPanel";
import { ResilientAutonomyPanel } from "./ResilientAutonomyPanel";
import { SecurityPanel } from "./SecurityPanel";
import { SimulationPanel } from "./SimulationPanel";
import { SmartSpacesPanel } from "./SmartSpacesPanel";
import { SrePanel } from "./SrePanel";
import { TraceabilityPanel } from "./TraceabilityPanel";
import { TwinsPanel } from "./TwinsPanel";
import { ControlCenterTelemetryPanel } from "./ControlCenterTelemetryPanel";
import { ReadinessTrendsPanel } from "./ReadinessTrendsPanel";
import { ContinuityPanel } from "./ContinuityPanel";
import { FleetMapPanel } from "./FleetMapPanel";
import { ReportsPanel } from "./ReportsPanel";
import { PlaygroundPanel } from "./PlaygroundPanel";
import { MarketplacePanel } from "./MarketplacePanel";
import { ChaosPanel } from "./ChaosPanel";
import type { FleetAgent, FleetEntry, ReadinessImpact, RobotEntry } from "./controlCenterTypes";
import type { ProvisionReport } from "./ProvisioningPanel";

type DevicePool = NonNullable<import("./controlCenterTypes").DashboardData["device_pool"]>;

type Props = {
  tab: ControlCenterTab;
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
  busy: boolean;
  pool: DevicePool | null;
  dashboard: {
    fleet_agent_count: number;
    alert_count: number;
  } | null;
  readiness: ReadinessImpact | null;
  devices: DeviceEntry[];
  robots: RobotEntry[];
  fleets: FleetEntry[];
  agents: FleetAgent[];
  selectedRobot: string;
  onRobotChange: (robotId: string) => void;
  robotId: string;
  onNavigate: (tab: ControlCenterTab) => void;
  onRunReadiness: () => void;
  onRefresh: () => void;
  onSignIn: () => void;
  deviceWorkflow: {
    selectedDevice: string | null;
    deviceDetail: Record<string, unknown> | null;
    discoveryResult: Record<string, unknown> | null;
    provisionReport: ProvisionReport | null;
    inspectDevice: (id: string) => void;
    runDiscovery: (transports: string[]) => void;
    provisionDevice: () => void;
    quarantineDevice: () => void;
    assignDevice: () => void;
    trustDevice: () => void;
  };
};

export function ControlCenterTabContent({
  tab,
  baseUrl,
  authHeaders,
  can,
  hasToken,
  busy,
  pool,
  dashboard,
  readiness,
  devices,
  robots,
  fleets,
  agents,
  selectedRobot,
  onRobotChange,
  robotId,
  onNavigate,
  onRunReadiness,
  onRefresh,
  onSignIn,
  deviceWorkflow,
}: Props) {
  const {
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
  } = deviceWorkflow;

  switch (tab) {
    case "dashboard":
      return pool ? (
        <ControlCenterDashboard
          pool={pool}
          fleetAgentCount={dashboard?.fleet_agent_count ?? 0}
          alertCount={dashboard?.alert_count ?? 0}
          missionReady={readiness?.mission_ready}
          onNavigate={onNavigate}
        />
      ) : null;

    case "entities":
      return (
        <EntitiesPanel
          baseUrl={baseUrl}
          authHeaders={authHeaders}
          can={can}
          hasToken={hasToken}
        />
      );

    case "devices":
      return (
        <DevicesPanel
          devices={devices}
          loading={busy}
          onInspect={(id) => inspectDevice(id)}
          onDiscover={() => onNavigate("discovery")}
          baseUrl={baseUrl}
          authHeaders={authHeaders}
          canBulk={can("Operate") && hasToken}
          onBulkComplete={onRefresh}
        />
      );

    case "fleet":
      return <FleetPanel fleets={fleets} robots={robots} agents={agents} loading={busy} />;

    case "discovery":
      return (
        <DiscoveryPanel
          busy={busy}
          hasToken={hasToken}
          devices={devices}
          discoveryResult={discoveryResult}
          onDiscover={(transports) => runDiscovery(transports)}
          onSelectDevice={(id) => inspectDevice(id)}
          onSignIn={onSignIn}
        />
      );

    case "provisioning":
      return (
        <ProvisioningPanel
          devices={devices}
          selectedDevice={selectedDevice}
          deviceDetail={deviceDetail}
          provisionReport={provisionReport}
          robots={robots}
          robotId={robotId}
          busy={busy}
          hasToken={hasToken}
          can={can}
          onSelectDevice={(id) => inspectDevice(id)}
          onRobotChange={onRobotChange}
          onTrust={() => trustDevice()}
          onProvision={() => provisionDevice()}
          onAssign={() => assignDevice()}
          onQuarantine={() => quarantineDevice()}
          onSignIn={onSignIn}
        />
      );

    case "mapping":
      return <MappingPanel baseUrl={baseUrl} />;

    case "health":
      return pool ? <HealthPanel pool={pool} /> : null;

    case "readiness":
      return (
        <ReadinessPanel
          readiness={readiness}
          devices={devices}
          busy={busy}
          onRunCheck={onRunReadiness}
          onInspectDevice={(id) => inspectDevice(id)}
        />
      );

    case "sre":
      return (
        <SrePanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "drift":
      return (
        <DriftPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "config":
      return (
        <ConfigPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "alerts":
      return <AlertsPanel baseUrl={baseUrl} />;

    case "security":
      return <SecurityPanel baseUrl={baseUrl} />;

    case "ota":
      return (
        <OtaPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "compliance":
      return (
        <CompliancePanel
          baseUrl={baseUrl}
          authHeaders={authHeaders}
          can={can}
          hasToken={hasToken}
        />
      );

    case "governance":
      return (
        <GovernancePanel
          baseUrl={baseUrl}
          authHeaders={authHeaders}
          can={can}
          hasToken={hasToken}
        />
      );

    case "audit":
      return (
        <AuditPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "decisions":
      return <DecisionsPanel baseUrl={baseUrl} authHeaders={authHeaders} />;

    case "differentiation":
      return <DifferentiationPanel baseUrl={baseUrl} />;

    case "recovery":
      return (
        <RecoveryPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "mesh":
      return (
        <MeshPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "resilient-autonomy":
      return <ResilientAutonomyPanel baseUrl={baseUrl} authHeaders={authHeaders} />;

    case "mission":
      return (
        <MissionViewPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "operator":
      return (
        <OperatorPanel
          baseUrl={baseUrl}
          authHeaders={authHeaders}
          can={can}
          hasToken={hasToken}
          selectedDeviceId={selectedDevice ?? undefined}
          onQuarantine={() => onRefresh()}
        />
      );

    case "assurance":
      return <AssuranceDiagnosisPanel baseUrl={baseUrl} focus="assurance" />;

    case "diagnosis":
      return <AssuranceDiagnosisPanel baseUrl={baseUrl} focus="diagnosis" />;

    case "administration":
      return (
        <AdministrationPanel
          baseUrl={baseUrl}
          authHeaders={authHeaders}
          can={can}
          hasToken={hasToken}
        />
      );

    case "simulation":
      return (
        <SimulationPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "replay":
      return (
        <ReplayPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "executive":
      return <ExecutivePanel baseUrl={baseUrl} />;

    case "analytics":
      return <AnalyticsPanel baseUrl={baseUrl} />;

    case "twins":
      return (
        <TwinsPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "adas":
      return (
        <AdasPanel
          baseUrl={baseUrl}
          devicePool={pool}
          alertCount={dashboard?.alert_count ?? 0}
        />
      );

    case "humans":
      return (
        <HumansPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "smart-spaces":
      return <SmartSpacesPanel baseUrl={baseUrl} />;

    case "digital-thread":
      return <DigitalThreadPanel baseUrl={baseUrl} />;

    case "traceability":
      return <TraceabilityPanel baseUrl={baseUrl} devices={devices} />;

    case "telemetry":
      return <ControlCenterTelemetryPanel baseUrl={baseUrl} />;

    case "trends":
      return <ReadinessTrendsPanel baseUrl={baseUrl} />;

    case "continuity":
      return (
        <ContinuityPanel
          baseUrl={baseUrl}
          authHeaders={authHeaders}
          can={can}
          hasToken={hasToken}
          agents={agents}
        />
      );

    case "fleet-map":
      return <FleetMapPanel baseUrl={baseUrl} />;

    case "reports":
      return (
        <ReportsPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    case "playground":
      return <PlaygroundPanel baseUrl={baseUrl} />;

    case "marketplace":
      return <MarketplacePanel baseUrl={baseUrl} />;

    case "chaos":
      return (
        <ChaosPanel baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} />
      );

    default:
      return null;
  }
}
