<script lang="ts">
  interface Props {
    checked: boolean;
    type: boolean;
  }

  let { checked = $bindable(), type }: Props = $props();
  checked = type
</script>

<input type="checkbox" id="switch" bind:checked />
<label for="switch">Toggle</label>

<style lang="scss">
  input[type="checkbox"] {
    height: 0;
    width: 0;
    visibility: hidden;
  }

  label {
    --offset: 0.2rem;
    --height: 1.5rem;
    cursor: pointer;
    text-indent: -9999px;
    height: var(--height);
    width: 2.7rem;
    background: #3b3b3b;
    display: block;
    border-radius: 50px;
    position: relative;

    &:after {
      --size: calc(var(--height) - var(--offset) * 2);
      content: "";
      position: absolute;
      top: var(--offset);
      left: var(--offset);
      background: #000;

      width: var(--size);
      height: var(--size);
      border-radius: 50px;
      transition: 0.1s;
    }

    input:checked + & {
      background: var(--blue);

      &:after {
        // background: #fff;

        left: calc(100% - var(--offset));
        transform: translateX(-100%);
      }
    }

    &:active:after {
      width: 1.55rem;
    }
  }
</style>
