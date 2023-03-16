<script>
  import "../app.scss"

  import { live } from "../anvil"
  import MainNav from "$lib/MainNav.svelte"
  import ConfigData from "$lib/ConfigData.svelte"
</script>

<MainNav />
<main>
  {#if $live}
    <div class="sticky">
      <ConfigData />
    </div>
  {/if}
  <slot />
</main>

<style lang="scss">
  .sticky {
    position: sticky;
    top: 0;
    width: 100%;
  }

  main {
    margin-left: 56px; // 56 = m4 + p16 + w36 
    margin-top: -8px;
  }
</style>
