<script lang="ts">
  import Accounts from "./views/Accounts.svelte"
  import Log from "./views/Log.svelte"
  import NavItem from "./lib/NavItem.svelte"
  import MainNav from "./views/MainNav.svelte"
  import TopStats from "./views/TopStats.svelte"

  let view = ""
</script>

<nav>
  <button on:click={() => (view = "accounts")}>
    <NavItem name="accounts" />
  </button>

  <button on:click={() => (view = "log")}>
    <NavItem name="log" />
  </button>

  <button on:click={() => (view = "blocks")}>
    <NavItem name="blocks" />
  </button>
</nav>
<main>
  {#if view == "log"}
    <Log />
  {:else}
    <TopStats />
    {#if view == "accounts"}
      <Accounts />
    {/if}
  {/if}
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

  nav {
    height: 100%;
    position: fixed;
    padding: 30px 0;
    top: 0;
    left: 0;
    background-color: black;

    border-right: 1px solid var(--border);

    display: flex;
    flex-direction: column;

    button {
      background: transparent;
      cursor: pointer;
      border: 0;
      opacity: 0.7;
      padding: 20px 12px;

      &:hover {
        opacity: 1;
      }
    }

    div {
      padding: 8px;
    }
  }
  main {
    margin-left: 69px;
  }
</style>
