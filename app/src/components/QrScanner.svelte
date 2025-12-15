<script>
  import { onMount, onDestroy } from 'svelte';

  let { onScan, onCancel } = $props();

  let videoEl;
  let scanning = $state(true);
  let error = $state(null);

  // Mock scanner for this implementation since we don't have the camera plugin set up completely
  // In a real app, we'd use @tauri-apps/plugin-barcode-scanner
  onMount(() => {
    startScanning();
  });

  onDestroy(() => {
    stopScanning();
  });

  async function startScanning() {
    try {
        // Here we would initialize the native scanner
        // For now, we simulate a scan after 3 seconds for demonstration
        setTimeout(() => {
            if (scanning) {
                // Mock success result: A JSON payload with public key and secret
                const mockPayload = JSON.stringify({
                    peer_id: "12D3KooW...",
                    secret: "abc123secret",
                    endpoint: "ws://192.168.1.50:8080"
                });
                onScan(mockPayload);
            }
        }, 3000);
    } catch (e) {
        error = "Camera access denied";
    }
  }

  function stopScanning() {
    scanning = false;
  }
</script>

<div class="fixed inset-0 bg-black/90 z-50 flex flex-col items-center justify-center p-4">
  <div class="relative w-full max-w-sm aspect-square bg-black border-2 border-primary/50 rounded-3xl overflow-hidden shadow-[0_0_50px_rgba(251,191,36,0.2)]">
    <!-- Camera Viewfinder Mock -->
    <div class="absolute inset-0 flex items-center justify-center">
        {#if error}
            <div class="text-red-500 text-center">
                <p class="text-2xl mb-2">🚫</p>
                <p>{error}</p>
            </div>
        {:else}
            <div class="text-white/50 animate-pulse">
                Scanning for Hive QR...
            </div>
        {/if}
    </div>

    <!-- Scanner Overlay -->
    <div class="absolute inset-0 border-[40px] border-black/50 mask-image-scanner"></div>
    <div class="absolute top-1/2 left-4 right-4 h-0.5 bg-primary/80 shadow-[0_0_10px_#fbbf24] animate-[scan_2s_ease-in-out_infinite]"></div>

    <!-- Corner Markers -->
    <div class="absolute top-8 left-8 w-12 h-12 border-l-4 border-t-4 border-primary rounded-tl-xl"></div>
    <div class="absolute top-8 right-8 w-12 h-12 border-r-4 border-t-4 border-primary rounded-tr-xl"></div>
    <div class="absolute bottom-8 left-8 w-12 h-12 border-l-4 border-b-4 border-primary rounded-bl-xl"></div>
    <div class="absolute bottom-8 right-8 w-12 h-12 border-r-4 border-b-4 border-primary rounded-br-xl"></div>
  </div>

  <p class="text-gray-400 mt-8 text-center text-sm">
    Point your camera at a Hive Node QR code<br>to automatically link this device.
  </p>

  <button
    class="mt-8 px-8 py-3 bg-white/10 hover:bg-white/20 text-white rounded-full font-medium backdrop-blur transition-all"
    onclick={onCancel}
  >
    Cancel
  </button>
</div>

<style>
  @keyframes scan {
    0%, 100% { transform: translateY(-100px); opacity: 0; }
    10% { opacity: 1; }
    90% { opacity: 1; }
    50% { transform: translateY(100px); }
  }
</style>
