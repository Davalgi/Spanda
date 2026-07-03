/** Operational governance SDK clients. */

import type { JsonValue, SpandaClient } from "./client";

export class GovernanceClient {
  constructor(private readonly client: SpandaClient) {}

  async summary(): Promise<JsonValue> {
    return this.client.request("GET", "/v1/governance");
  }

  async validate(body: JsonValue = {}): Promise<JsonValue> {
    return this.client.request("POST", "/v1/governance/validate", body, true);
  }
}

export class ComplianceClient {
  constructor(private readonly client: SpandaClient) {}

  async summary(): Promise<JsonValue> {
    return this.client.request("GET", "/v1/compliance", undefined, true);
  }

  async check(body: JsonValue = {}): Promise<JsonValue> {
    return this.client.request("POST", "/v1/compliance/check", body, true);
  }
}

export class CertificationClient {
  constructor(private readonly client: SpandaClient) {}

  async list(): Promise<JsonValue> {
    return this.client.request("GET", "/v1/certifications", undefined, true);
  }
}

export class DeploymentProfileClient {
  constructor(private readonly client: SpandaClient) {}

  async list(): Promise<JsonValue> {
    return this.client.request("GET", "/v1/deployment-profiles");
  }

  async get(name: string): Promise<JsonValue> {
    return this.client.request(
      "GET",
      `/v1/deployment-profiles?name=${encodeURIComponent(name)}`,
    );
  }
}

export class RiskClient {
  constructor(private readonly client: SpandaClient) {}

  async summary(): Promise<JsonValue> {
    return this.client.request("GET", "/v1/risk", undefined, true);
  }
}
