<script>
  import { IconDiamond } from '@tabler/icons-svelte';
  export let title = "Pro Tip";
</script>

<div class="relative group p-1 rounded-xl bg-gradient-to-r from-primary/50 via-primaryDim/50 to-primary/50 animate-pulse-slow">
  <div class="absolute inset-0 bg-primary/20 blur-lg group-hover:blur-xl transition-all duration-500"></div>
  <div class="relative bg-surface/90 backdrop-blur-xl rounded-lg p-4 border border-primary/30 flex items-start gap-4 overflow-hidden">
    <!-- Rotating Gem -->
    <div class="relative shrink-0">
      <div class="absolute inset-0 bg-primary/40 blur-md rounded-full animate-pulse"></div>
      <div class="relative animate-[spin_4s_linear_infinite]">
        <IconDiamond size={24} class="text-primary" />
      </div>
    </div>

    <div class="flex-1 min-w-0">
      <h4 class="text-primary font-bold text-sm uppercase tracking-wider mb-1 font-mono flex items-center gap-2">
        {title}
        <span class="h-px flex-1 bg-gradient-to-r from-primary/50 to-transparent"></span>
      </h4>
      <div class="text-textBody text-sm font-sans leading-relaxed">
        <slot />
      </div>
    </div>
  </div>
</div>