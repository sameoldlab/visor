<script lang="ts">
  import Stat from "$lib/Stat.svelte"
  import configJson from "$lib/anvil.json"
  import { getAddress } from "viem"
  import { formatEther } from "$lib/utils"
  import { client } from "$lib/clients/public"
  import { onMount } from "svelte"
  import { live } from "../../anvil"

  let { available_accounts, private_keys, wallet } = configJson
  type Account = {
    addr: string
    key: string
    balance: bigint
    transaction_count: number
  }
  let accounts = <Account[]>[]

  onMount(async () => {
    if (!$live) return
    for (let i = 0, l = available_accounts.length; i < l; i++) {
      const balance = await client.getBalance({
        address: getAddress(available_accounts[i]),
        blockTag: "latest",
      })
      const transaction_count = await client.getTransactionCount({
        address: getAddress(available_accounts[i]),
        blockTag: "latest",
      })

      accounts[i] = {
        addr: available_accounts[i],
        key: private_keys[i],
        balance: balance,
        transaction_count: transaction_count,
      }
    }
  })
</script>

<div class="container">
  <div class="box">
    <div class="wallet-config">
      <!-- <div class="grow0">Hello</div> -->
      <!-- <div class="grow1">Stronk</div> -->
      <Stat title="Mnemonic" data={wallet.mnemonic} grow={true} />
      <Stat title="HD Path" data={wallet.derivation_path} />
    </div>

    {#each accounts as { addr, key, balance, transaction_count }, i}
      <div class="item p-base row">
        <div class="row">
          <span class="id">{i}</span>
          <div>
            <Stat title="Address" data={addr} border={false} />
          </div>
        </div>
        <div class="row text-right">
          <!-- <Stat -->
          <!-- title="Balance" -->
          <!-- data="ETH {formatEther(balance)}" -->
          <!-- border={false} -->
          <!-- /> -->
          <div class="stack">
            <span class="title">Balance</span>
            <span
              ><span class="data">{formatEther(balance, 2)}</span>
              <span class="eth">ETH</span></span
            >
          </div>
          <Stat title="Nonce" data={transaction_count} border={false} />
        </div>
        <!-- <Stat title="Key" data={key} border={false} /> -->
        <!--         <div>
          <span class="title"> Key</span>
          <span class="data">{key}</span>
        </div> -->
      </div>
    {/each}
  </div>
</div>

<style lang="scss">
  .wallet-config {
    display: flex;
    flex-direction: row;
    margin: -1px;
  }
  .eth {
    opacity: 0.4;
    font-size: 12px;
    // margin-left: -8px;
    font-weight: 700;
    font-stretch: expanded;
    font-synthesis: 700 normal true;
  }
</style>
