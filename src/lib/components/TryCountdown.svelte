<script lang="ts">
  let {
    countdown,
    onApprove,
    onCancel,
  }: {
    countdown: number;
    onApprove: () => void;
    onCancel: () => void;
  } = $props();

  let progress = $derived((countdown / 10) * 100);
</script>

<div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
  <div class="bg-gray-800 rounded-2xl p-8 flex flex-col items-center gap-6 shadow-2xl border border-gray-600 max-w-sm mx-4">
    <h3 class="text-xl font-semibold text-white">Wallpaper Preview</h3>
    <p class="text-gray-400 text-center text-sm">
      Your wallpaper has been set. Keep it or revert?
    </p>

    <!-- Countdown circle -->
    <div class="relative w-24 h-24">
      <svg class="w-full h-full -rotate-90" viewBox="0 0 100 100">
        <circle
          cx="50" cy="50" r="42"
          fill="none" stroke="#374151" stroke-width="8"
        />
        <circle
          cx="50" cy="50" r="42"
          fill="none" stroke="#3b82f6" stroke-width="8"
          stroke-dasharray={2 * Math.PI * 42}
          stroke-dashoffset={2 * Math.PI * 42 * (1 - progress / 100)}
          stroke-linecap="round"
          class="transition-all duration-1000 ease-linear"
        />
      </svg>
      <span class="absolute inset-0 flex items-center justify-center text-2xl font-bold text-white">
        {countdown}
      </span>
    </div>

    <div class="flex gap-4 w-full">
      <button
        class="flex-1 px-4 py-2.5 bg-green-600 hover:bg-green-500 text-white rounded-xl font-medium transition-colors cursor-pointer"
        onclick={onApprove}
      >
        Keep
      </button>
      <button
        class="flex-1 px-4 py-2.5 bg-red-600 hover:bg-red-500 text-white rounded-xl font-medium transition-colors cursor-pointer"
        onclick={onCancel}
      >
        Revert
      </button>
    </div>
  </div>
</div>
