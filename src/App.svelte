<script lang="ts">
  import Accounts from "./views/Accounts.svelte"
  import Log from "./views/Log.svelte"
  import NavItem from "./lib/NavItem.svelte"
  import MainNav from "./views/MainNav.svelte"
  import TopStats from "./views/TopStats.svelte"
  
  let view = ""
</script>

<main>
  <nav>
    <div>
      <button on:click={() => (view = "accounts")}
        ><NavItem name="accounts" /></button
      >
    </div>
    <div>
      <button on:click={() => (view = "log")}> <NavItem name="log" /></button>
    </div>
    <div>
      <button on:click={() => (view = "blocks")}>
        <NavItem name="blocks" />
      </button>
    </div>
  </nav>

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
    padding: 30px 5px;
    top: 0;
    left: 0;
    background-color: black;

    border-right: 1px solid var(--border);

    display: flex;
    flex-direction: column;
    gap: 2.5em;

    button {
      background: transparent;
      cursor: pointer;
      border: 0;
      opacity: 0.7;

      &:hover {
        opacity: 1;
      }
    }

    div {
      padding: 8px;
    }
  }
  main {
    padding-left: 110px;
  }
</style>
