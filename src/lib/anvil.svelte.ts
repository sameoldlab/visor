import type { Block } from "viem"
// import { client } from "./clients/public"

let _testnet_log = $state("")
let _live = $state(false)
let _block_number = $state("0")
const _blocks: Block[] = $state([])

export const testnet_log = () => _testnet_log
export const live = () => _live
export const block_number = () => _block_number
export const blocks = () => _blocks

class Anvil {
  emitOnBegin = true
  emitMissed = true
  live = $state(false)

  onBlock(block: Block) {
    // block_number.update((current) => 
    let current = block_number()
    // Check if current block has changed then update block_number and blocks.
    // Avoids double check and unnecessary updates on block store
    if (block.number === null) return current
    if (current === block.number.toString()) return current

    _blocks.push(block)
    _block_number = block.number.toString()

  }
}
export const anvil = new Anvil()

// export async function startTestnet(args: string[] = []) {
//   // args = ["--config-out", `${appLocalDataDirPath}config.json`, ...args]
//   // const cmd = Command.sidecar("../public/bin/anvil", args)

//   const unwatch = client.watchBlocks({


//     cmd.stdout.on("data", (line: string) => {
//       console.info("stdout: ", line)
//       testnet_log.update((state) => {
//         if (state.includes("command finished with code")) {
//           blocks.set([]) // save output until next run
//           return `${line}<br/>`
//         } else {
//           return state + `${line}<br/>`
//         }
//       })
//       live.set(true)
//     })

//   cmd.stderr.on("data", (line) => {
//       console.debug("stderr: ", line)
//       testnet_log.update((state) => state + `${line}<br/>`)
//     })
//   }
