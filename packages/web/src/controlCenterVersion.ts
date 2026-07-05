/** Control Center UI semver injected at build time. @module */

declare const __CONTROL_CENTER_VERSION__: string | undefined;

/** Semver for the Control Center UI bundle (packages/web). */
export const CONTROL_CENTER_VERSION =
  typeof __CONTROL_CENTER_VERSION__ === "string" && __CONTROL_CENTER_VERSION__.length > 0
    ? __CONTROL_CENTER_VERSION__
    : "dev";
