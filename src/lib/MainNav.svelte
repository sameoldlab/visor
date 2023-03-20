<script lang="ts">
  import NavItem from "$lib/Icons.svelte"
  import { page } from "$app/stores"
  import { live, killTestnet, startTestnet } from "../anvil"

  const routes = ["log", "accounts", "blocks", "transactions",  "settings"]
</script>

<nav>
  <div class="nav--main">
    {#each routes as route}
      {@const active = $page.url.pathname?.includes(route)}
      <a
        class="nav-item"
        class:active
        href="./{route}"
      >
        <NavItem name={route} {active} />
      </a>
    {/each}
  </div>

  <div class="nav--end">
    <!--     <a class="nav-item" href="./settings">
      <NavItem name="settings" active={$page.url.pathname?.includes("settings")} />
    </a> -->
    {#if !$live}
      <button class="nav-item" on:click={() => startTestnet()}>
        <NavItem name="start" />
      </button>
    {:else}
      <button class="nav-item" on:click={killTestnet}>
        <NavItem name="pause" />
      </button>
    {/if}
  </div>
</nav>

<style lang="scss">
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
  }

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

  .nav-item {
    display: block;
    background: transparent;
    padding: 16px 8px;
    opacity: 0.6;
    transition: all;
    transition-duration: 200ms;

    &:hover {
      opacity: 1;
    }
  }
  .active {
      opacity: 1;
    }
</style>
