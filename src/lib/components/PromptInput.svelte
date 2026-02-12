<script lang="ts">
  let {
    value = $bindable(""),
    placeholder = "Describe your perfect wallpaper...",
  }: {
    value?: string;
    placeholder?: string;
  } = $props();
</script>

<div class="w-full">
  <textarea
    class="w-full bg-gray-800 border border-gray-600 rounded-xl px-4 py-3 text-gray-100
           placeholder-gray-500 focus:border-blue-400 focus:outline-none focus:ring-1
           focus:ring-blue-400 resize-none transition-colors"
    rows={3}
    {placeholder}
    bind:value
  ></textarea>
</div>
