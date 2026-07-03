/**
 * Per-tab refresh registration for the Control Center shell header.
 * @module
 */

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from "react";
import type { ControlCenterTab } from "./controlCenterRbac";

export type TabRefreshHandler = {
  refresh: () => void | Promise<void>;
  busy?: boolean;
};

type TabRefreshContextValue = {
  handler: TabRefreshHandler | null;
  register: (handler: TabRefreshHandler | null) => void;
};

const TabRefreshContext = createContext<TabRefreshContextValue | null>(null);

type ProviderProps = {
  tab: ControlCenterTab;
  children: ReactNode;
};

export function ControlCenterTabRefreshProvider({ tab, children }: ProviderProps) {
  const handlerRef = useRef<TabRefreshHandler | null>(null);
  const tabRef = useRef(tab);
  const [revision, setRevision] = useState(0);

  // Clear stale handlers synchronously when the active tab changes.
  if (tabRef.current !== tab) {
    tabRef.current = tab;
    handlerRef.current = null;
  }

  const register = useCallback((next: TabRefreshHandler | null) => {
    handlerRef.current = next;
    setRevision((current) => current + 1);
  }, []);

  const value = useMemo(
    () => ({
      handler: handlerRef.current,
      register,
    }),
    [register, revision],
  );

  return <TabRefreshContext.Provider value={value}>{children}</TabRefreshContext.Provider>;
}

export function useControlCenterTabRefresh(): TabRefreshContextValue | null {
  return useContext(TabRefreshContext);
}

export function useRegisterTabRefresh(
  refresh: () => void | Promise<void>,
  options?: { busy?: boolean },
) {
  const register = useContext(TabRefreshContext)?.register;
  const busy = options?.busy ?? false;

  useEffect(() => {
    if (!register) return undefined;
    register({ refresh, busy });
    return () => register(null);
  }, [register, refresh, busy]);
}
