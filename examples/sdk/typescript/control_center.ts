/**
 * Control Center dashboard + entity inventory via TypeScript SDK.
 *
 *   npx tsx examples/sdk/typescript/control_center.ts
 */
import { SpandaClient } from "../../../sdk/typescript/src/index.js";

async function main(): Promise<void> {
  const client = SpandaClient.local();
  const health = await client.healthCheck();
  console.log("Service health:", health);
  const entities = await client.listEntities();
  console.log(`Entities: ${entities.length}`);
}

main().catch(console.error);
