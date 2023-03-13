<script lang="ts">
  import Accounts from "./views/Accounts.svelte"
  import Log from "./views/Log.svelte"
  import NavItem from "./lib/Icons.svelte"
  import TopStats from "./views/ConfigData.svelte"
  import { killTestnet, live, startTestnet } from "./anvil"
  import Settings from "./views/Settings.svelte"
  import Blocks from "./views/Blocks.svelte"
  import { slide } from "svelte/transition"

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
  {#if $live}
    <div transition:slide>
      <TopStats />
    </div>
  {/if}
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
      padding: 16px 8px;

      &:hover {
        opacity: 1;
      }
    }
  }
  main {
    margin-left: 48px; //48 = (56) - 8 = (m4 + p16 + w36 ) - 8
    margin-top: -8px;
    margin-right: -8px;
  }
</style>
