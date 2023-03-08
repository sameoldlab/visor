<script lang="ts">
  import Stat from "../lib/Stat.svelte"
  import { state } from "../anvil"
  import { ethers } from "ethers"
  import { onMount } from "svelte"
  import configJson from "../lib/anvil.json"

  
  let { base_fee, gas_limit, genesis_timestamp } = configJson

  let current_block = 0
  let network_id = "0"

  const provider = new ethers.JsonRpcProvider()

  onMount(async () => {
    const network = await provider.getNetwork()
    current_block = await provider.getBlockNumber()
    network_id = await network.chainId.toString()
  })

  $: chain_state = [
    { title: "Current Block", data: current_block },
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
    padding: 8px 0px;

    background: #000;
    
  }
</style>
