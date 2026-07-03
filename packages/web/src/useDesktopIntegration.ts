import { useEffect, useRef } from "react";
import { desktopNotify, desktopUpdateTray, isDesktopShell } from "./desktopBridge";

type Options = {
  baseUrl: string;
  enabled?: boolean;
};

export function useDesktopIntegration({ baseUrl, enabled = true }: Options) {
  const fastBurnNotified = useRef(false);

  useEffect(() => {
    if (!enabled || !isDesktopShell()) return;
    let cancelled = false;

    const poll = async () => {
      try {
        const instanceRes = await fetch(`${baseUrl}/v1/instance`);
        if (instanceRes.ok && !cancelled) {
          const body = await instanceRes.json();
          const status = String(body.overall_status ?? body.status ?? "unknown");
          const robots = body.robot_count ?? body.robots ?? "—";
          await desktopUpdateTray(status, `robots: ${robots}`);
        }

        const sreRes = await fetch(`${baseUrl}/v1/sre/summary`);
        if (sreRes.ok && !cancelled) {
          const summary = await sreRes.json();
          const fastBurn = summary?.burn_rate?.fast_burn === true;
          if (fastBurn && !fastBurnNotified.current) {
            fastBurnNotified.current = true;
            await desktopNotify(
              "Spanda SLO fast-burn",
              "SRE burn-rate alert — open Control Center SRE tab to investigate.",
            );
          }
          if (!fastBurn) {
            fastBurnNotified.current = false;
          }
        }
      } catch {
        if (!cancelled) {
          await desktopUpdateTray("offline", "API unreachable");
        }
      }
    };

    void poll();
    const timer = window.setInterval(() => void poll(), 60_000);
    return () => {
      cancelled = true;
      window.clearInterval(timer);
    };
  }, [baseUrl, enabled]);
}
