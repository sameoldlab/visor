<script lang="ts">
  import { block_number } from "../anvil"
  import configJson from "$lib/anvil.json"
  import Stat from "$lib/Stat.svelte"

  let { base_fee, gas_limit, genesis_timestamp, gas_price } = configJson //check if these change when integrating test client

  let network_id = "31337" //get from viem
  let rpc = "localhost:8545" //get from config

  $: chain_state = [
    { title: "Current Block", data: $block_number, grow: false },
    { title: "Base Fee", data: base_fee, grow: false },
    { title: "Gas Limit", data: gas_limit, grow: false },
    { title: "Gas Price", data: gas_price, grow: false },
    { title: "Genesis Timestamp", data: genesis_timestamp, grow: true },
    { title: "Chain ID", data: network_id, grow: false },
    { title: "RPC Server", data: rpc, grow: true },
  ]
</script>

<div class="contain">
  {#each chain_state as state}
    <Stat {...state} />
  {/each}
</div>

<style lang="scss">
  div.contain {
    display: flex;
    flex-direction: row;
    gap: 0;
    width: 100%;

    background: #000;
  }
</style>
