<script>
  import {
    IconDashboard,
    IconServer,
    IconNetwork,
    IconDatabase,
    IconSettings,
    IconBrandTorchain,
    IconUser,
    IconTerminal
  } from '@tabler/icons-svelte';

  export let currentPath = '';

  const menuItems = [
    { label: 'Dashboard', href: '/', icon: IconDashboard },
    { label: 'Nodes', href: '/nodes', icon: IconServer },
    { label: 'Terminal', href: '/terminal', icon: IconTerminal },
    { label: 'Tor Network', href: '/tor', icon: IconBrandTorchain },
    { label: 'Database', href: '/database', icon: IconDatabase },
    { label: 'Settings', href: '/settings', icon: IconSettings },
  ];
</script>

<aside class="w-64 bg-surface/40 backdrop-blur-xl border-r border-glass-border h-full flex flex-col z-20 relative">
  <!-- Branding -->
  <div class="p-6 flex items-center gap-3 border-b border-glass-border bg-surface/20">
    <div class="w-8 h-8 bg-gradient-to-br from-primary to-primaryDim rounded-lg flex items-center justify-center shadow-lg shadow-primary/20 relative overflow-hidden group">
      <div class="absolute inset-0 bg-white/20 translate-y-full group-hover:translate-y-0 transition-transform duration-300"></div>
      <IconNetwork size={20} class="text-white relative z-10" />
    </div>
    <span class="font-bold text-xl tracking-tight text-textHeading font-sans">Edge Hive</span>
  </div>

  <!-- Navigation -->
  <nav class="flex-1 overflow-y-auto py-4 px-3 space-y-1">
    {#each menuItems as item}
      {@const isActive = currentPath === item.href}
      <a
        href={item.href}
        class="flex items-center gap-3 px-3 py-2.5 rounded-lg transition-all duration-200 group relative overflow-hidden
        {isActive
          ? 'bg-primary/10 text-primary border border-primary/20 shadow-[0_0_15px_rgba(249,115,22,0.1)]'
          : 'text-muted hover:text-white hover:bg-glass-100 border border-transparent'}"
      >
        {#if isActive}
          <div class="absolute left-0 top-0 bottom-0 w-1 bg-primary shadow-[0_0_10px_#f97316]"></div>
        {/if}
        <svelte:component this={item.icon} size={20} stroke={1.5} class={isActive ? 'text-primary' : 'group-hover:text-white transition-colors'} />
        <span class="font-medium font-sans">{item.label}</span>
      </a>
    {/each}
  </nav>

  <!-- User Profile -->
  <div class="p-4 border-t border-glass-border bg-surface/20">
    <button class="flex items-center gap-3 w-full p-2 rounded-lg hover:bg-glass-100 transition-colors text-left border border-transparent hover:border-glass-border group">
      <div class="w-8 h-8 bg-surfaceHighlight rounded-full flex items-center justify-center text-muted group-hover:text-white border border-glass-border transition-colors">
        <IconUser size={16} />
      </div>
      <div class="flex-1 min-w-0">
        <p class="text-sm font-medium text-textHeading truncate font-sans">Admin User</p>
        <p class="text-xs text-muted truncate font-mono">admin@edgehive.local</p>
      </div>
    </button>
  </div>
</aside>
