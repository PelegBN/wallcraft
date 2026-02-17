<script lang="ts">
  import { STYLES, COLOR_SCHEMES } from "$lib/utils/categories";

  let {
    selectedStyles = $bindable<Set<string>>(new Set()),
    selectedSchemes = $bindable<Set<string>>(new Set()),
  }: {
    selectedStyles?: Set<string>;
    selectedSchemes?: Set<string>;
  } = $props();

  function toggleStyle(id: string) {
    const next = new Set(selectedStyles);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    selectedStyles = next;
  }

  function toggleScheme(id: string) {
    const next = new Set(selectedSchemes);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    selectedSchemes = next;
  }
</script>

<div class="flex flex-col gap-5">
  <!-- Shapes & Patterns -->
  <div>
    <h4 class="text-xs font-medium text-gray-500 uppercase tracking-wider mb-2">
      Shapes & Patterns
    </h4>
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3">
      {#each STYLES as style}
        <button
          class="flex flex-col items-center gap-1 p-3 rounded-xl border-2 transition-all cursor-pointer
            {selectedStyles.has(style.id)
              ? 'border-blue-400 bg-blue-500/20 shadow-lg shadow-blue-500/10'
              : 'border-gray-600 bg-gray-800 hover:border-gray-400'}"
          onclick={() => toggleStyle(style.id)}
        >
          <span class="text-2xl">{style.icon}</span>
          <span class="text-sm font-medium text-gray-200">{style.label}</span>
        </button>
      {/each}
    </div>
  </div>

  <!-- Color Schemes -->
  <div>
    <h4 class="text-xs font-medium text-gray-500 uppercase tracking-wider mb-2">
      Color Schemes <span class="text-gray-600 normal-case">(optional)</span>
    </h4>
    <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
      {#each COLOR_SCHEMES as scheme}
        <button
          class="flex flex-col items-center gap-1 p-3 rounded-xl border-2 transition-all cursor-pointer
            {selectedSchemes.has(scheme.id)
              ? 'border-purple-400 bg-purple-500/20 shadow-lg shadow-purple-500/10'
              : 'border-gray-600 bg-gray-800 hover:border-gray-400'}"
          onclick={() => toggleScheme(scheme.id)}
          title={scheme.description}
        >
          <span class="text-2xl">{scheme.icon}</span>
          <span class="text-sm font-medium text-gray-200">{scheme.label}</span>
          <span class="text-xs text-gray-400 text-center leading-tight">{scheme.description}</span>
        </button>
      {/each}
    </div>
  </div>
</div>
