<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { Command } from "@tauri-apps/api/shell"
  import { killTestnet, live, startTestnet, testnet_log } from "../anvil"

  let stream: string = ""

  /*  async function startTestnet() {
    // process = await invoke("start_testnet", { args: ["-b", "3"] })
    const cmd: Command = new Command("anvil-cli", ["-b", "3"])
    let i = 0
    cmd.on("close", (data) => {
      console.debug(
        `command finished with code ${data.code} and signal ${data.signal}`
      )
      if (data.signal == 6) {
        stream =
          "anvil is already running. You can kill it from the command line with `pkill '^anvil$'`"
        PROCESS = null
      }
    })
    
    // data.signal 6 = error/failed to start
    // data.signal 15 = killed a running program
    
    cmd.on("error", (line) => {
      console.debug("stdout: ", line)
      stream += `${line}\n`
    })
    cmd.stderr.on("data", (line) => {
      console.debug("stdout: ", line)
      stream += `${line}<br/>`
    })
    cmd.stdout.on("data", (line) => {
      console.debug("stdout: ", line)
      stream += `${line}<br/>`
    })

    let child = await cmd.spawn()
    PROCESS = child.pid
    } */
</script>

<section class="container">
  {#if !$live}
    <button on:click={() => startTestnet()}>Start Anvil</button>
  {:else}
    <button on:click={killTestnet}>Stop Anvil</button>
  {/if}

  <br />
  <div class="term">
    <!-- \[+\d+m/g -->
    <p>
      {@html $testnet_log
        .replaceAll("[32m", `<span style='color: greenyellow'>`)
        .replaceAll("[0m", "</span>")}
    </p>
  </div>
</section>

<style lang="scss">
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
