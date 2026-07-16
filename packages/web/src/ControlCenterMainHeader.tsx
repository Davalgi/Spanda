import { TAB_DESCRIPTIONS, tabLabel } from "./controlCenterNavConfig";
import type { ControlCenterTab } from "./controlCenterRbac";
import { CcBadge } from "./controlCenterUi";
import { useControlCenterDemoMode } from "./useControlCenterDemoMode";
import { useControlCenterTabRefresh } from "./useControlCenterTabRefresh";

type Props = {
  tab: ControlCenterTab;
  pluginTitle: string | null;
  coreBusy: boolean;
  onCoreRefresh: () => void;
  onReadinessRefresh: () => void;
};

export function ControlCenterMainHeader({
  tab,
  pluginTitle,
  coreBusy,
  onCoreRefresh,
  onReadinessRefresh,
}: Props) {
  // Render the active-tab title, Demo mode toggle, and refresh action.
  //
  // Parameters:
  // - `tab` — active Control Center tab id
  // - `pluginTitle` — optional plugin panel title overriding the tab label
  // - `coreBusy` — dashboard core refresh busy flag
  // - `onCoreRefresh` / `onReadinessRefresh` — shell-level refresh handlers
  //
  // Returns:
  // Main header element.
  //
  // Options:
  // None.
  //
  // Example:
  // <ControlCenterMainHeader tab="dashboard" pluginTitle={null} … />

  const refreshCtx = useControlCenterTabRefresh();
  const panelHandler = refreshCtx?.handler ?? null;
  const { demoMode, toggleDemoMode } = useControlCenterDemoMode();

  const sectionTitle = pluginTitle ?? tabLabel(tab);
  const sectionHint = pluginTitle ? null : TAB_DESCRIPTIONS[tab];

  const busy = panelHandler?.busy ?? coreBusy;

  const handleRefresh = () => {
    if (panelHandler) {
      void panelHandler.refresh();
      return;
    }
    if (tab === "readiness") {
      onReadinessRefresh();
      return;
    }
    onCoreRefresh();
  };

  return (
    <header className="cc-main-header">
      <div>
        <h2 className="cc-section-title">{sectionTitle}</h2>
        {sectionHint && <p className="cc-section-hint">{sectionHint}</p>}
      </div>
      <div className="cc-main-actions">
        <label className={`cc-demo-toggle${demoMode ? " is-on" : ""}`} title="Show simulated and catalog examples when live data is empty">
          <input
            type="checkbox"
            checked={demoMode}
            onChange={() => toggleDemoMode()}
          />
          <span>Demo mode</span>
          {demoMode ? <CcBadge tone="info">on</CcBadge> : <CcBadge tone="neutral">off</CcBadge>}
        </label>
        <button type="button" onClick={handleRefresh} disabled={busy}>
          {busy ? "Refreshing…" : "Refresh"}
        </button>
      </div>
    </header>
  );
}
