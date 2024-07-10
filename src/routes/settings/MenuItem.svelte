<script lang="ts">
  import {
    Listbox,
    ListboxButton,
    ListboxOptions,
    ListboxOption,
    Switch,
  } from "@rgossiaux/svelte-headlessui"
  //   import Switch from "$lib/Switch.svelte"

  type List = {
    id: number
    name: string
    unavailable: boolean
  }[]

  export let title: string
  export let description: string
  export let type: string | number | boolean | List
  export let val: typeof type | undefined = undefined

  let selected = Array.isArray(type) && type[0]
  $: val = selected ? selected.name : val
  const isNumber = (e: KeyboardEvent) => {
    console.log(/[0-9]/i.test(e.key))
  }
</script>

<div class="setting">
  <div>
    <h4>{title}</h4>
    <p>{description}</p>
  </div>
  {#if typeof type === "number"}
    <input
      type="number"
      bind:value={val}
      placeholder={type}
      on:keydown={isNumber}
      min="0"
      step="10"
    />
  {:else if typeof type === "string"}
    <input type="text" bind:value={val} placeholder={type} />
  {:else if typeof type === "boolean"}
    <Switch
      checked={val || false}
      on:change={(e) => (val = e.detail)}
      class={val ? "switch switch-enabled" : "switch switch-disabled"}
    >
      <span class="sr-only">{description}</span>
      <span class="toggle" class:toggle-on={val} /></Switch
    >
  {:else}
    <Listbox value={selected} on:change={(e) => (selected = e.detail)}>
      <ListboxButton
        ><div class="list-btn">
          {selected.name}
          <!-- https://icons.radix-ui.com/ -->
          <svg
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            ><path
              d="M4.18179 6.18181C4.35753 6.00608 4.64245 6.00608 4.81819 6.18181L7.49999 8.86362L10.1818 6.18181C10.3575 6.00608 10.6424 6.00608 10.8182 6.18181C10.9939 6.35755 10.9939 6.64247 10.8182 6.81821L7.81819 9.81821C7.73379 9.9026 7.61934 9.95001 7.49999 9.95001C7.38064 9.95001 7.26618 9.9026 7.18179 9.81821L4.18179 6.81821C4.00605 6.64247 4.00605 6.35755 4.18179 6.18181Z"
              fill="currentColor"
              fill-rule="evenodd"
              clip-rule="evenodd"
            /></svg
          >
        </div></ListboxButton
      >
      <ListboxOptions class={({ open }) => (open ? "list-opts" : "")}>
        {#each [...type] as item (item.id)}
          <ListboxOption
            value={item}
            disabled={item.unavailable}
            class={({ active }) => (active ? "active" : "")}
            let:selected
          >
            {item.name}&nbsp;{#if selected}
              <!-- https://icons.radix-ui.com/ -->

              <svg
                width="15"
                height="15"
                viewBox="0 0 15 15"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                ><path
                  d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z"
                  fill="currentColor"
                  fill-rule="evenodd"
                  clip-rule="evenodd"
                /></svg
              >
            {/if}
          </ListboxOption>
        {/each}
      </ListboxOptions>
    </Listbox>
  {/if}

  <slot />
</div>

<style lang="scss">
  .setting {
    display: flex;
    flex-wrap: wrap;
    max-width: min(80ch, 100%);
    // margin: auto;
    gap: 0.25rem;
    align-items: center;
    justify-content: space-between;

    h4 {
      font-size: .9rem;
      font-weight: 600;
      margin-bottom: 0;
    }

    p {
      width: min(60ch, 100%);
      font-size: 0.8rem;
      margin-top: 0.5em;
      font-weight: 600;
      color: var(--text-secondary);
    }
  }

  input {
    transition: border 0.125s ease-in-out;
    font-family: var(--code);
    background: transparent;
    border: var(--border);
    padding: 0.5rem 0.5rem;
    border-radius: 0.25rem;
    height: 1.75rem;
    color-scheme: dark; //for number input arrows
    line-height: 17px;

    &[type="text"] {
      width: min(50ch, 100%);
    }

    &:focus {
      border-color: var(--blue);
      outline: 0;
      // border-radius: 0.5rem;
    }
  }
</style>
