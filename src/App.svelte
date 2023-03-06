<script lang="ts">
  import Accounts from "./views/Accounts.svelte"
  import Log from "./views/Log.svelte"
  import NavItem from "./lib/NavItem.svelte"

  import MainNav from "./views/MainNav.svelte"
  let view = "log"
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
      <button on:click={() => (view = "blocks")}
        ><NavItem name="blocks" /></button
      >
    </div>
  </nav>

  {#if view == "log"}
    <Log />
  {:else if view == "accounts"}
    <Accounts />
  {/if}
</main>

<style>
  nav {
    height: 100%;
    position: fixed;
    padding: 20px 5px;
    top: 0;
    left: 0;
    background-color: black;
    overflow-x: hidden;

    display: flex;
    flex-direction: column;
    gap: 5em;
  }
  nav > div {
    padding: 8px;
  }
  main {
    padding-left: 110px;
  }
</style>
