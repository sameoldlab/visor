<script lang="ts">
  import { killTestnet, live, startTestnet } from "/src/anvil"
  let parseArgs = ""
  const handleSubmit = () => {
    if (parseArgs.trim() == "") return startTestnet()

    const args = parseArgs.replace("anvil", "").trim().split(" ")
    return startTestnet(args)
  }
</script>

<section class="container">
  <h1>Settings</h1>
  Real GUI soon™
  <hr />
  <br />
  {#if !$live}
    <p class="title">Customize <code>`run`</code> command:</p>
    <form class="custom--run" on:submit|preventDefault={handleSubmit}>
      <input bind:value={parseArgs} placeholder="anvil" />
      <button type="submit">Run ▶</button>
    </form>

    <h4 id="examples">EXAMPLES</h4>
    <ol>
      <li>Set the number of accounts to 15 and their balance to 300 ETH</li>
    </ol>
    <pre><code class="language-sh"> --accounts 15 --balance 300
</code></pre>
    <ol start="2">
      <li>Choose the address which will execute the tests</li>
    </ol>
    <pre><code
        class="language-sh"> --sender 0xC8479C45EE87E0B437c09d3b8FE8ED14ccDa825E
</code></pre>
    <ol start="3">
      <li>Change how transactions are sorted in the mempool to FIFO</li>
    </ol>
    <pre><code class="language-sh"> --order fifo
</code></pre>

    <section>
      <!-- Todo: Change to inputs -->
      <h3 id="options">OPTIONS</h3>
      <h4 id="general-options">General Options</h4>
      <p>
        <code>-a, --accounts &lt;ACCOUNTS&gt;</code><br />
        Set the number of accounts [default: 10]
      </p>
      <p>
        <code>-b, --block-time &lt;block-time&gt;</code><br />
        Block time in seconds for interval mining
      </p>
      <p>
        <code>--balance &lt;BALANCE&gt;</code><br />
        Set the balance of the accounts [default: 10000]
      </p>
      <p>
        <code>--derivation-path &lt;DERIVATION_PATH&gt;</code><br />
        Set the derivation path of the child key to be derived [default: m/44'/60'/0'/0/]
      </p>
      <p>
        <code>-h, --help</code><br />
        Print help information
      </p>
      <p>
        <code>--hardfork &lt;HARDFORK&gt;</code><br />
        Choose the EVM hardfork to use [default: latest]
      </p>
      <p>
        <code>--init &lt;PATH&gt;</code><br />
        Initialize the genesis block with the given <code>genesis.json</code> file.
      </p>
      <p>
        <code>-m, --mnemonic &lt;MNEMONIC&gt;</code><br />
        BIP39 mnemonic phrase used for generating accounts
      </p>
      <p>
        <code>--no-mining</code><br />
        Disable auto and interval mining, and mine on demand instead
      </p>
      <p>
        <code>--order &lt;ORDER&gt;</code><br />
        How transactions are sorted in the mempool [default: fees]
      </p>
      <p>
        <code>-p, --port &lt;PORT&gt;</code><br />
        Port number to listen on [default: 8545]
      </p>
      <p>
        <code>--steps-tracing</code><br />
        Enable steps tracing used for debug calls returning geth-style traces [aliases:
        tracing]
      </p>
      <p>
        <code>--ipc [&lt;PATH&gt;]</code><br />
        Starts an IPC endpoint at the given <code>PATH</code> argument or the
        default path: unix:
        <code>tmp/anvil.ipc</code>, windows:
        <code>\\.\pipe\anvil.ipc</code>
      </p>
      <p>
        <code>--silent</code><br />
        Don't print anything on startup
      </p>
      <p>
        <code>--timestamp &lt;TIMESTAMP&gt;</code>
        Set the timestamp of the genesis block
      </p>
      <p>
        <code>-V, --version</code><br />
        Print version information
      </p>
      <h4 id="evm-options">EVM Options</h4>
      <p>
        <code>-f, --fork-url &lt;URL&gt;</code><br />
        Fetch state over a remote endpoint instead of starting from an empty state
      </p>
      <p>
        <code>--fork-block-number &lt;BLOCK&gt;</code><br />
        Fetch state from a specific block number over a remote endpoint (Must pass
        --fork-url in the same command-line)
      </p>
      <p>
        <code>--fork-retry-backoff &lt;BACKOFF&gt;</code><br />
        Initial retry backoff on encountering errors.
      </p>
      <p>
        <code>--retries &lt;retries&gt;</code><br />
        Number of retry requests for spurious networks (timed out requests). [default
        value= 5]
      </p>
      <p>
        <code>--timeout &lt;timeout&gt;</code><br />
        Timeout in ms for requests sent to remote JSON-RPC server in forking mode.
        [default value= 45000]
      </p>
      <p>
        <code>--compute-units-per-second &lt;CUPS&gt;</code><br />
        Sets the number of assumed available compute units per second for this provider
        [default value=330] See also, Alchemy Ratelimits
      </p>
      <p>
        <code>--no-rate-limit</code>
        Disables rate limiting for this node's provider. Will always override
        <code>--compute-units-per-second</code>
        if present. [default value= false] See also, Alchemy Ratelimits
      </p>
      <p>
        <code>--no-storage-caching&gt;</code><br />
        Explicitly disables the use of RPC caching. All storage slots are read entirely
        from the endpoint. This flag overrides the project's configuration file (Must
        pass --fork-url in the same command-line)
      </p>
      <h4 id="executor-environment-config">Executor Environment Config</h4>
      <p>
        <code>--base-fee &lt;FEE&gt;</code><br />
        <code>--block-base-fee-per-gas &lt;FEE&gt;</code><br />
        The base fee in a block
      </p>
      <p>
        <code>--chain-id &lt;CHAIN_ID&gt;</code><br />
        The chain ID
      </p>
      <p>
        <code>--code-size-limit &lt;CODE_SIZE&gt;</code><br />
        EIP-170: Contract code size limit in bytes. Useful to increase this because
        of tests. By default, it is 0x6000 (~25kb)
      </p>
      <p>
        <code>--gas-limit &lt;GAS_LIMIT&gt;</code><br />
        The block gas limit
      </p>
      <p>
        <code>--gas-price &lt;GAS_PRICE&gt;</code><br />
        The gas price
      </p>
      <h4 id="server-options">Server Options</h4>
      <p>
        <code>--allow-origin &lt;allow-origin&gt;</code><br />
        Set the CORS allow_origin [default: *]
      </p>
      <p>
        <code>--no-cors</code><br />
        Disable CORS
      </p>
      <p>
        <code>--host &lt;HOST&gt;</code><br />
        The IP address the server will listen on
      </p>
      <p>
        <code>--config-out &lt;OUT_FILE&gt;</code><br />
        Writes output of <code>anvil</code> as json to user-specified file
      </p>
      <p>
        <code>--prune-history</code><br />
        Don't keep full chain history
      </p>
    </section>
  {:else}
    <button on:click={killTestnet}>Stop Anvil</button>
  {/if}
</section>

<style lang="scss">
  form.custom--run {
    display: flex;
    width: 100%;
    border: 2px solid #606060;
    border-radius: 16px;
    overflow: hidden;
    box-shadow: 0px 4px 13px rgba(0, 0, 0, 0.25);

    input {
      flex-grow: 2;
      font-family: var(--mono);
      background: #343434;
      color: white;
      margin: 0;
      border: 0;
      padding: 0;
      padding-left: 24px;
    }
    button {
      color: white;
      background: #343434;
      margin: 0;
      border: 0;
    }
  }
</style>
