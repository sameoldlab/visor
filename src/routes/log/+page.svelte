<script lang="ts">
  import { testnet_log } from '../../anvil'
</script>

<section class="container">
  <h1>Log</h1>
</section>
<div class="term">
  <!-- \[+\d+m/g -->
  <p>
    {@html $testnet_log
      .replaceAll("[32m", `<span style='color: greenyellow'>`)
      .replaceAll("[0m", "</span>")
      .replaceAll("eth_getBlockByNumber<br/>", "")}
  </p>
</div>

<style lang="scss">
  .term {
    margin: 8px;
    padding: 4px 20px;
    background-color: #010101;
    border-radius: 8px;
    text-align: left;
    overflow: scroll;
    height: 100%;
    color: #d3d3d3e3;
    white-space: pre-wrap;
    font-family: monospace;
  }
</style>
