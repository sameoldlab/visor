<script lang="ts">
  import { onDestroy } from "svelte"
  import { block_number } from "../anvil"
  import { client, INTERVAL } from "$lib/clients/public"
  import configJson from "$lib/anvil.json"
  import Stat from "$lib/Stat.svelte"

  let { base_fee, gas_limit, genesis_timestamp } = configJson

  let network_id = "0"

  $: chain_state = [
    { title: "Current Block", data: $block_number },
    { title: "Base Fee", data: base_fee },
    { title: "Gas Limit", data: gas_limit },
    { title: "Genesis Timestamp", data: genesis_timestamp },
    // { title: "RPC Server", data: rpc },
    { title: "Network ID", data: network_id },
  ]
</script>

<div>
  {#each chain_state as { title, data }}
    <Stat {title} {data} />
  {/each}
</div>

<style lang="scss">
  div {
    display: flex;
    gap: 0;
    width: 100%;

    background: #000;
  }
</style>
