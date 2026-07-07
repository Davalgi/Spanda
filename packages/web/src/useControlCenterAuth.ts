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
  const [oidcLoginEnabled, setOidcLoginEnabled] = useState(false);

  useEffect(() => {
    void (async () => {
      try {
        const res = await fetch(`${base}/v1/auth/config`);
        if (!res.ok) return;
        const body = (await res.json()) as { oidc_login_enabled?: boolean };
        setOidcLoginEnabled(Boolean(body.oidc_login_enabled));
      } catch {
        setOidcLoginEnabled(false);
      }
    })();
  }, [base]);

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
      const sessionRes = await fetch(`${base}/v1/auth/session`, {
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${trimmed}`,
        },
      });
      if (!sessionRes.ok) {
        const testRes = await fetch(`${base}/v1/alerts/test`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${trimmed}`,
          },
        });
        if (!testRes.ok) {
          throw new Error(`token rejected (${testRes.status})`);
        }
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

  const signInWithOidc = useCallback(async () => {
    const redirectUri = `${window.location.origin}/admin/oauth/oidc/callback`;
    const res = await fetch(`${base}/v1/auth/oidc/authorize-url`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ redirect_uri: redirectUri }),
    });
    if (!res.ok) {
      throw new Error(`OIDC authorize failed (${res.status})`);
    }
    const body = (await res.json()) as { authorize_url?: string };
    if (!body.authorize_url) {
      throw new Error("authorize_url missing");
    }
    const popup = window.open(body.authorize_url, "spanda-oidc", "width=520,height=720");
    if (!popup) {
      throw new Error("popup blocked — allow popups for this site");
    }
    await new Promise<void>((resolve, reject) => {
      const onMessage = (event: MessageEvent) => {
        if (event.origin !== window.location.origin) return;
        if (event.data?.type !== "spanda-oidc-oauth" || !event.data.code) return;
        window.removeEventListener("message", onMessage);
        void (async () => {
          try {
            const callbackRes = await fetch(`${base}/v1/auth/oidc/callback`, {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify({
                code: String(event.data.code),
                state: event.data.state ? String(event.data.state) : undefined,
              }),
            });
            const callbackBody = (await callbackRes.json()) as {
              session_token?: string;
            };
            if (!callbackRes.ok || !callbackBody.session_token) {
              reject(new Error(`OIDC callback failed (${callbackRes.status})`));
              return;
            }
            await verifyAndSetApiKey(callbackBody.session_token, true);
            resolve();
          } catch (error) {
            reject(error);
          }
        })();
      };
      window.addEventListener("message", onMessage);
    });
  }, [base, verifyAndSetApiKey]);

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
    oidcLoginEnabled,
    signInWithOidc,
  };
}
