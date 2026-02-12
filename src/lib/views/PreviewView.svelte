<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { save } from "@tauri-apps/plugin-dialog";
  import ImagePreview from "$lib/components/ImagePreview.svelte";
  import TryCountdown from "$lib/components/TryCountdown.svelte";
  import { getGenerationStore } from "$lib/stores/generation.svelte";
  import { getMonitorStore } from "$lib/stores/monitors.svelte";
  import { getWallpaperStore } from "$lib/stores/wallpaper.svelte";
  import { getNavigation } from "$lib/utils/navigation.svelte";

  const generation = getGenerationStore();
  const monitors = getMonitorStore();
  const wallpaperStore = getWallpaperStore();
  const nav = getNavigation();

  let saving = $state(false);

  async function handleSave() {
    if (!generation.result) return;
    saving = true;
    try {
      const dest = await save({
        defaultPath: "wallcraft-wallpaper.png",
        filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg", "webp"] }],
      });
      if (dest) {
        await invoke("save_image_to_disk", {
          sourcePath: generation.result.image_path,
          destinationPath: dest,
        });
      }
    } catch (e) {
      console.error("Save failed:", e);
    } finally {
      saving = false;
    }
  }

  async function handleScrap() {
    if (!generation.result) return;
    try {
      await invoke("delete_temp_image", { imagePath: generation.result.image_path });
    } catch {
      // Best effort cleanup
    }
    generation.reset();
    nav.navigate("generate");
  }

  let tryError = $state<string | null>(null);

  async function handleTry() {
    if (!generation.result) return;
    tryError = null;
    const wpMode = monitors.selectedMode === "spanning" ? "span" : "crop";
    try {
      await wallpaperStore.tryWallpaper(generation.result.image_path, wpMode);
    } catch (e) {
      tryError = String(e);
      console.error("Try wallpaper failed:", e);
    }
  }

  async function handleApprove() {
    await wallpaperStore.approveTry();
  }

  async function handleCancel() {
    await wallpaperStore.cancelTry();
  }
</script>

<div class="flex flex-col gap-6 p-8 max-w-3xl mx-auto">
  <h2 class="text-2xl font-bold text-white text-center">Your Wallpaper</h2>

  {#if generation.result}
    <ImagePreview imagePath={generation.result.image_path} />

    <div class="text-center text-sm text-gray-400">
      {generation.result.final_width} x {generation.result.final_height}
      {#if generation.result.was_upscaled}
        <span class="text-blue-400">(upscaled)</span>
      {/if}
    </div>

    {#if tryError}
      <p class="text-red-400 text-sm text-center">{tryError}</p>
    {/if}

    <div class="flex gap-4 justify-center">
      <button
        class="px-6 py-2.5 bg-green-600 hover:bg-green-500 text-white rounded-xl font-medium transition-colors cursor-pointer disabled:opacity-50"
        onclick={handleSave}
        disabled={saving}
      >
        {saving ? "Saving..." : "Save"}
      </button>

      <button
        class="px-6 py-2.5 bg-yellow-600 hover:bg-yellow-500 text-white rounded-xl font-medium transition-colors cursor-pointer"
        onclick={handleTry}
      >
        Try
      </button>

      <button
        class="px-6 py-2.5 bg-red-600 hover:bg-red-500 text-white rounded-xl font-medium transition-colors cursor-pointer"
        onclick={handleScrap}
      >
        Scrap
      </button>
    </div>
  {:else}
    <p class="text-gray-400 text-center">No wallpaper generated yet.</p>
    <button
      class="mx-auto px-6 py-2.5 bg-blue-600 hover:bg-blue-500 text-white rounded-xl font-medium transition-colors cursor-pointer"
      onclick={() => nav.navigate("generate")}
    >
      Generate One
    </button>
  {/if}
</div>

{#if wallpaperStore.isTrying}
  <TryCountdown
    countdown={wallpaperStore.countdown}
    onApprove={handleApprove}
    onCancel={handleCancel}
  />
{/if}