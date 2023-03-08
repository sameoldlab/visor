<script lang="ts">
  import Stat from "../lib/Stat.svelte"
  import configJson from "../lib/anvil.json"
  import { createPublicClient, http } from "viem"
  import { mainnet } from "viem/chains"

  const client = createPublicClient({
    chain: mainnet,
    transport: http(`http://localhost:8545`),
  })

  let { available_accounts, private_keys, wallet } = configJson
  type Account = [{ addr?: string; key?: string }]

  let accounts: Account = [{}]
  for (let i = 0, l = available_accounts.length; i < l; i++) {
    accounts[i] = { addr: available_accounts[i], key: private_keys[i] }
  }
</script>

<div class="container">
  <h1>Accounts</h1>
  <div class="box">
    <div class="wallet-config">
      <Stat title="Mnemonic" data={wallet.mnemonic} grow={true} />
      <Stat title="HD Path" data={wallet.derivation_path} />
    </div>

    {#each accounts as { addr, key }, i}
      <div class="address">
        <span class="addr--id">{i}</span>
        <div>
          
          <Stat title="Address" data={addr} border={false} />
          <!-- <Stat title="Key" data={key} border={false} /> -->
        </div>
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
