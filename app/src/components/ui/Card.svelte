<script>
  export let title = '';
  export let hud = false; // Enable HUD corners
  export let hover = false; // Enable hover effects
  export let noPadding = false; // Disable default padding
</script>

<div class={`
  bg-surface/40 backdrop-blur-md border border-glass-border rounded-xl relative overflow-hidden
  ${noPadding ? '' : 'p-6'}
  ${hover ? 'transition-all duration-300 hover:border-primary/30 hover:shadow-lg hover:shadow-primary/5 group' : ''}
  ${$$props.class || ''}
`}>
  {#if hud}
    <div class="absolute top-0 right-0 w-4 h-4 border-t-2 border-r-2 border-primary/50 rounded-tr-sm pointer-events-none"></div>
    <div class="absolute bottom-0 left-0 w-4 h-4 border-b-2 border-l-2 border-primary/50 rounded-bl-sm pointer-events-none"></div>
  {/if}

  {#if title}
    <div class={`border-b border-glass-border ${noPadding ? 'p-5' : 'mb-4 pb-2'}`}>
      <h3 class="text-lg font-bold text-textHeading font-sans flex items-center justify-between">
        {title}
        <slot name="actions" />
      </h3>
    </div>
  {/if}

  <slot />
</div>