import { Child, Command } from "@tauri-apps/api/shell"
import { writable } from "svelte/store"

// export const provider = new ethers.JsonRpcProvider()
// export const blocks = writable<Block[]>()

type NetworkState = {
  current_block: number
  base_fee: number
  gas_limit: number
  gen_timestamp: number
  rpc: string
  network_id: number
}

export const state = writable<NetworkState>({
  current_block: 60,
  base_fee: 10006000000,
  gas_limit: 300060000,
  gen_timestamp: 60,
  rpc: "127.0.0.1:8545",
  network_id: 57677,
})

/* export async function getState() {
  state.update((s) => {
    s.current_block = await provider._detectNetwork
    s.base_fee = await 
    s.gas_limit = await 
    s.gen_timestamp = await 
    s.rpc = await 
    s.network_id = await 
  })
} */

export const mnemomic = writable<string>(
  "test test test test test test test test test test test junk"
)
export const hd_path = writable<string>("m/44'/60'/0'/0/")

/**
 * Handle to command child process
 */
let _child: Child
export const killTestnet = () => {
  if (_child == null) return
  return _child.kill()
}

export let testnet_log = writable<string>("")
export let live = writable<boolean>(false)
export async function startTestnet(args: string[] = []) {
  args = ["--config-out", "../src/lib/anvil.json", ...args]
  // process = await invoke("start_testnet", { args: ["-b", "3"] })
  const cmd: Command = new Command("anvil-cli", args)
  let i = 0

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
             <br/>You can kill it from the command line with <code>pkill ^anvil$'</code>`
      )
    }
    testnet_log.update(
      (state) =>
        `${state} 
        <br/>command finished with code ${data.code} and signal ${data.signal}`
    )
    live.set(false)
    return
  })

  cmd.on("error", (line) => {
    console.debug("stderr: ", line)
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
        return `${line}<br/>`
      } else {
        return state + `${line}<br/>`
      }
    })
  })

  // Might not need this
  cmd.stderr.on("data", (line) => {
    console.debug("stderr: ", line)
    testnet_log.update((state) => state + `${line}<br/>`)
  })
}
