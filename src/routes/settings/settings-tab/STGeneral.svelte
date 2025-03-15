<script lang="ts">
  import EvmOpts from "../settings"
  import MenuItem from "../MenuItem.svelte"

  let parseArgs = ""

  let runCmd = $derived(EvmOpts.reduce((acc, curr) => {
    if (curr.val) return [...acc, curr.args, curr.val]
    return acc
  }, []))
  let accounts: number = $state(),
    balance: number = $state(),
    hdPath: string,
    mnemonic: string = $state(),
    noMining: boolean
  let automine = true

  const GeneralOpts = [
    { arg: "--accounts", value: accounts },
    { arg: "--balance", value: balance },
    { arg: "--derivatioin-path", value: hdPath },
    { arg: "-mnemonic", value: mnemonic },
    { arg: "-a", value: accounts },
  ]
</script>

<MenuItem
  title="Number of accounts"
  description="Set the number of accounts. Default: 10"
  type={10}
  bind:val={accounts}
/>
<MenuItem
  title="Account Balance"
  description="Set the balance for each account in ETH"
  type={10000}
  bind:val={balance}
/>

<MenuItem
  title="Seed Phrase"
  description="Mnemonic phrase used to generate accounts"
  type={"test test test test test test test test test test test junk"}
  bind:val={mnemonic}
/>
<MenuItem
  title="Derivation Path"
  description="Set the derivation path of the child key to be derived [default: m/44'/60'/0'/0/]"
  type={"m/44'/60'/0'/0/"}
/>
<br />
<hr />


<!-- 
--base-fee <FEE>
--block-base-fee-per-gas <FEE>
The base fee in a block

--chain-id <CHAIN_ID>
The chain ID

--code-size-limit <CODE_SIZE>
EIP-170: Contract code size limit in bytes. Useful to increase this because of tests. By default, it is 0x6000 (~25kb)

--gas-limit <GAS_LIMIT>
The block gas limit

--gas-price <GAS_PRICE>
The gas price

 -->
<style>
    .row {
        display: flex;
        justify-content: space-between;
    }
</style>