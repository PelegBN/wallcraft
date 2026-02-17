<script lang="ts">
  import { onMount } from "svelte";
  import MonitorLayout from "$lib/components/MonitorLayout.svelte";
  import { getMonitorStore } from "$lib/stores/monitors.svelte";
  import { getNavigation } from "$lib/utils/navigation.svelte";

  const monitors = getMonitorStore();
  const nav = getNavigation();

  onMount(() => {
    monitors.detectMonitors();
  });

  function selectMode(mode: "individual" | "spanning") {
    monitors.selectedMode = mode;
    nav.navigate("generate");
  }
</script>

<div class="flex flex-col items-center gap-8 p-8">
  <div class="text-center">
    <h1 class="text-3xl font-bold text-white mb-2">WallCraft</h1>
    <p class="text-gray-400">AI-powered wallpaper generator for your desktop</p>
  </div>

  {#if monitors.loading}
    <p class="text-gray-400">Detecting monitors...</p>
  {:else if monitors.error}
    <div class="text-red-400 text-center">
      <p>Failed to detect monitors: {monitors.error}</p>
      <button
        class="mt-4 px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded-lg cursor-pointer"
        onclick={() => monitors.detectMonitors()}
      >
        Retry
      </button>
    </div>
  {:else if monitors.layout}
    <div class="w-full max-w-xl">
      <h2 class="text-lg font-medium text-gray-300 mb-4 text-center">
        {monitors.layout.monitors.length === 1
          ? "1 Monitor Detected"
          : `${monitors.layout.monitors.length} Monitors Detected`}
      </h2>

      <MonitorLayout
        layout={monitors.layout}
        selectedIndex={monitors.selectedMonitorIndex}
        mode={monitors.selectedMode}
        onSelectMonitor={(i) => (monitors.selectedMonitorIndex = i)}
      />
    </div>

    <div class="flex flex-col sm:flex-row gap-4 mt-4">
      <button
        class="px-6 py-3 bg-blue-600 hover:bg-blue-500 text-white rounded-xl font-medium transition-colors cursor-pointer"
        onclick={() => selectMode("individual")}
      >
        {monitors.layout.monitors.length > 1
          ? "Individual Wallpaper"
          : "Generate Wallpaper"}
      </button>

      {#if monitors.layout.monitors.length > 1}
        <button
          class="px-6 py-3 bg-purple-600 hover:bg-purple-500 text-white rounded-xl font-medium transition-colors cursor-pointer"
          onclick={() => selectMode("spanning")}
        >
          Spanning Wallpaper
        </button>
      {/if}
    </div>

    <!-- Resolution picker -->
    <div class="flex flex-col items-center gap-3 w-full max-w-md">
      <div class="flex gap-2">
        <button
          class="px-4 py-1.5 rounded-lg text-sm font-medium transition-colors cursor-pointer
            {!monitors.useCustomResolution ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}"
          onclick={() => (monitors.useCustomResolution = false)}
        >
          Screen Resolution
        </button>
        <button
          class="px-4 py-1.5 rounded-lg text-sm font-medium transition-colors cursor-pointer
            {monitors.useCustomResolution ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}"
          onclick={() => (monitors.useCustomResolution = true)}
        >
          Custom Resolution
        </button>
      </div>

      {#if monitors.useCustomResolution}
        <div class="flex items-center gap-2">
          <input
            type="number"
            min="100"
            max="7680"
            bind:value={monitors.customWidth}
            class="w-24 px-3 py-1.5 bg-gray-800 border border-gray-600 rounded-lg text-white text-sm text-center
                   focus:outline-none focus:border-blue-500"
          />
          <span class="text-gray-400 text-sm">x</span>
          <input
            type="number"
            min="100"
            max="4320"
            bind:value={monitors.customHeight}
            class="w-24 px-3 py-1.5 bg-gray-800 border border-gray-600 rounded-lg text-white text-sm text-center
                   focus:outline-none focus:border-blue-500"
          />
          <span class="text-gray-500 text-xs">px</span>
        </div>
      {/if}

      <p class="text-sm text-gray-500">
        Target: {monitors.targetResolution.width} x {monitors.targetResolution.height}
        {#if !monitors.useCustomResolution && monitors.selectedMode === "spanning"}
          (spanning all monitors)
        {:else if monitors.useCustomResolution}
          (custom)
        {/if}
      </p>
    </div>
  {/if}
</div>
