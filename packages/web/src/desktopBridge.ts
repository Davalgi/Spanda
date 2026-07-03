/** Optional Tauri desktop shell bridge for Control Center. @module */

type TauriNotificationApi = {
  isPermissionGranted: () => Promise<boolean>;
  requestPermission: () => Promise<"granted" | "denied" | "default">;
  sendNotification: (options: { title: string; body: string }) => void;
};

type TauriWindow = Window & {
  __TAURI__?: {
    core?: {
      invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
    };
    notification?: TauriNotificationApi;
  };
};

export function isDesktopShell(): boolean {
  return typeof window !== "undefined" && Boolean((window as TauriWindow).__TAURI__);
}

export async function desktopInvoke<T>(
  cmd: string,
  args?: Record<string, unknown>,
): Promise<T | undefined> {
  const invoke = (window as TauriWindow).__TAURI__?.core?.invoke;
  if (!invoke) return undefined;
  try {
    return await invoke<T>(cmd, args ?? {});
  } catch {
    return undefined;
  }
}

export async function desktopNotify(title: string, body: string): Promise<void> {
  const notification = (window as TauriWindow).__TAURI__?.notification;
  if (!notification) {
    await desktopInvoke("desktop_notify", { title, body });
    return;
  }
  let granted = await notification.isPermissionGranted();
  if (!granted) {
    const permission = await notification.requestPermission();
    granted = permission === "granted";
  }
  if (granted) {
    notification.sendNotification({ title, body });
  }
}

export async function desktopUpdateTray(status: string, detail?: string): Promise<void> {
  await desktopInvoke("update_tray_status", { status, detail: detail ?? null });
}
