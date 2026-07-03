import { useCallback, useEffect, useMemo, useState } from "react";
import { ControlCenterAuthBanner } from "./ControlCenterAuthBanner";
import { ControlCenterNav } from "./ControlCenterNav";
import { ControlCenterPluginPanel } from "./ControlCenterPluginPanel";
import { ControlCenterTabContent } from "./ControlCenterTabContent";
import { TAB_DESCRIPTIONS, tabLabel } from "./controlCenterNavConfig";
import {
  addProfile,
  ensureProfile,
  getActiveProfileId,
  type ControlCenterProfile,
  setActiveProfileId,
  upsertProfileTenant,
} from "./controlCenterProfiles";
import { type ControlCenterTab } from "./controlCenterRbac";
import { useControlCenterAuth } from "./useControlCenterAuth";
import { useControlCenterCoreData } from "./useControlCenterCoreData";
import { useDesktopIntegration } from "./useDesktopIntegration";
import { useDeviceWorkflow } from "./useDeviceWorkflow";

type Props = {
  apiBase: string;
};

export function ControlCenterPanel({ apiBase }: Props) {
  const [profiles, setProfiles] = useState<ControlCenterProfile[]>(() => ensureProfile(apiBase));
  const [activeProfileId, setActiveProfileIdState] = useState(
    () => getActiveProfileId() ?? profiles[0]?.id ?? "",
  );

  const activeProfile = useMemo(
    () => profiles.find((profile) => profile.id === activeProfileId) ?? profiles[0],
    [activeProfileId, profiles],
  );
  const activeApiBase = activeProfile?.apiBase ?? apiBase;

  const auth = useControlCenterAuth({ apiBase: activeApiBase });
  const {
    base,
    apiHost,
    rbacCtx,
    authHeaders,
    can,
    isTabAllowed,
    hasToken,
    effectiveRole,
    roleMeta,
    verifyAndSetApiKey,
    forgetToken,
    showAuthSetup,
    setShowAuthSetup,
    authError,
    envKeyLocked,
  } = auth;

  const [tab, setTab] = useState<ControlCenterTab>("dashboard");
  const [pluginTab, setPluginTab] = useState<string | null>(null);

  const core = useControlCenterCoreData({ baseUrl: base, authHeaders });

  useEffect(() => {
    if (rbacCtx?.tenant_id && activeProfile?.id) {
      setProfiles(upsertProfileTenant(activeProfile.id, rbacCtx.tenant_id));
    }
  }, [activeProfile?.id, rbacCtx?.tenant_id]);

  useDesktopIntegration({ baseUrl: base });

  const switchProfile = useCallback((profileId: string) => {
    setActiveProfileId(profileId);
    setActiveProfileIdState(profileId);
  }, []);

  const handleAddConnection = useCallback((nextBase: string) => {
    setProfiles(addProfile(nextBase));
    const id = getActiveProfileId();
    if (id) setActiveProfileIdState(id);
  }, []);

  useEffect(() => {
    if (!isTabAllowed(tab)) {
      setTab("dashboard");
    }
  }, [tab, isTabAllowed, effectiveRole]);

  useEffect(() => {
    if (tab === "readiness" && !core.readiness) {
      void core.runReadiness();
    }
  }, [tab, core.readiness, core.runReadiness]);

  const openProvisioning = useCallback(() => {
    setPluginTab(null);
    setTab("provisioning");
  }, []);

  const deviceWorkflow = useDeviceWorkflow({
    baseUrl: base,
    authHeaders,
    can,
    robotId: core.robotId,
    onRefresh: core.load,
    onOpenProvisioning: openProvisioning,
    setError: core.setError,
    setBusy: core.setBusy,
  });

  const activePlugin = pluginTab
    ? core.pluginPanels.find((panel) => `${panel.plugin}:${panel.id}` === pluginTab)
    : null;
  const sectionTitle = activePlugin ? activePlugin.title : tabLabel(tab);
  const sectionHint = pluginTab ? null : TAB_DESCRIPTIONS[tab];

  const selectTab = (name: ControlCenterTab) => {
    setPluginTab(null);
    setTab(name);
  };

  return (
    <div className="cc-shell">
      <aside className="cc-sidebar">
        <div className="cc-sidebar-brand">
          <span className="cc-sidebar-title">Control Center</span>
          <span className="cc-sidebar-host" title={base}>
            {apiHost}
          </span>
        </div>
        <ControlCenterNav
          activeTab={pluginTab ? null : tab}
          pluginTab={pluginTab}
          pluginPanels={core.pluginPanels}
          isTabAllowed={isTabAllowed}
          onSelectTab={selectTab}
          onSelectPlugin={setPluginTab}
        />
        <div className="cc-sidebar-footer">
          <ControlCenterAuthBanner
            apiHost={apiHost}
            effectiveRole={effectiveRole}
            roleMeta={roleMeta}
            keyId={rbacCtx?.key_id}
            tenantId={rbacCtx?.tenant_id ?? activeProfile?.tenantId}
            permissions={rbacCtx?.permissions ?? []}
            hasToken={hasToken}
            showAuthSetup={showAuthSetup}
            envKeyLocked={envKeyLocked}
            authError={authError}
            profiles={profiles}
            activeProfileId={activeProfile?.id}
            onSwitchProfile={switchProfile}
            onAddConnection={handleAddConnection}
            onVerify={verifyAndSetApiKey}
            onForget={forgetToken}
            onOpenSetup={() => setShowAuthSetup(true)}
          />
        </div>
      </aside>

      <main className="cc-main">
        <header className="cc-main-header">
          <div>
            <h2 className="cc-section-title">{sectionTitle}</h2>
            {sectionHint && <p className="cc-section-hint">{sectionHint}</p>}
          </div>
          <div className="cc-main-actions">
            <button type="button" onClick={() => void core.load()} disabled={core.busy}>
              {core.busy ? "Refreshing…" : "Refresh"}
            </button>
          </div>
        </header>

        {core.error && <div className="error">{core.error}</div>}
        {core.usingCache && (
          <div className="banner cc-offline-banner">
            Showing cached dashboard from {core.cacheAge ?? "earlier"} — API unreachable.
          </div>
        )}

        <div className="cc-main-body">
          {pluginTab && activePlugin ? (
            <ControlCenterPluginPanel panel={activePlugin} baseUrl={base} />
          ) : (
            <ControlCenterTabContent
              tab={tab}
              baseUrl={base}
              authHeaders={authHeaders}
              can={can}
              hasToken={hasToken}
              busy={core.busy}
              pool={core.pool}
              dashboard={core.dashboard}
              readiness={core.readiness}
              devices={core.devices}
              robots={core.robots}
              fleets={core.fleets}
              agents={core.agents}
              selectedRobot={core.selectedRobot}
              onRobotChange={core.setSelectedRobot}
              robotId={core.robotId}
              onNavigate={selectTab}
              onRunReadiness={() => void core.runReadiness()}
              onRefresh={() => void core.load()}
              onSignIn={() => setShowAuthSetup(true)}
              deviceWorkflow={deviceWorkflow}
            />
          )}
        </div>
      </main>
    </div>
  );
}
