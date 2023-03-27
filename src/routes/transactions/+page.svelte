<script lang="ts">
  import { blocks, live } from "../../anvil"
  import Stat from "$lib/Stat.svelte"
  import { client } from "$lib/clients/public"
  import type { Transaction, Address } from "viem"
  import { formatEther } from "viem"
  import { trunc, naturalDate } from "$lib/utils"
  import { fade, slide, blur } from "svelte/transition"

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
      input:
        "0x45b163f7c444ec30277af999340ce8d3690c1e442x5ffa20abd6a3a64ba59efe924e18bc89c547262360xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
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

  let active: null | number = null
  const setActive = (id: number) => (active = active === id ? null : id)
</script>

<div class="container">
  <h1>Transactions</h1>
  <div class="box">
    {#each transactions as { hash, from, to, value, ...t }, id}
      <button class="item row p-base" on:click={() => setActive(id)} in:fade>
        <span class="data hash">{hash}</span>
        <div class="text-right">
          <Stat title="To: " data={trunc(to)} border={false} unstack={true} />
          <Stat
            title="From: "
            data={trunc(from)}
            border={false}
            unstack={true}
          />
        </div>
        <div class="text-right">
          <Stat
            title="Value"
            data="{formatEther(value)}&nbsp;Ξ"
            border={false}
          />
        </div>
      </button>
      {#if active === id}
        <div class="stack p-base">
          <div class="row">
            <Stat title="Block #" data={t.blockNumber} border={false} />
            <Stat
              title="Timestamp"
              data={naturalDate($blocks[Number(t.blockNumber) - 1]?.timestamp)}
              border={false}
            />
            <div class=" text-right">
              <Stat title="TX Type" data={t.type} border={false} />
            </div>
          </div>
          <div class="row">
            <Stat title="Gas Used" data={t.gas} border={false} />
            <Stat title="Gas Price" data={t.gasPrice} border={false} />

            <div class=" text-right">
              <Stat title="Gas Limit" data={t.maxFeePerGas} border={false} />
            </div>
            <!-- <Stat title="Block Number" data={t.blockNumber} border={false} /> -->
          </div>
          {#if t.input !== "0x"}
            <div class="row">
              <p class="data tx-input">
                {t.input}
              </p>
              <!-- <Stat title="Input" data={t.input} border={false} /> -->
            </div>
          {/if}
        </div>
      {/if}
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


  .row {
    overflow-wrap: break-word;
  }

  .tx-input {
    width: 100%;
  }
</style>
