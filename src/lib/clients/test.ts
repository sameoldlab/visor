import { createTestClient, http } from "viem";
import { foundry } from "viem/chains";

const test_client = createTestClient({
  chain: foundry,
  mode: "anvil",
  pollingInterval: 4_000,
  transport: http()
});

export const isAutomining = await test_client.getAutomine();

// Delay on adding all of these as anvil should still work with cast.
// Though to avoid breaking GUI best to assume anything might be updated randomly.
