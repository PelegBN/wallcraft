<script lang="ts">
  import type { GenerationStatus } from "$lib/stores/generation.svelte";

  let { status }: { status: GenerationStatus } = $props();

  const messages: Record<GenerationStatus, string> = {
    idle: "",
    starting: "Preparing...",
    generating: "Generating wallpaper with AI...",
    upscaling: "Upscaling to target resolution...",
    complete: "Done!",
    error: "Something went wrong",
  };
</script>

<div class="flex flex-col items-center gap-6 py-12">
  {#if status !== "complete" && status !== "error" && status !== "idle"}
    <div class="relative w-16 h-16">
      <div
        class="absolute inset-0 rounded-full border-4 border-gray-700"
      ></div>
      <div
        class="absolute inset-0 rounded-full border-4 border-blue-400 border-t-transparent animate-spin"
      ></div>
    </div>
  {/if}

  <p class="text-lg text-gray-300">{messages[status]}</p>
</div>
