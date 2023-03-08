<script lang="ts">
  import Accounts from "./views/Accounts.svelte"
  import Log from "./views/Log.svelte"
  import NavItem from "./lib/Icons.svelte"
  import TopStats from "./views/ConfigData.svelte"
  import { killTestnet, live, startTestnet } from "./anvil"
  import Settings from "./views/Settings.svelte"
  import Blocks from "./views/Blocks.svelte"

  let view = "settings"
</script>

<nav>
  <div class="nav--main">
    <button on:click={() => (view = "accounts")}>
      <NavItem name="accounts" />
    </button>
    <button on:click={() => (view = "log")}>
      <NavItem name="log" />
    </button>
    <button on:click={() => (view = "blocks")}>
      <NavItem name="blocks" />
    </button>
  </div>
  <div class="nav--end">
    <button on:click={() => (view = "settings")}>
      <NavItem name="settings" />
    </button>
    {#if !$live}
      <button on:click={() => startTestnet()}>
        <NavItem name="start" />
      </button>
    {:else}
      <button on:click={killTestnet}>
        <NavItem name="pause" />
      </button>
    {/if}
  </div>
</nav>

<main>
  <TopStats />
  <div>
    <!-- Todo: Routing  -->
    {#if view == "log"}
      <Log />
    {:else if view == "accounts"}
      <Accounts />
    {:else if view == "blocks"}
      <Blocks />
    {:else if view == "settings"}
      <Settings />
    {/if}
  </div>
</main>

<style lang="scss">
  body {
    margin: 0;
    overflow-x: hidden;
  }

  nav {
    height: 100%;
    position: fixed;
    top: 0;
    left: 0;

    background-color: black;
    border-right: var(--border);
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    .nav--main {
      //   padding: 10px 0;
      flex-direction: column;
      display: flex;
    }

    .nav--end {
      //   padding: 10px 0;
      display: flex;
      flex-direction: column;
    }

    button {
      display: block;
      background: transparent;
      opacity: 0.7;
      padding: 16px 12px;

      &:hover {
        opacity: 1;
      }
    }
  }
  main {
    margin-left: 56px;
    margin-top: -8px;
    margin-right: -8px;
  }
</style>
