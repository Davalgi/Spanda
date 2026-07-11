import { useCallback, useEffect, useMemo, useState } from "react";
import { ControlCenterAuthBanner } from "./ControlCenterAuthBanner";
import { ControlCenterNav } from "./ControlCenterNav";
import { ControlCenterPluginPanel } from "./ControlCenterPluginPanel";
import { ControlCenterTabContent } from "./ControlCenterTabContent";
import { ControlCenterMainHeader } from "./ControlCenterMainHeader";
import { ControlCenterTabRefreshProvider } from "./useControlCenterTabRefresh";
import {
  addProfile,
  ensureProfile,
  getActiveProfileId,
  type ControlCenterProfile,
  setActiveProfileId,
  upsertProfileCredentials,
} from "./controlCenterProfiles";
import { type ControlCenterTab } from "./controlCenterRbac";
import { useControlCenterAuth } from "./useControlCenterAuth";
import { useControlCenterCoreData } from "./useControlCenterCoreData";
import { useControlCenterVersion } from "./useControlCenterVersion";
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
    oidcLoginEnabled,
    signInWithOidc,
  } = auth;

  const [tab, setTab] = useState<ControlCenterTab>("dashboard");
  const [pluginTab, setPluginTab] = useState<string | null>(null);

  const core = useControlCenterCoreData({ baseUrl: base, authHeaders });

  useEffect(() => {
    if (!activeProfile?.id) return;
    const next: { tenantId?: string; apiKey?: string } = {};
    if (rbacCtx?.tenant_id) next.tenantId = rbacCtx.tenant_id;
    if (auth.apiKey) next.apiKey = auth.apiKey;
    if (next.tenantId || next.apiKey) {
      setProfiles(upsertProfileCredentials(activeProfile.id, next));
    }
  }, [activeProfile?.id, auth.apiKey, rbacCtx?.tenant_id]);

  useDesktopIntegration({ baseUrl: base });
  const controlCenterVersion = useControlCenterVersion();

  const switchProfile = useCallback(
    (profileId: string) => {
      setActiveProfileId(profileId);
      setActiveProfileIdState(profileId);
      const profile = profiles.find((entry) => entry.id === profileId);
      if (profile?.apiKey) {
        void verifyAndSetApiKey(profile.apiKey, true);
      }
    },
    [profiles, verifyAndSetApiKey],
  );

  const forgetTokenAndProfile = useCallback(() => {
    if (activeProfile?.id) {
      setProfiles(upsertProfileCredentials(activeProfile.id, { apiKey: null }));
    }
    forgetToken();
  }, [activeProfile?.id, forgetToken]);

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

  const selectTab = (name: ControlCenterTab) => {
    setPluginTab(null);
    setTab(name);
  };

  return (
    <div className="cc-shell">
      <aside className="cc-sidebar">
        <div className="cc-sidebar-brand">
          <div className="cc-sidebar-title-row">
            <span className="cc-sidebar-title">Control Center</span>
            <span className="cc-sidebar-version">v{controlCenterVersion}</span>
          </div>
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
            compact
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
            onSignInWithOidc={signInWithOidc}
            oidcLoginEnabled={oidcLoginEnabled}
            onForget={forgetTokenAndProfile}
            onOpenSetup={() => setShowAuthSetup(true)}
          />
        </div>
      </aside>

      <main className="cc-main">
        <ControlCenterTabRefreshProvider tab={tab}>
          <ControlCenterMainHeader
            tab={tab}
            pluginTitle={activePlugin?.title ?? null}
            coreBusy={core.busy}
            onCoreRefresh={() => void core.load()}
            onReadinessRefresh={() => void core.runReadiness()}
          />

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
        </ControlCenterTabRefreshProvider>
      </main>
    </div>
  );
}
