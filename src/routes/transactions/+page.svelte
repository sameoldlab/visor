<script lang="ts">
  import { blocks, live } from "../../anvil"
  import Stat from "$lib/Stat.svelte"
  import { client } from "$lib/clients/public"
  import type { Transaction, Address } from "viem"
  import { formatEther } from "viem"

  let transactions: Transaction[] = []

  // Currently recreating the entire array every update. Not good for performance.
  blocks.subscribe((val) => {
    let hashes: Address[]
    hashes = val.reduce((accumulate: Address[], current) => {
      if (current.transactions) accumulate.push(...current.transactions)
      return accumulate
    }, [])
    void resolve(hashes)
  })

  async function resolve(hashes: Address[]) {
    transactions = await Promise.all(
      hashes.map(async (hash) => await client.getTransaction({ hash: hash }))
    )
  }
  let mock = [
    {
      accessList: [],
      blockHash:
        "0x840a7f1275b130599e5b03688718d2939e63613a5d8614e9841d77c7875f71a6",
      blockNumber: 1n,
      chainId: 31337,
      from: "0x70997970c51812dc3a010c7d01b50e0d17dc79c8",
      gas: 21000n,
      gasPrice: 4000000000n,
      hash: "0x07fe94672df3644359f23fc494230452f1523355c2535113b056f751d229312e",
      input: "0x",
      maxFeePerGas: 5000000000n,
      maxPriorityFeePerGas: 3000000000n,
      nonce: 0,
      r: "0x45b163f7c444ec30277af999340ce8d3690c1e442299a18b1b85a3712fff3bb5",
      s: "0x5ffa20abd6a3a64ba59efe924e18bc89c54726236d571202f923804f6c69ddeb",
      to: "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
      transactionIndex: 0,
      type: "eip1559",
      v: 1n,
      value: 1000000000000000000n,
    },
    {
      accessList: [],
      blockHash:
        "0x840a7f1275b130599e5b03688718d2939e63613a5d8614e9841d77c7875f71a6",
      blockNumber: 1n,
      chainId: 31337,
      from: "0x70997970c51812dc3a010c7d01b50e0d17dc79c8",
      gas: 21000n,
      gasPrice: 4000000000n,
      hash: "0x45b163f7c444ec30277af999340ce8d3690c1e442299a18b1b85a3712fff3bb5",
      input: "0x",
      maxFeePerGas: 5000000000n,
      maxPriorityFeePerGas: 3000000000n,
      nonce: 0,
      r: "0x45b163f7c444ec30277af999340ce8d3690c1e442299a18b1b85a3712fff3bb5",
      s: "0x5ffa20abd6a3a64ba59efe924e18bc89c54726236d571202f923804f6c69ddeb",
      to: "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
      transactionIndex: 0,
      type: "eip1559",
      v: 1n,
      value: 1000000000000000000n,
    },
  ]

  const trunc= (hex: string) => `${hex.slice(0, 6)}~${hex.slice(hex.length-5, hex.length-1)}`
</script>

<div class="container">
  <h1>Transactions</h1>
  <div class="box">
    {#each mock as { hash, nonce, from, blockHash, blockNumber,value, transactionIndex, r, s, to }}
      <div class= "item">
        <span class="data hash" >{hash}</span>
        <div class="stack text-right">
            <Stat title="To: " data={trunc(to)} border={false} stack={true}/>
            <Stat title="From: " data={trunc(from)} border={false} stack={true}/>
        </div>
        <div class="text-right">
            <Stat title="Value" data="{formatEther(value)}&nbsp;Ξ" border={false} />
        </div>
        <!-- <Stat title="S:" data={s} border={false} /> -->
        <!-- <Stat title="R:" data={r} border={false} /> -->
        <!-- <Stat title="R:" data={r} border={false} /> -->
      </div>
    {:else}
      <null />
    {/each}
  </div>
</div>

<style lang="scss">

  .hash {
    width: 33ch; /* 66 chars total */
    overflow-wrap: break-word;
  }

  .stack {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  
</style>
