<script lang="ts">
  import Accounts from "./views/Accounts.svelte"
  import Log from "./views/Log.svelte"
  import NavItem from "./lib/NavItem.svelte"
  import MainNav from "./views/MainNav.svelte"
  import TopStats from "./views/TopStats.svelte"
  import { live, startTestnet } from "./anvil"
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
    {#if $live}
    <button on:click={() => startTestnet()}>
      <NavItem name="start" />
    </button>
    {/if}
  </div>
</nav>

<main>
  <TopStats />
  <div>
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
  :root {
    --backgroud: #111111;
    --border: #373737;
    --mono: "Fira Code", "Source Code Pro", "Fira Mono", monospace;
    background-color: var(--backgroud);
    color: white;

    button {
      cursor: pointer;
    }
  }

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
    border-right: 1px solid var(--border);
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
      cursor: pointer;
      border: 0;
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
