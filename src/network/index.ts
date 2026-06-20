/** std.network — communication and transport domain types for Spanda programs. */

export const STD_NETWORK_TYPES = [
  "Transport",
  "QosProfile",
  "QoS",
  "Bandwidth",
  "Latency",
  "TopicPath",
  "ServiceEndpoint",
  "MessageEnvelope",
  "DiscoveryFilter",
  "NetworkRequirements",
  "Reliability",
  "HistoryPolicy",
  "CommBus",
  "Endpoint",
  "Topic",
  "Message",
  "Service",
  "Action",
] as const;

export type StdNetworkType = (typeof STD_NETWORK_TYPES)[number];

export function isStdNetworkType(name: string): name is StdNetworkType {
  return (STD_NETWORK_TYPES as readonly string[]).includes(name);
}

export function resolveStdNetworkImport(path: string): boolean {
  return path === "std.network";
}
