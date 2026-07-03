/** Saved Control Center connection profiles (API host + tenant metadata). @module */

export type ControlCenterProfile = {
  id: string;
  label: string;
  apiBase: string;
  tenantId?: string;
};

const PROFILES_KEY = "spanda.control_center.profiles.v1";
const ACTIVE_PROFILE_KEY = "spanda.control_center.active_profile.v1";

function normalizeBase(apiBase: string): string {
  return apiBase.replace(/\/$/, "");
}

function profileLabel(apiBase: string): string {
  try {
    return new URL(apiBase).host;
  } catch {
    return apiBase;
  }
}

export function loadProfiles(): ControlCenterProfile[] {
  try {
    const raw = localStorage.getItem(PROFILES_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw) as ControlCenterProfile[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

export function saveProfiles(profiles: ControlCenterProfile[]): void {
  try {
    localStorage.setItem(PROFILES_KEY, JSON.stringify(profiles));
  } catch {
    /* private mode */
  }
}

export function getActiveProfileId(): string | null {
  try {
    return localStorage.getItem(ACTIVE_PROFILE_KEY);
  } catch {
    return null;
  }
}

export function setActiveProfileId(id: string): void {
  try {
    localStorage.setItem(ACTIVE_PROFILE_KEY, id);
  } catch {
    /* private mode */
  }
}

export function ensureProfile(apiBase: string, tenantId?: string): ControlCenterProfile[] {
  const base = normalizeBase(apiBase);
  const profiles = loadProfiles();
  const existing = profiles.find((profile) => normalizeBase(profile.apiBase) === base);
  if (existing) {
    if (tenantId && existing.tenantId !== tenantId) {
      existing.tenantId = tenantId;
      saveProfiles(profiles);
    }
    setActiveProfileId(existing.id);
    return profiles;
  }
  const created: ControlCenterProfile = {
    id: crypto.randomUUID(),
    label: profileLabel(base),
    apiBase: base,
    tenantId,
  };
  const next = [...profiles, created];
  saveProfiles(next);
  setActiveProfileId(created.id);
  return next;
}

export function upsertProfileTenant(profileId: string, tenantId: string): ControlCenterProfile[] {
  const profiles = loadProfiles().map((profile) =>
    profile.id === profileId ? { ...profile, tenantId } : profile,
  );
  saveProfiles(profiles);
  return profiles;
}

export function addProfile(apiBase: string, label?: string): ControlCenterProfile[] {
  const base = normalizeBase(apiBase);
  const profiles = loadProfiles();
  if (profiles.some((profile) => normalizeBase(profile.apiBase) === base)) {
    return profiles;
  }
  const created: ControlCenterProfile = {
    id: crypto.randomUUID(),
    label: label?.trim() || profileLabel(base),
    apiBase: base,
  };
  const next = [...profiles, created];
  saveProfiles(next);
  setActiveProfileId(created.id);
  return next;
}

export function removeProfile(profileId: string): ControlCenterProfile[] {
  const next = loadProfiles().filter((profile) => profile.id !== profileId);
  saveProfiles(next);
  if (getActiveProfileId() === profileId && next.length > 0) {
    setActiveProfileId(next[0].id);
  }
  return next;
}

export function resolveActiveApiBase(fallback: string): string {
  const profiles = loadProfiles();
  const activeId = getActiveProfileId();
  const active = profiles.find((profile) => profile.id === activeId);
  if (active) return normalizeBase(active.apiBase);
  if (profiles.length > 0) {
    setActiveProfileId(profiles[0].id);
    return normalizeBase(profiles[0].apiBase);
  }
  return normalizeBase(fallback);
}
