/**
 * Lean-core provider contracts mirrored for TypeScript tests and CLI fallback.
 * @module
 */

/** Stable identifier for a registered provider implementation. */
export type ProviderId = {
  package: string;
  name: string;
};

/** Where a module or feature belongs in the lean-core architecture. */
export enum ModuleOwnership {
  Core = "core",
  StandardLibrary = "stdlib",
  OfficialPackage = "official-package",
  ExperimentalPackage = "experimental-package",
  CompatibilityShim = "compatibility-shim",
  Deprecated = "deprecated",
}

export type ModuleClassification = {
  module: string;
  ownership: ModuleOwnership;
  targetPackage: string | null;
  notes: string;
};

/** Official first-party package names recognized by the lean-core model. */
export const OFFICIAL_PACKAGE_NAMES: readonly string[] = [
  "spanda-gps",
  "spanda-wifi",
  "spanda-ble",
  "spanda-cellular",
  "spanda-mqtt",
  "spanda-dds",
  "spanda-ros2",
  "spanda-slam",
  "spanda-nav",
  "spanda-opencv",
  "spanda-yolo",
  "spanda-moveit",
  "spanda-gazebo",
  "spanda-webots",
  "spanda-fleet",
  "spanda-ota",
  "spanda-maintenance",
  "spanda-ledger",
  "spanda-cloud",
  "spanda-openai",
] as const;

/** Static audit table aligned with Rust `providers/classification.rs`. */
export const MODULE_CLASSIFICATIONS: readonly ModuleClassification[] = [
  { module: "lexer", ownership: ModuleOwnership.Core, targetPackage: null, notes: "Compiler front-end" },
  { module: "parser", ownership: ModuleOwnership.Core, targetPackage: null, notes: "Compiler front-end" },
  { module: "type_system", ownership: ModuleOwnership.Core, targetPackage: null, notes: "Type checker" },
  { module: "safety", ownership: ModuleOwnership.Core, targetPackage: null, notes: "ActionProposal / SafeAction gate" },
  { module: "providers", ownership: ModuleOwnership.Core, targetPackage: null, notes: "Extension trait contracts" },
  {
    module: "connectivity_positioning",
    ownership: ModuleOwnership.CompatibilityShim,
    targetPackage: "spanda-gps / spanda-wifi / spanda-ble / spanda-cellular",
    notes: "Type names in core; drivers in packages",
  },
  {
    module: "transport_mqtt",
    ownership: ModuleOwnership.CompatibilityShim,
    targetPackage: "spanda-mqtt",
    notes: "Use spanda-mqtt package",
  },
  {
    module: "transport_rclrs",
    ownership: ModuleOwnership.CompatibilityShim,
    targetPackage: "spanda-ros2",
    notes: "Use spanda-ros2 package",
  },
  {
    module: "fleet_orchestrator",
    ownership: ModuleOwnership.CompatibilityShim,
    targetPackage: "spanda-fleet",
    notes: "Fleet orchestration CLI; logic moves to package",
  },
] as const;

/** In-memory registry of installed provider keys (TS fallback; Rust owns live backends). */
export class ProviderRegistry {
  private readonly transports = new Set<string>();
  private readonly capabilities = new Set<string>();

  grantCapability(cap: string): void {
    this.capabilities.add(cap);
  }

  hasCapability(cap: string): boolean {
    return this.capabilities.has(cap);
  }

  registerTransport(id: ProviderId): void {
    this.transports.add(`${id.package}::${id.name}`);
  }

  listTransports(): ProviderId[] {
    return [...this.transports].map((key) => {
      const [pkg, name] = key.split("::");
      return { package: pkg, name: name ?? "default" };
    });
  }
}

/** Register default compatibility-shim providers for known official packages. */
export function bootstrapDefaultProviders(): ProviderRegistry {
  const registry = new ProviderRegistry();
  registry.grantCapability("mqtt.publish");
  registry.grantCapability("comm.ros2.publish");
  registry.registerTransport({ package: "spanda-mqtt", name: "stub" });
  registry.registerTransport({ package: "spanda-ros2", name: "stub" });
  registry.registerTransport({ package: "spanda-dds", name: "stub" });
  return registry;
}
