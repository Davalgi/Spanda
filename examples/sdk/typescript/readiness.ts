/**
 * Evaluate readiness via the Spanda TypeScript SDK.
 *
 *   npx tsx examples/sdk/typescript/readiness.ts
 */
import { SpandaClient } from "../../../sdk/typescript/src/index.js";

async function main(): Promise<void> {
  const client = SpandaClient.local();
  const report = await client.readiness("examples/robotics/rover.sd");
  console.log("Readiness score:", report.score ?? report.raw);
}

main().catch(console.error);
