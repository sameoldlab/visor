<script lang="ts">
  export let title = ""
  export let data: string
  export let grow = false
  export let border = true
</script>

<div class="{grow} {border ? 'border' : ''}">
  <span class="title">{title}</span>
  <span class="data">{data}</span>
</div>

<style lang="scss">
  div {
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: 4px;
    flex-grow: 0;
  }
  .border {
    margin-left: 1px;
    padding: 10px 20px;
    outline: var(--border); // No double borders
  }
  .grow {
    flex-grow: 3;
  }
</style>
