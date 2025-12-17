<script>
  import { IconMenu2, IconBell, IconSearch } from '@tabler/icons-svelte';
  export let currentPath = '';

  // Simple breadcrumb logic
  $: breadcrumbs = currentPath === '/'
    ? ['Dashboard']
    : ['Dashboard', ...currentPath.split('/').filter(Boolean).map(s => s.charAt(0).toUpperCase() + s.slice(1))];
</script>

<header class="bg-surface/40 backdrop-blur-md border-b border-glass-border h-16 flex items-center justify-between px-6 sticky top-0 z-10">
  <!-- Left: Breadcrumbs -->
  <div class="flex items-center gap-4">
    <button class="text-muted hover:text-white lg:hidden">
      <IconMenu2 size={24} />
    </button>
    <nav class="flex items-center text-sm font-medium text-muted font-mono">
      {#each breadcrumbs as crumb, i}
        {#if i > 0}
          <span class="mx-2 text-surfaceHighlight">/</span>
        {/if}
        <span class={i === breadcrumbs.length - 1 ? 'text-textHeading' : ''}>
          {crumb}
        </span>
      {/each}
    </nav>
  </div>

  <!-- Right: Actions -->
  <div class="flex items-center gap-4">
    <!-- Search -->
    <div class="relative hidden md:block group">
      <IconSearch size={16} class="absolute left-3 top-1/2 -translate-y-1/2 text-muted group-focus-within:text-primary transition-colors" />
      <input
        type="text"
        placeholder="Search..."
        class="bg-surface/50 border border-glass-border text-sm rounded-lg pl-9 pr-4 py-1.5 text-white placeholder-muted focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary w-64 backdrop-blur-sm transition-all focus:bg-surface font-sans"
      />
    </div>

    <!-- Notifications -->
    <button class="relative p-2 text-muted hover:text-white hover:bg-glass-100 rounded-lg transition-colors border border-transparent hover:border-glass-border">
      <IconBell size={20} />
      <span class="absolute top-2 right-2 w-2 h-2 bg-danger rounded-full border-2 border-surface"></span>
    </button>

    <!-- Status Indicator -->
    <div class="flex items-center gap-2 px-3 py-1.5 bg-success/10 border border-success/20 rounded-full backdrop-blur-sm">
      <div class="w-2 h-2 bg-success rounded-full animate-pulse shadow-[0_0_8px_rgba(16,185,129,0.5)]"></div>
      <span class="text-xs font-medium text-success font-mono tracking-wide">SYSTEM.READY</span>
    </div>
  </div>
</header>
