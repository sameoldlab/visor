<script lang="ts">
  import Stat from "../lib/Stat.svelte"
  import { state } from "../anvil"
  import { ethers } from "ethers"
  import { onMount } from "svelte"

  let blockNumber = 10,
    chainID,
    net: JSON
  const provider = new ethers.JsonRpcProvider()

  const loadb = async () => {
    let {baseFeePerGas: $base_fee, gasLimit, gasUsed, transactions} = await provider.getBlock(0)
    const network = await provider.getNetwork()
    let nodeInfo = await provider.send("anvil_nodeInfo", [])
    blockNumber = await provider.getBlockNumber()
    chainID = await network.chainId
    
    console.log(`Block: `, $base_fee, gasLimit, gasUsed)
    console.log(nodeInfo)
    console.log("provider: ", provider)
    console.log("net: ", net)
    console.log("chainID: ", chainID)
  }

  $: chain_state = [
    { title: "Current Block", data: `${$state.current_block}` },
    { title: "Base Fee", data: `${$state.base_fee}` },
    { title: "Gas Limit", data: `${$state.gas_limit}` },
    { title: "Genesis Timestamp", data: `${$state.gen_timestamp}` },
    { title: "RPC Server", data: `${$state.rpc}` },
    { title: "Network ID", data: `${$state.network_id}` },
  ]
</script>

<div>
  {#each chain_state as { title, data }}
    <Stat {title} {data} />
  {/each}
</div>

<button on:click={loadb}>Get Block</button>
{blockNumber}

<style lang="scss">
  div {
    display: flex;
    gap: 0;
  }
</style>
