/** Operational governance SDK clients. */

import type { SpandaClient } from "./client.js";
import type { JsonValue } from "./types.js";

export class GovernanceClient {
  constructor(private readonly client: SpandaClient) {}

  async summary(): Promise<JsonValue> {
    return this.client.getGovernance();
  }

  async validate(body: JsonValue = {}): Promise<JsonValue> {
    return this.client.validateGovernance(body);
  }
}

export class ComplianceClient {
  constructor(private readonly client: SpandaClient) {}

  async summary(): Promise<JsonValue> {
    return this.client.getComplianceSummary();
  }

  async check(body: JsonValue = {}): Promise<JsonValue> {
    return this.client.checkCompliance(body);
  }
}

export class CertificationClient {
  constructor(private readonly client: SpandaClient) {}

  async list(): Promise<JsonValue> {
    return this.client.listCertifications();
  }
}

export class DeploymentProfileClient {
  constructor(private readonly client: SpandaClient) {}

  async list(): Promise<JsonValue> {
    return this.client.listDeploymentProfiles();
  }

  async get(name: string): Promise<JsonValue> {
    return this.client.getDeploymentProfile(name);
  }
}

export class RiskClient {
  constructor(private readonly client: SpandaClient) {}

  async summary(): Promise<JsonValue> {
    return this.client.getRiskSummary();
  }
}
