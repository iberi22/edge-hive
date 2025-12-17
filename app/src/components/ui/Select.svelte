<script>
  export let label = '';
  export let value = '';
  export let options = []; // Array of strings or objects { value, label }
  export let disabled = false;
  export let id = Math.random().toString(36).substring(2);

  $: selectClasses = `
    w-full bg-surface/50 border border-glass-border rounded-lg px-4 py-2 text-textHeading
    focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary transition-all font-sans appearance-none
    ${disabled ? 'opacity-50 cursor-not-allowed' : ''}
    ${$$props.class || ''}
  `;
</script>

<div class="w-full">
  {#if label}
    <label for={id} class="block text-sm font-medium text-muted mb-1.5 font-sans">
      {label}
    </label>
  {/if}
  
  <div class="relative">
    <select
      {id}
      bind:value
      {disabled}
      class={selectClasses}
      on:change
      {...$$restProps}
    >
      {#each options as option}
        {#if typeof option === 'object'}
          <option value={option.value}>{option.label}</option>
        {:else}
          <option value={option}>{option}</option>
        {/if}
      {/each}
    </select>
    <div class="absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none text-muted">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </div>
  </div>
</div>