<script lang="ts">
  import NavItem from "$lib/Icons.svelte"
  import { live, killTestnet, startTestnet } from "/src/anvil"
</script>

<nav>
  <div class="nav--main">
    <a class="nav-item" href="./accounts">
      <NavItem name="accounts" />
    </a>
    <a class="nav-item" href="./log">
      <NavItem name="log" />
    </a>
    <a class="nav-item" href="./blocks">
      <NavItem name="blocks" />
    </a>
  </div>
  <div class="nav--end">
    <a class="nav-item" href="./settings">
      <NavItem name="settings" />
    </a>
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
      opacity: 0.7;
      padding: 16px 8px;

      &:hover {
        opacity: 1;
      }
    }
  }

  .sticky {
    position: sticky;
    top: 0;
    width: 100%;
  }
</style>
