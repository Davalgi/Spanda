import { useEffect, useState } from "react";
import { CONTROL_CENTER_VERSION } from "./controlCenterVersion";
import { desktopInvoke, isDesktopShell } from "./desktopBridge";

type DesktopFeatures = {
  app_version?: string;
};

/** Resolve the Control Center version for the current shell (web or Tauri desktop). */
export function useControlCenterVersion(): string {
  const [version, setVersion] = useState(CONTROL_CENTER_VERSION);

  useEffect(() => {
    if (!isDesktopShell()) return;
    let cancelled = false;

    void desktopInvoke<DesktopFeatures>("desktop_features").then((features) => {
      if (!cancelled && features?.app_version) {
        setVersion(features.app_version);
      }
    });

    return () => {
      cancelled = true;
    };
  }, []);

  return version;
}
