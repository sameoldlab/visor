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
  body {
    margin: 0;
    overflow-x: hidden;
  }

  .sticky {
    position: sticky;
    top: 0;
    width: 100%;
  }

  main {
    margin-left: 48px; //48 = (56) - 8 = (m4 + p16 + w36 ) - 8
    margin-top: -8px;
    margin-right: -8px;
  }
</style>
