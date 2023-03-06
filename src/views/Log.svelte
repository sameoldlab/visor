<script lang="ts">
  import { appWindow, WebviewWindow } from "@tauri-apps/api/window"
  import { invoke } from "@tauri-apps/api/tauri"
  import { emit, listen } from "@tauri-apps/api/event"
  import { onMount } from "svelte"
  import { Command } from "@tauri-apps/api/shell"

  let stream: string = ""
  let PROCESS = 0

  onMount(async () => {
    await listen("anvil-aoutput", (data) => {
      console.log("anvil-output", data)
    })
  })

  async function startTestnet() {
    // process = await invoke("start_testnet", { args: ["-b", "3"] })
    const cmd: Command = new Command("anvil-cli")
    let i = 0;
    cmd.on("close", (data) => {
      console.log(
        `command finished with code ${data.code} and signal ${data.signal}`
      )
    })
    cmd.on("error", (line) => {
      console.log("stdout: ", line)
      stream += `${line}\n`
    })
    cmd.stderr.on("data", (line) => {
      console.log("stdout: ", line)
      stream += `${line}<br/>`
    })
    cmd.stdout.on("data", (line) => {
      console.log("stdout: ", line)
      stream += `${line}<br/>`
    })

    let child = await cmd.spawn()
    PROCESS = child.pid
  }
  async function stopTestnet() {
    PROCESS = await invoke("stop_testnet", { process: PROCESS })
    
  }

  // emit an event that are only visible to the current window
  /*   appWindow.emit("event", { message: "Tauri is awesome!" })
  // listen to the `click` event and get a function to remove the event listener
  // there's also a `once` function that subscribes to an event and automatically unsubscribes the listener on the first event
  // create a new webview window and emit an event only to that window
  const webview = new WebviewWindow("window")
  webview.emit("event")

  // emits the `click` event with the object payload
  emit("click", {
    theMessage: "Tauri is awesome!",
  })*/
  let live = false
</script>

<section class="container">
  {#if PROCESS == 0}
    <button on:click={startTestnet}>Start Anvil</button>
  {:else}
    <button on:click={stopTestnet}>Stop Anvil</button>
  {/if}
  <br />
  <div class="term">
    <!-- <p>running on process: {PROCESS}</p> -->
<p>running on process: 0

 

                              _   _
                             (_) | |
       __ _   _ __   __   __  _  | |
      / _` | | '_ \  \ \ / / | | | |
     | (_| | | | | |  \ V /  | | | |
      \__,_| |_| |_|   \_/   |_| |_|
 
     0.1.0 (9477764 2023-03-02T00:06:40.046592288Z)
     https://github.com/foundry-rs/foundry
 
 Available Accounts
 ==================
 
 (0) "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266" (10000 ETH)
 (1) "0x70997970C51812dc3A010C7d01b50e0d17dc79C8" (10000 ETH)
 (2) "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC" (10000 ETH)
 (3) "0x90F79bf6EB2c4f870365E785982E1f101E93b906" (10000 ETH)
 (4) "0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65" (10000 ETH)
 (5) "0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc" (10000 ETH)
 (6) "0x976EA74026E726554dB657fA54763abd0C3a0aa9" (10000 ETH)
 (7) "0x14dC79964da2C08b23698B3D3cc7Ca32193d9955" (10000 ETH)
 (8) "0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f" (10000 ETH)
 (9) "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720" (10000 ETH)
 
 Private Keys
 ==================
 
 (0) 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
 (1) 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
 (2) 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a
 (3) 0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6
 (4) 0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a
 (5) 0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba
 (6) 0x92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e
 (7) 0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356
 (8) 0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97
 (9) 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
 
 Wallet
 ==================
 Mnemonic:          test test test test test test test test test test test junk
 Derivation path:   m/44'/60'/0'/0/
 
 
 Base Fee
 ==================
 [32m
 1000000000[0m
 
 Gas Limit
 ==================
 [32m
 30000000[0m
 
 Genesis Timestamp
 ==================
 [32m
 1678044775[0m
 
 Listening on 127.0.0.1:8545</p>
    <p>{@html stream}</p>
  </div>
</section>

<style>
  section {
    padding: 16px;
  }
  .term {
    padding: 16px;
    background-color: #010101;
    border-radius: 1em;
    text-align: left;
    overflow: scroll;
    height: 100%;
    color: #d3d3d3e3;
    white-space: pre-wrap;
    font-family: monospace;
  }
</style>
