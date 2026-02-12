<script lang="ts">
  import { getNavigation, type View } from "$lib/utils/navigation.svelte";

  const nav = getNavigation();

  function goTo(view: View) {
    nav.navigate(view);
  }
</script>

<nav class="flex items-center justify-between px-6 py-3 bg-gray-900 text-white border-b border-gray-700">
  <button class="text-xl font-bold tracking-tight cursor-pointer" onclick={() => goTo("home")}>
    WallCraft
  </button>
  <div class="flex gap-2">
    <button
      class="px-3 py-1.5 rounded-lg text-sm transition-colors cursor-pointer {nav.current === 'home' ? 'bg-blue-600' : 'hover:bg-gray-700'}"
      onclick={() => goTo("home")}
    >
      Home
    </button>
    <button
      class="px-3 py-1.5 rounded-lg text-sm transition-colors cursor-pointer {nav.current === 'generate' ? 'bg-blue-600' : 'hover:bg-gray-700'}"
      onclick={() => goTo("generate")}
    >
      Generate
    </button>
    <button
      class="px-3 py-1.5 rounded-lg text-sm transition-colors cursor-pointer {nav.current === 'settings' ? 'bg-blue-600' : 'hover:bg-gray-700'}"
      onclick={() => goTo("settings")}
    >
      Settings
    </button>
  </div>
</nav>
