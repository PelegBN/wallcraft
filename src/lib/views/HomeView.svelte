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

    <div class="text-center text-sm text-gray-500">
      {#if monitors.selectedMode === "spanning"}
        Target: {monitors.targetResolution.width} x {monitors.targetResolution.height} (spanning all monitors)
      {:else}
        Target: {monitors.targetResolution.width} x {monitors.targetResolution.height}
      {/if}
    </div>
  {/if}
</div>
