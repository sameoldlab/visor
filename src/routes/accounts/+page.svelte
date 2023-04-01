<script lang="ts">
  import Stat from "$lib/Stat.svelte"
  import getConfig from "$lib/getConfig"
  import type { Address } from "viem"
  import { formatEther } from "$lib/utils"
  import { client } from "$lib/clients/public"
  import { live, block_number } from "../../anvil"

  type Account = {
    address: Address
    key: string
    balance: bigint
    transaction_count: number
  }
  let accounts = <Account[]>[]

  const updAccount = async (address: Address) => {
    const balance = await client.getBalance({
      address,
      blockTag: "latest",
    })
    const transaction_count = await client.getTransactionCount({
      address,
      blockTag: "latest",
    })
    return { balance, transaction_count }
  }

  //   Update accounts data on each block
  block_number.subscribe(() => {
    const promise = accounts.map(async (current) => {
      const { balance, transaction_count } = await updAccount(current.address)
      return { ...current, balance, transaction_count }
    })

    Promise.all(promise)
      .then((res) => (accounts = res))
      .catch((err) => console.error(err))
  })

  const loadData = async () => {
    const data = await getConfig()

    const { available_accounts, private_keys, wallet } = data

    const promise = available_accounts.map((address, i) =>
      updAccount(address).then((res) => ({
        address,
        key: private_keys[i],
        balance: res.balance,
        transaction_count: res.transaction_count,
      }))
    )
    accounts = await Promise.all(promise)

    return { wallet }
  }
</script>

<div class="container">
  {#if $live}
    <div class="box">
      {#await loadData() then { wallet }}
        <div class="wallet-config">
          <Stat title="Mnemonic" data={wallet.mnemonic} grow={true} />
          <Stat title="HD Path" data={wallet.derivation_path} />
        </div>

        {#each accounts as { address, key, balance, transaction_count }, i}
          <div class="item p-base row">
            <div class="row">
              <span class="id">{i}</span>
              <div>
                <Stat title="Address" data={address} border={false} />
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
        {:else}
          loadConfigFromFile...
        {/each}
      {:catch}
        config not found
      {/await}
    </div>
  {/if}
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
