<script lang="ts">
  import Stat from "../lib/Stat.svelte"
  import configJson from "../lib/anvil.json"
  import { formatEther, getAddress } from "viem"
  import { client } from "../lib/clients/public"
  import { onMount } from "svelte"
  import { live } from "../anvil"

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
  <h1>Accounts</h1>
  <div class="box">
    <div class="wallet-config">
      <Stat title="Mnemonic" data={wallet.mnemonic} grow={true} />
      <Stat title="HD Path" data={wallet.derivation_path} />
    </div>

    {#each accounts as { addr, key, balance, transaction_count }, i}
      <div class="address">
        <span class="addr--id">{i}</span>
        <div>
          <Stat title="Address" data={addr} border={false} />
        </div>
        <Stat title="Balance" data="Ξ {formatEther(balance)}" border={false} />
        <Stat title="TX Count" data={transaction_count} border={false} />

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
  }
  .box {
    background: #000;
    // padding: 2em;
    border: var(--border);
    border-radius: 8px;
    overflow: hidden;
  }
  .address {
    padding: 20px;
    display: flex;
    flex-direction: row;
    gap: 24px;
    // justify-content: space-between;
    align-items: center;
  }
  .addr--id {
    padding: 8px 12px;
    text-align: center;
    font-weight: 800;
    font-weight: 800;
    border: 1px solid white;
    border-radius: 8px;
  }
</style>
