import { TAB_DESCRIPTIONS, tabLabel } from "./controlCenterNavConfig";
import type { ControlCenterTab } from "./controlCenterRbac";
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
  const refreshCtx = useControlCenterTabRefresh();
  const panelHandler = refreshCtx?.handler ?? null;

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
        <button type="button" onClick={handleRefresh} disabled={busy}>
          {busy ? "Refreshing…" : "Refresh"}
        </button>
      </div>
    </header>
  );
}
