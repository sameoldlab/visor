<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  let total = 0
    const add = async () => count(1);
    const subtract = async () => count(-1);

    const count = async (diff: number) => {
        total = await invoke("count", { total, diff })
    }
</script>

    <div class="row">
      <button on:click={add}>+</button>
      <p id="total"> {total} </p>
      <button on:click={subtract}>-</button>
    </div>
  

<style>
    #total {
        padding: 0 2em;
    }
    .logo.vite:hover {
      filter: drop-shadow(0 0 2em #747bff);
    }
  
    .logo.svelte:hover {
      filter: drop-shadow(0 0 2em #ff3e00);
    }
  </style>

