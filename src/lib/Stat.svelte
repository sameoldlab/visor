<script lang="ts">
  interface Props {
    title?: string;
    data: any;
    grow?: boolean;
    border?: boolean;
    unstack?: boolean;
  }

  let {
    title = "",
    data,
    grow = false,
    border = true,
    unstack = false
  }: Props = $props();
</script>

<div class:grow class:border class:unstack>
  <span class="title">{title}</span>
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
  .title, .data {
    display: block;
  }
  .unstack {
    flex-direction: row;
    align-items: baseline;
  }
</style>
