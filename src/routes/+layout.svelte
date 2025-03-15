<script>
  import "$lib/styles/app.css"
  import "$lib/styles/normalize.css"

  import { live } from "../anvil"
  import MainNav from "$lib/MainNav.svelte"
  import ConfigData from "$lib/ConfigData.svelte"
</script>

<MainNav />
<main>
  {#if $live}
    <ConfigData />
  {/if}
  <slot />
</main>

<style>


  main {
    margin-left: 56px; 
    /* 56 = m4 + p16 + w36 */
  }
</style>
