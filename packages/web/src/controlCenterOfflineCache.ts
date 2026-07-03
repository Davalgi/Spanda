/** Offline cache for Control Center dashboard snapshots. @module */

const CACHE_PREFIX = "spanda.control_center.cache.v1:";

export type CachedDashboard = {
  saved_at: number;
  dashboard: unknown;
  pool: unknown;
};

export function cacheKey(apiHost: string): string {
  return `${CACHE_PREFIX}${apiHost}`;
}

export function saveDashboardCache(apiHost: string, dashboard: unknown, pool: unknown): void {
  try {
    const payload: CachedDashboard = {
      saved_at: Date.now(),
      dashboard,
      pool,
    };
    localStorage.setItem(cacheKey(apiHost), JSON.stringify(payload));
  } catch {
    // Ignore quota or private-mode errors.
  }
}

export function loadDashboardCache(apiHost: string): CachedDashboard | null {
  try {
    const raw = localStorage.getItem(cacheKey(apiHost));
    if (!raw) return null;
    return JSON.parse(raw) as CachedDashboard;
  } catch {
    return null;
  }
}

export function formatCacheAge(savedAt: number): string {
  const delta = Date.now() - savedAt;
  const minutes = Math.round(delta / 60000);
  if (minutes < 1) return "just now";
  if (minutes < 60) return `${minutes}m ago`;
  return `${Math.round(minutes / 60)}h ago`;
}
