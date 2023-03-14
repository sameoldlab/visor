import { createPublicClient, formatEther, http } from "viem"
import { foundry } from "viem/chains"

export let INTERVAL = 4_000
export const client = createPublicClient({
  chain: foundry,
  transport: http(),
  pollingInterval: INTERVAL, //lower if `-b < 4
})
