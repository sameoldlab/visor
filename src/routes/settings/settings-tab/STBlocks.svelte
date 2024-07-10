<script lang="ts">
  import EvmOpts from "../settings"
  import MenuItem from "../MenuItem.svelte"

  let parseArgs = ""

  $: runCmd = EvmOpts.reduce((acc, curr) => {
    if (curr.val) return [...acc, curr.args, curr.val]
    return acc
  }, [])
  let blockTime: number,
    noMining: boolean,
    order: boolean,
    genStamp: number,
    baseFee: number,
    chainId: number,
    gasLimit: number,
    gasPrice: number,
    codeSize: number

  let automine = true

  const GeneralOpts = [
    { arg: "--block-time", value: blockTime },
    { arg: "--no-mining", value: noMining },
    { arg: "--order", value: order },
    { arg: "-genesis-timestamp", value: genStamp },
    { arg: "-base-fee", value: baseFee },
    { arg: "-chain-id", value: chainId },
    { arg: "-gas-limit", value: gasLimit },
    { arg: "-gas-price", value: gasPrice },
    { arg: "-code-size", value: codeSize },
  ]
</script>

<MenuItem
  title="Automine"
  description="Process transactions instantly"
  type={true}
  bind:val={automine}
  />
  {#if !automine}
  <MenuItem
  title="Block Time"
  description="Block time in seconds for interval mining."
  type={0}
  bind:val={blockTime}
  />
{/if}
<MenuItem
  title="Manual Mining"
  description="Disable automining and interval mining. Mine new block the balance of the accounts. Default: 10000"
  type={false}
  bind:val={noMining}
/>

<br />
<hr />
<MenuItem
  title="Genesis Timestamp"
  description="Set the timestamp of the genesis block"
  type={0}
  bind:val={genStamp}
/>
<MenuItem
  title="Base Fee"
  description=""
  type={1000000000}
  bind:val={baseFee}
/>
<MenuItem title="Chain ID" description="" type={31337} bind:val={chainId} />

<MenuItem
  title="Gas Limit"
  description="The block gas limit"
  type={30000000}
  bind:val={gasLimit}
/>


<MenuItem
  title="Gas Price"
  description="Mnemonic phrase used to generate accounts"
  type={1875000000}
  bind:val={gasPrice}
/>

<MenuItem
  title="Code Size"
  description="EIP-170: Contract code size limit in bytes. Useful to increase this because of tests. By default, it is 0x6000 (~25kb)"
  type={0}
  bind:val={codeSize}
/>