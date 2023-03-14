<script lang="ts">
  import { blocks } from "../anvil"

  let naturalDate = (unix_time) => new Date(unix_time * 1000).toLocaleString() // Internationalization for freee? Should probably test this
</script>

<div class="container">
  <h1>Blocks</h1>
  {#each $blocks as { number, hash, timestamp }, i}
    <p>{number}</p>
    <p>Hash: {hash}</p>
    <p>Timestamp: {naturalDate(timestamp)}</p>
    <br />
  {/each}
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
transactions: [] (0)
transactionsRoot: "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
uncles: [] (0)
 -->
