import { useCallback, useEffect, useMemo, useState } from "react";
import {
  authStorageKey,
  canAction,
  type ControlCenterTab,
  type RbacAction,
  type RbacContext,
  roleKey,
  ROLE_META,
  tabAllowed,
} from "./controlCenterRbac";

type Options = {
  apiBase: string;
};

export function useControlCenterAuth({ apiBase }: Options) {
  const base = apiBase.replace(/\/$/, "");
  const apiHost = useMemo(() => {
    try {
      return new URL(base).host;
    } catch {
      return base;
    }
  }, [base]);

  const envKey =
    (import.meta as { env?: { VITE_SPANDA_API_KEY?: string } }).env?.VITE_SPANDA_API_KEY ?? "";

  const [apiKey, setApiKeyState] = useState(() => {
    if (envKey) return envKey;
    try {
      const stored = localStorage.getItem(authStorageKey(apiHost));
      return stored?.trim() ?? "";
    } catch {
      return "";
    }
  });
  const [rbacCtx, setRbacCtx] = useState<RbacContext | null>(null);
  const [authError, setAuthError] = useState<string | null>(null);
  const [showAuthSetup, setShowAuthSetup] = useState(false);

  const authHeaders = useCallback((): HeadersInit => {
    const headers: Record<string, string> = { "Content-Type": "application/json" };
    if (apiKey) headers.Authorization = `Bearer ${apiKey}`;
    return headers;
  }, [apiKey]);

  const refreshRbac = useCallback(async () => {
    if (!apiKey) {
      setRbacCtx(null);
      return;
    }
    try {
      const res = await fetch(`${base}/v1/rbac/me`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`rbac/me ${res.status}`);
      setRbacCtx((await res.json()) as RbacContext);
      setAuthError(null);
    } catch {
      setRbacCtx({ role: "guest", permissions: [] });
    }
  }, [apiKey, authHeaders, base]);

  useEffect(() => {
    void refreshRbac();
  }, [refreshRbac]);

  const verifyAndSetApiKey = useCallback(
    async (value: string, persist: boolean) => {
      const trimmed = value.trim();
      if (!trimmed) return;
      const res = await fetch(`${base}/v1/alerts/test`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${trimmed}`,
        },
      });
      if (!res.ok) {
        throw new Error(`token rejected (${res.status})`);
      }
      setApiKeyState(trimmed);
      if (persist) {
        try {
          localStorage.setItem(authStorageKey(apiHost), trimmed);
        } catch {
          /* private mode */
        }
      }
      setShowAuthSetup(false);
      setAuthError(null);
    },
    [apiHost, base],
  );

  const forgetToken = useCallback(() => {
    setApiKeyState("");
    setRbacCtx(null);
    try {
      localStorage.removeItem(authStorageKey(apiHost));
    } catch {
      /* ignore */
    }
    setShowAuthSetup(true);
  }, [apiHost]);

  const effectiveRole = apiKey ? roleKey(rbacCtx?.role) : "guest";
  const roleMeta = ROLE_META[effectiveRole] ?? ROLE_META.guest;

  const can = useCallback(
    (action: RbacAction) => canAction(rbacCtx, action),
    [rbacCtx],
  );

  const isTabAllowed = useCallback(
    (tab: ControlCenterTab) => tabAllowed(tab, effectiveRole),
    [effectiveRole],
  );

  const hasToken = Boolean(apiKey);

  return {
    base,
    apiKey,
    apiHost,
    rbacCtx,
    authError,
    setAuthError,
    showAuthSetup,
    setShowAuthSetup,
    authHeaders,
    refreshRbac,
    verifyAndSetApiKey,
    forgetToken,
    effectiveRole,
    roleMeta,
    can,
    isTabAllowed,
    hasToken,
    envKeyLocked: Boolean(envKey),
  };
}
