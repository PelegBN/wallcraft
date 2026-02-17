<script lang="ts">
  import { onMount } from "svelte";
  import CategorySelector from "$lib/components/CategorySelector.svelte";
  import PromptInput from "$lib/components/PromptInput.svelte";
  import GenerationProgress from "$lib/components/GenerationProgress.svelte";
  import { getGenerationStore } from "$lib/stores/generation.svelte";
  import { getMonitorStore } from "$lib/stores/monitors.svelte";
  import { getSettingsStore } from "$lib/stores/settings.svelte";
  import { getNavigation } from "$lib/utils/navigation.svelte";
  import { buildPrompt } from "$lib/utils/prompt-builder";

  const generation = getGenerationStore();
  const monitors = getMonitorStore();
  const settingsStore = getSettingsStore();
  const nav = getNavigation();

  onMount(() => {
    if (!settingsStore.loaded) settingsStore.load();
  });

  let mode = $state<"categories" | "prompt">("categories");
  let selectedStyles = $state<Set<string>>(new Set());
  let selectedSchemes = $state<Set<string>>(new Set());
  let customPrompt = $state("");
  let directPrompt = $state("");

  let isGenerating = $derived(
    generation.status === "starting" ||
    generation.status === "generating" ||
    generation.status === "upscaling"
  );

  $effect(() => {
    if (generation.status === "complete" && generation.result) {
      nav.navigate("preview");
    }
  });

  async function handleGenerate() {
    const target = monitors.targetResolution;
    const provider = settingsStore.settings.ai_provider === "openai" ? "OpenAi" as const : "Pollinations" as const;

    if (mode === "categories") {
      const { styles, color_schemes, custom_prompt } = buildPrompt(
        Array.from(selectedStyles),
        Array.from(selectedSchemes),
        customPrompt || null
      );
      // Vector art: skip upscaling — clean edges scale well without it
      await generation.generate({
        styles,
        color_schemes,
        custom_prompt,
        width: target.width,
        height: target.height,
        provider,
        target_width: target.width,
        target_height: target.height,
      }, false, 4);
    } else {
      await generation.generate({
        styles: [],
        color_schemes: [],
        custom_prompt: directPrompt || null,
        width: target.width,
        height: target.height,
        provider,
        target_width: target.width,
        target_height: target.height,
      }, settingsStore.settings.upscale_enabled, settingsStore.settings.upscale_factor);
    }
  }
</script>

<div class="flex flex-col gap-6 p-8 max-w-3xl mx-auto">
  <div class="text-center">
    <h2 class="text-2xl font-bold text-white mb-1">Create Your Wallpaper</h2>
    <p class="text-sm text-gray-400">
      Target: {monitors.targetResolution.width} x {monitors.targetResolution.height}
    </p>
  </div>

  {#if isGenerating}
    <GenerationProgress status={generation.status} />
  {:else}
    <!-- Mode toggle -->
    <div class="flex gap-2 justify-center">
      <button
        class="px-4 py-2 rounded-lg text-sm font-medium transition-colors cursor-pointer
          {mode === 'categories' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}"
        onclick={() => (mode = "categories")}
      >
        Vector Art
      </button>
      <button
        class="px-4 py-2 rounded-lg text-sm font-medium transition-colors cursor-pointer
          {mode === 'prompt' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}"
        onclick={() => (mode = "prompt")}
      >
        Direct Prompt
      </button>
    </div>

    {#if mode === "categories"}
      <div class="flex flex-col gap-4">
        <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide">
          Select Styles
        </h3>
        <CategorySelector bind:selectedStyles={selectedStyles} bind:selectedSchemes={selectedSchemes} />

        <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide mt-2">
          Additional Details (optional)
        </h3>
        <PromptInput
          bind:value={customPrompt}
          placeholder="Add extra details... e.g. 'blue and orange palette, dark background'"
        />
      </div>
    {:else}
      <div class="flex flex-col gap-4">
        <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide">
          Describe Your Wallpaper
        </h3>
        <PromptInput
          bind:value={directPrompt}
          placeholder="Describe your perfect wallpaper in detail..."
        />
      </div>
    {/if}

    {#if generation.status === "error"}
      <p class="text-red-400 text-sm text-center">{generation.error}</p>
    {/if}

    <button
      class="w-full py-3 bg-blue-600 hover:bg-blue-500 disabled:bg-gray-600 disabled:cursor-not-allowed
             text-white rounded-xl font-medium transition-colors cursor-pointer text-lg"
      onclick={handleGenerate}
      disabled={mode === "categories" && selectedStyles.size === 0 && !customPrompt.trim()}
    >
      Generate Wallpaper
    </button>
  {/if}
</div>
