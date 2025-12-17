<script>
  export let variant = 'default'; // default, success, warning, danger, info, primary
  export let size = 'md'; // sm, md

  const variants = {
    default: 'bg-surfaceHighlight text-textBody border-glass-border',
    success: 'bg-success/10 text-success border-success/20 shadow-[0_0_5px_rgba(16,185,129,0.2)]',
    warning: 'bg-primary/10 text-primary border-primary/20 shadow-[0_0_5px_rgba(249,115,22,0.2)]',
    danger: 'bg-danger/10 text-danger border-danger/20 shadow-[0_0_5px_rgba(239,68,68,0.2)]',
    info: 'bg-secondary/10 text-secondary border-secondary/20 shadow-[0_0_5px_rgba(14,165,233,0.2)]',
    primary: 'bg-primary/10 text-primary border-primary/20',
  };

  const sizes = {
    sm: 'px-2 py-0.5 text-[10px]',
    md: 'px-2.5 py-1 text-xs',
  };

  $: classes = `
    inline-flex items-center gap-1.5 rounded-full font-medium border font-mono tracking-wide
    ${variants[variant]}
    ${sizes[size]}
    ${$$props.class || ''}
  `;
</script>

<span class={classes}>
  <slot />
</span>