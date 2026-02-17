<script lang="ts">
  import { convertFileSrc, invoke } from "@tauri-apps/api/core";

  let { imagePath }: { imagePath: string } = $props();

  let src = $derived(convertFileSrc(imagePath));
  let fallbackSrc = $state<string | null>(null);
  let loadError = $state(false);

  async function handleImageError() {
    if (fallbackSrc) return;
    loadError = true;
    try {
      const base64: string = await invoke("read_image_base64", { path: imagePath });
      fallbackSrc = `data:image/png;base64,${base64}`;
      loadError = false;
    } catch (e) {
      console.error("Failed to load image via fallback:", e);
    }
  }
</script>

<div class="flex items-center justify-center bg-gray-900 rounded-xl border border-gray-700 overflow-hidden">
  {#if loadError && !fallbackSrc}
    <div class="p-8 text-center text-gray-400">
      <p class="text-sm">Failed to load image preview.</p>
      <p class="text-xs mt-1 text-gray-500">{imagePath}</p>
    </div>
  {:else}
    <img
      src={fallbackSrc ?? src}
      alt="Generated wallpaper"
      class="max-w-full max-h-[400px] object-contain"
      onerror={handleImageError}
    />
  {/if}
</div>
