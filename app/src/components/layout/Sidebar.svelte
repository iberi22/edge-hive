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

<aside class="w-64 bg-gray-900/90 backdrop-blur-xl text-white flex flex-col border-r border-glass-border h-full shadow-2xl">
  <!-- Branding -->
  <div class="p-6 flex items-center gap-3 border-b border-glass-border">
    <div class="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center shadow-lg shadow-blue-500/20">
      <IconNetwork size={20} />
    </div>
    <span class="font-bold text-xl tracking-tight">Edge Hive</span>
  </div>

  <!-- Navigation -->
  <nav class="flex-1 overflow-y-auto py-4 px-3">
    <ul class="space-y-1">
      {#each menuItems as item}
        <li>
          <a
            href={item.href}
            class="flex items-center gap-3 px-3 py-2.5 rounded-lg transition-all duration-200
            {currentPath === item.href
              ? 'bg-blue-600/10 text-blue-400 border border-blue-500/20 shadow-sm'
              : 'text-gray-400 hover:bg-glass-100 hover:text-white hover:border hover:border-glass-border border border-transparent'}"
          >
            <svelte:component this={item.icon} size={20} stroke={1.5} />
            <span class="font-medium">{item.label}</span>
          </a>
        </li>
      {/each}
    </ul>
  </nav>

  <!-- User Profile -->
  <div class="p-4 border-t border-glass-border">
    <button class="flex items-center gap-3 w-full p-2 rounded-lg hover:bg-glass-100 transition-colors text-left border border-transparent hover:border-glass-border">
      <div class="w-8 h-8 bg-gray-800 rounded-full flex items-center justify-center text-gray-300 border border-glass-border">
        <IconUser size={16} />
      </div>
      <div class="flex-1 min-w-0">
        <p class="text-sm font-medium text-white truncate">Admin User</p>
        <p class="text-xs text-gray-500 truncate">admin@edgehive.local</p>
      </div>
    </button>
  </div>
</aside>
