<script lang="ts">
  import type { MonitorLayout as MonitorLayoutType } from "$lib/stores/monitors.svelte";

  let {
    layout,
    selectedIndex = 0,
    mode = "individual",
    onSelectMonitor,
  }: {
    layout: MonitorLayoutType;
    selectedIndex?: number;
    mode?: "individual" | "spanning";
    onSelectMonitor?: (index: number) => void;
  } = $props();

  const containerWidth = 500;
  const containerHeight = 250;

  let positions = $derived.by(() => {
    if (!layout.monitors.length) return [];

    const minX = Math.min(...layout.monitors.map((m) => m.x));
    const minY = Math.min(...layout.monitors.map((m) => m.y));
    const maxX = Math.max(...layout.monitors.map((m) => m.x + m.width));
    const maxY = Math.max(...layout.monitors.map((m) => m.y + m.height));

    const totalW = maxX - minX;
    const totalH = maxY - minY;

    const scale = Math.min(
      (containerWidth - 40) / totalW,
      (containerHeight - 40) / totalH
    );

    return layout.monitors.map((m, i) => ({
      left: (m.x - minX) * scale + 20,
      top: (m.y - minY) * scale + 20,
      width: m.width * scale,
      height: m.height * scale,
      monitor: m,
      index: i,
    }));
  });
</script>

<div
  class="relative bg-gray-800 rounded-xl border border-gray-600 mx-auto"
  style="width: {containerWidth}px; height: {containerHeight}px;"
>
  {#each positions as pos}
    <button
      class="absolute rounded-lg border-2 flex flex-col items-center justify-center text-xs transition-all cursor-pointer
        {mode === 'spanning'
          ? 'border-blue-400 bg-blue-500/20'
          : pos.index === selectedIndex
            ? 'border-blue-400 bg-blue-500/20'
            : 'border-gray-500 bg-gray-700/50 hover:border-gray-400'}"
      style="left: {pos.left}px; top: {pos.top}px; width: {pos.width}px; height: {pos.height}px;"
      onclick={() => onSelectMonitor?.(pos.index)}
    >
      <span class="font-semibold text-white truncate px-1">
        {pos.monitor.name || `Monitor ${pos.index + 1}`}
      </span>
      <span class="text-gray-300">
        {pos.monitor.width}x{pos.monitor.height}
      </span>
      {#if pos.monitor.is_primary}
        <span class="text-blue-300 text-[10px]">Primary</span>
      {/if}
    </button>
  {/each}
</div>
