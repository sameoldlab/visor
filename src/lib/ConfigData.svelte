<script lang="ts">
  import { block_number } from "../anvil"
  import getConfig from "$lib/getConfig"
  import Stat from "$lib/Stat.svelte"
  import { onMount } from "svelte"

  let base_fee: string, gas_limit: string, genesis_timestamp: string, gas_price: string

  onMount(async () => {
    // Read the text file in the `$APPCONFIG/app.conf` path
    const configJson = await getConfig()
    if (!configJson) return
    ;({ base_fee, gas_limit, genesis_timestamp, gas_price } = configJson) //check if these change when integrating test client

  })

  let network_id = "31337" //get from viem
  let rpc = "localhost:8545" //get from settings config

  $: chain_state = [
    { title: "Block #", data: $block_number, grow: false },
    { title: "Base Fee", data: base_fee, grow: false },
    { title: "Gas Limit", data: gas_limit, grow: false },
    { title: "Gas Price", data: gas_price, grow: false },
    { title: "Genesis Stamp", data: genesis_timestamp, grow: true },
    { title: "Chain", data: network_id, grow: false },
    { title: "RPC Server", data: rpc, grow: true },
  ]
</script>

<div class="contain sticky">
  {#each chain_state as state}
    <Stat {...state} />
  {/each}
</div>

<style lang="scss">
  .contain {
    display: flex;
    flex-direction: row;
    width: 100%;
    background: #000;
  }

  .sticky {
    position: sticky;
    top: 0;
    width: 100%;
  }
</style>
