<script lang="ts">
  import Stat from "$lib/Stat.svelte"
  import { blocks } from "../../anvil"
  import { naturalDate } from "$lib/utils"

  let tx = <
    {
      view: boolean
      index: number | null
    }
  >{
    view: false,
    index: null,
  }

  function viewTx(i: number) {
    if (i === tx.index) {
      tx = { view: false, index: null }
      return
    }
    tx = { view: true, index: i }
  }
</script>

<div class="container">
  <h1>Blocks</h1>
  <div class="box">
    {#each $blocks as { number, hash, timestamp, transactions }, i}
      <button class="item row p-base" on:click={() => viewTx(i)}>
        <div class="row row-tight">
          <span class="id">{number?.toString()}</span>
          <!-- move or truncate. breaks layout after 4 digits -->
          <div class="hash">
            <Stat title="Hash" data={hash} border={false} grow={true} />
          </div>
        </div>
        <div class="stack">
          <span class="title">On</span>
          <span class="data">{naturalDate(timestamp)}</span>
        </div>
        {#if transactions.length > 0}
          <div class="stack view-tx text-right">
            <span class="title">TX Count</span>
            <span class="data">{transactions.length}</span>
            <!-- Should send tblock has to a function that-->
          </div>
          {:else}
          <span class="title">empty</span>

        {/if}
      </button>

      {#if tx.view && i === tx.index && transactions.length > 0}
        <div class="stack p-base">
          <div class="tx">
            <p class="title">Transactions</p>
            {#each transactions as transaction}
              <p class="data tx">{transaction}</p>
              <!-- <Stat title="Transaction" data={transaction} border={false} /> -->
            {/each}
          </div>
        </div>
      {/if}
    {/each}
  </div>
</div>

<!-- 
baseFeePerGas: 10293n
difficulty: 0n
extraData: "0x"
gasLimit: 30000000n
gasUsed: 0n
hash: "0xcb752c8d61683b861564897eb42a4f5cd41fa99f5995d3f4cb0265bdedb1c82c"
logsBloom: "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000…"
miner: "0x0000000000000000000000000000000000000000"
mixHash: "0x0000000000000000000000000000000000000000000000000000000000000000"
nonce: "0x0000000000000000"
number: 87n
parentHash: "0x38266c868a5e9c26c56140f21983da394c44dc011d5be5af62b75b9585fed61e"
receiptsRoot: "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
sealFields: ["0x0000000000000000000000000000000000000000000000000000000000000000", "0x0000000000000000"] (2)
sha3Uncles: "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
size: 512n
stateRoot: "0x10baabfe446f34ffd15cf66bb1d4969f4499deea67207244c6a675ba2b5a6b88"
timestamp: 1678722441n
totalDifficulty: 0n
transactions: [ "0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2", ...]
transactionsRoot: "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
uncles: [] (0)
 -->
<style lang="scss">
  .hash {
    width: 37ch; /* 66 chars total */
    overflow-wrap: break-word;
  }
  .block {
    // justify-content: space-between;
    align-items: center;

    transition: all;
    transition-duration: 200ms;
  }
  .stack {
    display: flex;
    flex-direction: column;
    gap: 4px;
    // padding-bottom: 20px;
  }

  .tx {
    width: 100%;
    overflow-wrap: break-word;
  }
</style>
