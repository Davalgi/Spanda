/**
 * Global Control Center Demo mode — show simulated/catalog examples when live data is empty.
 * @module
 */

import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
  type ReactNode,
} from "react";

const STORAGE_KEY = "spanda.control_center.demo_mode.v1";

type DemoModeContextValue = {
  demoMode: boolean;
  setDemoMode: (enabled: boolean) => void;
  toggleDemoMode: () => void;
};

const DemoModeContext = createContext<DemoModeContextValue | null>(null);

function readStoredDemoMode(): boolean {
  // Default on so empty serve still has useful showcase data.
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw === null) return true;
    return raw === "1" || raw === "true";
  } catch {
    return true;
  }
}

function writeStoredDemoMode(enabled: boolean): void {
  try {
    localStorage.setItem(STORAGE_KEY, enabled ? "1" : "0");
  } catch {
    // Ignore quota / private-mode failures.
  }
}

type ProviderProps = {
  children: ReactNode;
};

export function ControlCenterDemoModeProvider({ children }: ProviderProps) {
  // Persist Demo mode so reload keeps the operator's preference.
  //
  // Parameters:
  // - `children` — Control Center shell subtree
  //
  // Returns:
  // Context provider element.
  //
  // Options:
  // None.
  //
  // Example:
  // <ControlCenterDemoModeProvider><App /></ControlCenterDemoModeProvider>

  const [demoMode, setDemoModeState] = useState(readStoredDemoMode);

  const setDemoMode = useCallback((enabled: boolean) => {
    setDemoModeState(enabled);
    writeStoredDemoMode(enabled);
  }, []);

  const toggleDemoMode = useCallback(() => {
    setDemoModeState((current) => {
      const next = !current;
      writeStoredDemoMode(next);
      return next;
    });
  }, []);

  const value = useMemo(
    () => ({ demoMode, setDemoMode, toggleDemoMode }),
    [demoMode, setDemoMode, toggleDemoMode],
  );

  return <DemoModeContext.Provider value={value}>{children}</DemoModeContext.Provider>;
}

export function useControlCenterDemoMode(): DemoModeContextValue {
  // Read Demo mode; fall back to defaults when used outside the provider.
  //
  // Parameters:
  // None.
  //
  // Returns:
  // `{ demoMode, setDemoMode, toggleDemoMode }`.
  //
  // Options:
  // None.
  //
  // Example:
  // const { demoMode } = useControlCenterDemoMode();

  const ctx = useContext(DemoModeContext);
  if (ctx) return ctx;
  return {
    demoMode: true,
    setDemoMode: () => undefined,
    toggleDemoMode: () => undefined,
  };
}
