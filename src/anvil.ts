import { Child, Command } from "@tauri-apps/api/shell"
import { writable } from "svelte/store"
import type { Block } from "viem"
import { client, INTERVAL } from "./lib/clients/public"

export let testnet_log = writable<string>("")
export let live = writable<boolean>(false)
export let block_number = writable<string>("0")
export const blocks = writable<Block[]>([])

/**
 * Handle to command child process
 */
let _child: Child
export const killTestnet = () => {
  return _child.kill()
}
export async function startTestnet(args: string[] = []) {
  args = ["--config-out", "../src/lib/anvil.json", ...args]
  // process = await invoke("start_testnet", { args: ["-b", "3"] })
  const cmd: Command = new Command("anvil-cli", args)
  let i = 0

  const unwatch = client.watchBlocks({
    onBlock: (block) => {
        block_number.set(block.number)
      blocks.update((state) => {
        if (block && block?.number !== state[state.length - 1]?.number) {
          state = [...state, block] as Block[]
        }
        return state
      })
      console.log("new block", block)
    },
    emitOnBegin: true,
    emitMissed: true,
  })

  cmd.on("close", (data) => {
    console.debug(
      `command finished with code ${data.code} and signal ${data.signal}`
    )
    /* 
      data.signal 6 = error/failed to start
      data.signal 15 = killed a running program
      */
    if (data.signal == 6) {
      testnet_log.update(
        () => `anvil is already running. 
             <br/>You can kill it from your terminal with <code>pkill '^anvil$'</code>`
      )
    }
    testnet_log.update(
      (state) =>
        `${state} 
        <br/>command finished with code ${data.code} and signal ${data.signal}`
    )
    live.set(false)
    if (_child == null) return
    console.log("Clearing state")
    unwatch()
    block_number.set("0")
    return
  })

  cmd.on("error", (line) => {
    console.debug("on err: ", line)
    live.set(false)
    testnet_log.update((state) => state + `${line}\n`)
  })

  // Assigning after  don't accidentally get a running process when commnd fails
  _child = await cmd.spawn()

  cmd.stdout.on("data", (line) => {
    console.debug("stdout: ", line)
    live.set(true)
    testnet_log.update((state) => {
      if (state.includes("command finished with code")) {
        blocks.set([]) // save output until next run
        return `${line}<br/>`
      } else {
        return state + `${line}<br/>`
      }
    })
  })

  // Not needed. Errors already handled by listen events
  /*   cmd.stderr.on("data", (line) => {
    console.debug("stderr: ", line)
    testnet_log.update((state) => state + `${line}<br/>`)
  }) */
}
