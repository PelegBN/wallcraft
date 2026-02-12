<script lang="ts">
  import { CATEGORIES } from "$lib/utils/categories";

  let {
    selected = $bindable<Set<string>>(new Set()),
  }: {
    selected?: Set<string>;
  } = $props();

  function toggle(id: string) {
    const next = new Set(selected);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    selected = next;
  }
</script>

<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3">
  {#each CATEGORIES as category}
    <button
      class="flex flex-col items-center gap-1 p-3 rounded-xl border-2 transition-all cursor-pointer
        {selected.has(category.id)
          ? 'border-blue-400 bg-blue-500/20 shadow-lg shadow-blue-500/10'
          : 'border-gray-600 bg-gray-800 hover:border-gray-400'}"
      onclick={() => toggle(category.id)}
    >
      <span class="text-2xl">{category.icon}</span>
      <span class="text-sm font-medium text-gray-200">{category.label}</span>
    </button>
  {/each}
</div>
