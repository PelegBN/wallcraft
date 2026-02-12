<script lang="ts">
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { getSettingsStore } from "$lib/stores/settings.svelte";

  const store = getSettingsStore();
  let saveStatus = $state<"idle" | "saving" | "saved">("idle");

  onMount(() => {
    store.load();
  });

  async function handleSave() {
    saveStatus = "saving";
    try {
      await store.save();
      saveStatus = "saved";
      setTimeout(() => (saveStatus = "idle"), 2000);
    } catch {
      saveStatus = "idle";
    }
  }

  async function pickDirectory() {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      store.update({ save_directory: selected as string });
    }
  }
</script>

<div class="flex flex-col gap-8 p-8 max-w-xl mx-auto">
  <h2 class="text-2xl font-bold text-white">Settings</h2>

  <!-- AI Provider -->
  <div class="flex flex-col gap-3">
    <span class="text-sm font-medium text-gray-400 uppercase tracking-wide">
      AI Provider
    </span>
    <div class="flex gap-3">
      <button
        class="flex-1 px-4 py-2.5 rounded-xl text-sm font-medium transition-colors cursor-pointer
          {store.settings.ai_provider === 'pollinations'
            ? 'bg-blue-600 text-white'
            : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}"
        onclick={() => store.update({ ai_provider: "pollinations" })}
      >
        Pollinations (Free)
      </button>
      <button
        class="flex-1 px-4 py-2.5 rounded-xl text-sm font-medium transition-colors cursor-pointer
          {store.settings.ai_provider === 'openai'
            ? 'bg-blue-600 text-white'
            : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}"
        onclick={() => store.update({ ai_provider: "openai" })}
      >
        OpenAI (Paid)
      </button>
    </div>
  </div>

  <!-- OpenAI API Key -->
  {#if store.settings.ai_provider === "openai"}
    <div class="flex flex-col gap-2">
      <label class="text-sm font-medium text-gray-400 uppercase tracking-wide" for="api-key">
        OpenAI API Key
      </label>
      <input
        id="api-key"
        type="password"
        class="bg-gray-800 border border-gray-600 rounded-xl px-4 py-2.5 text-gray-100
               placeholder-gray-500 focus:border-blue-400 focus:outline-none focus:ring-1
               focus:ring-blue-400 transition-colors"
        placeholder="sk-..."
        value={store.settings.openai_api_key || ""}
        oninput={(e) => store.update({ openai_api_key: (e.target as HTMLInputElement).value || null })}
      />
    </div>
  {/if}

  <!-- Default Save Directory -->
  <div class="flex flex-col gap-2">
    <span class="text-sm font-medium text-gray-400 uppercase tracking-wide">
      Default Save Directory
    </span>
    <div class="flex gap-2">
      <input
        type="text"
        class="flex-1 bg-gray-800 border border-gray-600 rounded-xl px-4 py-2.5 text-gray-100
               placeholder-gray-500 focus:border-blue-400 focus:outline-none transition-colors"
        placeholder="Not set (will prompt each time)"
        value={store.settings.save_directory || ""}
        readonly
      />
      <button
        class="px-4 py-2.5 bg-gray-700 hover:bg-gray-600 text-gray-200 rounded-xl transition-colors cursor-pointer"
        onclick={pickDirectory}
      >
        Browse
      </button>
    </div>
  </div>

  <!-- Upscaling -->
  <div class="flex flex-col gap-3">
    <span class="text-sm font-medium text-gray-400 uppercase tracking-wide">
      AI Upscaling (Real-ESRGAN)
    </span>
    <div class="flex items-center gap-3">
      <button
        class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors cursor-pointer
          {store.settings.upscale_enabled ? 'bg-blue-600' : 'bg-gray-600'}"
        aria-label="Toggle upscaling"
        onclick={() => store.update({ upscale_enabled: !store.settings.upscale_enabled })}
      >
        <span
          class="inline-block h-4 w-4 rounded-full bg-white transition-transform
            {store.settings.upscale_enabled ? 'translate-x-6' : 'translate-x-1'}"
        ></span>
      </button>
      <span class="text-sm text-gray-300">
        {store.settings.upscale_enabled ? "Enabled" : "Disabled"}
      </span>
    </div>

    {#if store.settings.upscale_enabled}
      <div class="flex gap-2">
        <button
          class="px-4 py-2 rounded-lg text-sm font-medium transition-colors cursor-pointer
            {store.settings.upscale_factor === 2 ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}"
          onclick={() => store.update({ upscale_factor: 2 })}
        >
          2x
        </button>
        <button
          class="px-4 py-2 rounded-lg text-sm font-medium transition-colors cursor-pointer
            {store.settings.upscale_factor === 4 ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}"
          onclick={() => store.update({ upscale_factor: 4 })}
        >
          4x
        </button>
      </div>
    {/if}
  </div>

  <!-- Save Button -->
  <button
    class="w-full py-3 bg-green-600 hover:bg-green-500 text-white rounded-xl font-medium transition-colors cursor-pointer"
    onclick={handleSave}
  >
    {#if saveStatus === "saving"}
      Saving...
    {:else if saveStatus === "saved"}
      Saved!
    {:else}
      Save Settings
    {/if}
  </button>
</div>