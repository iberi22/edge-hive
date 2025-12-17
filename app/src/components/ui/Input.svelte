<script>
  export let label = '';
  export let type = 'text';
  export let value = '';
  export let placeholder = '';
  export let error = '';
  export let disabled = false;
  export let id = Math.random().toString(36).substring(2);

  $: inputClasses = `
    w-full bg-surface/50 border rounded-lg px-4 py-2 text-textHeading placeholder-muted
    focus:outline-none focus:ring-1 transition-all font-sans
    ${error
      ? 'border-danger focus:border-danger focus:ring-danger'
      : 'border-glass-border focus:border-primary focus:ring-primary'}
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
    <input
      {id}
      {type}
      {value}
      {placeholder}
      {disabled}
      class={inputClasses}
      on:input
      on:change
      on:focus
      on:blur
      {...$$restProps}
    />
    {#if $$slots.icon}
      <div class="absolute right-3 top-1/2 -translate-y-1/2 text-muted pointer-events-none">
        <slot name="icon" />
      </div>
    {/if}
  </div>

  {#if error}
    <p class="mt-1 text-xs text-danger font-sans">{error}</p>
  {/if}
</div>