<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";

  let { imagePath }: { imagePath: string } = $props();

  let src = $derived(convertFileSrc(imagePath));
</script>

<div class="flex items-center justify-center bg-gray-900 rounded-xl border border-gray-700 overflow-hidden">
  <img
    {src}
    alt="Generated wallpaper"
    class="max-w-full max-h-[400px] object-contain"
  />
</div>
