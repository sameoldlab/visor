<script lang="ts">
  export let title = ""
  export let data
  export let grow = false
  export let border = true
  export let stack = false
</script>

<div class:grow class:border class:stack>
  <span class="title" >{title}</span>
  <span class="data">{data}</span>
</div>

<style lang="scss">
  div {
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: 4px;
  }
  .border {
    box-sizing: border-box;
    margin-left: 1px;
    padding: 12px 16px;
    outline: var(--border); // No double borders
  }
  .grow {
    flex-grow: 1;
  }
  .stack {
    flex-direction: row;
  }
</style>
