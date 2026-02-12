<script lang="ts">
  import Navbar from "$lib/components/Navbar.svelte";
  import HomeView from "$lib/views/HomeView.svelte";
  import GenerateView from "$lib/views/GenerateView.svelte";
  import PreviewView from "$lib/views/PreviewView.svelte";
  import SettingsView from "$lib/views/SettingsView.svelte";
  import { getNavigation } from "$lib/utils/navigation.svelte";

  const nav = getNavigation();
</script>

<div class="min-h-screen bg-gray-950 text-gray-100 flex flex-col">
  <Navbar />
  <main class="flex-1 overflow-y-auto">
    {#if nav.current === "home"}
      <HomeView />
    {:else if nav.current === "generate"}
      <GenerateView />
    {:else if nav.current === "preview"}
      <PreviewView />
    {:else if nav.current === "settings"}
      <SettingsView />
    {/if}
  </main>
</div>