<script lang="ts">
  import "$lib/styles/app.css"
  import "$lib/styles/normalize.css"

  import { live } from "../anvil"
  import MainNav from "$lib/MainNav.svelte"
  import ConfigData from "$lib/ConfigData.svelte"
  interface Props {
    children?: import('svelte').Snippet;
  }

  let { children }: Props = $props();
</script>

<MainNav />
<main>
  {#if $live}
    <ConfigData />
  {/if}
  {@render children?.()}
</main>

<style>


  main {
    margin-left: 56px; 
    /* 56 = m4 + p16 + w36 */
  }
</style>
