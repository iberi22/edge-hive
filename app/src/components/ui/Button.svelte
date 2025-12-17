<script>
  export let variant = 'primary'; // primary, secondary, danger, ghost, outline
  export let size = 'md'; // sm, md, lg
  export let type = 'button';
  export let disabled = false;
  export let href = '';
  export let fullWidth = false;

  const baseClasses = 'inline-flex items-center justify-center rounded-lg font-medium transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-background disabled:opacity-50 disabled:cursor-not-allowed font-sans tracking-wide';

  const variants = {
    primary: 'bg-primary text-white hover:bg-primaryDim shadow-lg shadow-primary/20 border border-transparent',
    secondary: 'bg-secondary text-white hover:bg-secondary/90 shadow-lg shadow-secondary/20 border border-transparent',
    danger: 'bg-danger text-white hover:bg-danger/90 shadow-lg shadow-danger/20 border border-transparent',
    ghost: 'bg-transparent text-muted hover:text-textHeading hover:bg-surfaceHighlight',
    outline: 'bg-transparent border border-glass-border text-textHeading hover:border-primary hover:text-primary',
  };

  const sizes = {
    sm: 'px-3 py-1.5 text-xs',
    md: 'px-4 py-2 text-sm',
    lg: 'px-6 py-3 text-base',
  };

  $: classes = `
    ${baseClasses}
    ${variants[variant]}
    ${sizes[size]}
    ${fullWidth ? 'w-full' : ''}
    ${$$props.class || ''}
  `;
</script>

{#if href}
  <a {href} class={classes} {...$$restProps}>
    <slot />
  </a>
{:else}
  <button {type} {disabled} class={classes} on:click {...$$restProps}>
    <slot />
  </button>
{/if}