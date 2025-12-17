<script>
  import { onMount } from 'svelte';
  import Card from '../ui/Card.svelte';
  import Badge from '../ui/Badge.svelte';

  export let title = '';
  export let value = '';
  export let unit = '';
  export let icon = null;
  export let trend = 0; // Percentage change
  export let color = 'primary'; // primary, secondary, success, danger

  // Map colors to Tailwind classes
  const colorMap = {
    primary: 'text-primary bg-primary/10 border-primary/20',
    primaryDim: 'text-primaryDim bg-primaryDim/10 border-primaryDim/20',
    secondary: 'text-secondary bg-secondary/10 border-secondary/20',
    success: 'text-success bg-success/10 border-success/20',
    danger: 'text-danger bg-danger/10 border-danger/20',
    blue: 'text-secondary bg-secondary/10 border-secondary/20', // Fallback
    green: 'text-success bg-success/10 border-success/20', // Fallback
    red: 'text-danger bg-danger/10 border-danger/20', // Fallback
    yellow: 'text-primary bg-primary/10 border-primary/20', // Fallback
    purple: 'text-accent bg-accent/10 border-accent/20', // Fallback
  };

  $: iconClass = colorMap[color] || colorMap.primary;
</script>

<Card hover={true} hud={true} class="h-full">
  <div class="flex justify-between items-start mb-4 relative z-10">
    <div>
      <p class="text-muted text-sm font-medium mb-1 group-hover:text-textBody transition-colors font-sans">{title}</p>
      <h3 class="text-2xl font-bold text-textHeading tracking-tight font-mono">
        {value} <span class="text-sm text-muted font-normal ml-1 font-sans">{unit}</span>
      </h3>
    </div>
    {#if icon}
      <div class={`p-2 rounded-lg border ${iconClass} backdrop-blur-sm shadow-inner`}>
        <svelte:component this={icon} size={22} stroke={1.5} />
      </div>
    {/if}
  </div>

  <!-- Trend / Sparkline Placeholder -->
  <div class="flex items-center gap-2 relative z-10">
    {#if trend !== 0}
      <Badge variant={trend > 0 ? 'success' : 'danger'} class="font-mono">
        {trend > 0 ? '+' : ''}{trend}%
      </Badge>
      <span class="text-xs text-muted font-sans">vs last hour</span>
    {:else}
      <span class="text-xs text-muted font-sans">Stable</span>
    {/if}
  </div>
</Card>
