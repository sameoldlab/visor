<script lang="ts">
    import {createPublicClient, http} from "viem"
    import {mainnet, } from "viem/chains"


  import configJson from "../lib/anvil.json"
    const client = createPublicClient({
        chain: mainnet,
        transport: http(`http://localhost:8545`)
    })
  let { available_accounts, private_keys, wallet } = configJson
  available_accounts.forEach(acc => {
    
  })
</script>
<h1>Accounts</h1>
