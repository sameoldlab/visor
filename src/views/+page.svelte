<script lang="ts">
  import { appWindow, WebviewWindow } from "@tauri-apps/api/window"
  import { invoke } from "@tauri-apps/api/tauri"
  import { emit, listen } from "@tauri-apps/api/event"
  import { onMount } from "svelte"
  
  let anvil = `{"currentBlockNumber":"0x132","currentBlockTimestamp":1677855499,"currentBlockHash":"0xffec2778f31af827f1bdcce638f3adaebe5542678178ce7f5021aa7b794f6005","hardFork":"LATEST","transactionOrder":"fees","environment":{"baseFee":"0x0","chainId":"0x7a69","gasLimit":"0x1c9c380","gasPrice":"0x3b9aca00"},"forkConfig":{"forkUrl":null,"forkBlockNumber":null,"forkRetryBackoff":null}}`
  let process = 0
  
  onMount(async () => {
    await listen("anvil-aoutput", (data) => {
      console.log("anvil-output", data)
    })
  })
  async function startTestnet() {
    process = await invoke("start_testnet", { args: ["-b", "3"] })
  }
  async function stopTestnet() {
    process = await invoke("stop_testnet", { process: process })
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

<main class="container">
  {#if process == 0}
    <button on:click={startTestnet}>Start Anvil</button>
  {:else}
    <button on:click={stopTestnet}>Stop Anvil</button>
  {/if}
  <br />
  <div class="term">
    <p>Anvil running on process: {process}</p>
    <p class="term">Anvil Output: {anvil}</p>
  </div>
</main>

<style>
  .term {
    padding: 16px;
    background-color: #111;
    border-radius: 1em;
    text-align: left;
    overflow: auto;
    color: #d3d3d3e3;
  }
</style>
