import {
  createPublicClient,
  formatEther,
  http,
  type Address,
  type Block,
} from "viem"
import { foundry } from "viem/chains"

export const client = createPublicClient({
  chain: foundry,
  transport: http(),
  pollingInterval: 4_000, //lower if `-b < 4
})

let blocks
export const getBlocks = async () => {
  const latest: Block = await client.getBlock()
  let m = latest
  blocks.push(latest)
  return blocks
}

export const block_number = await client.getBlockNumber()

export const watchBlock = client.watchBlocks({
  onBlock: (block) => blocks.push(block),
})
