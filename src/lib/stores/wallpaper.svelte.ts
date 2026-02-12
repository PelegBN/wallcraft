import { invoke } from "@tauri-apps/api/core";

let isTrying = $state(false);
let countdown = $state(0);
let countdownInterval = $state<ReturnType<typeof setInterval> | null>(null);
let errorMessage = $state<string | null>(null);

export function getWallpaperStore() {
  return {
    get isTrying() { return isTrying; },
    get countdown() { return countdown; },
    get error() { return errorMessage; },

    async tryWallpaper(imagePath: string, wallpaperMode: string | null = null) {
      errorMessage = null;

      // Save current wallpaper (best effort — don't block if it fails)
      try {
        await invoke("get_current_wallpaper");
      } catch (e) {
        console.warn("Could not save current wallpaper for restore:", e);
      }

      // This is the critical call — let errors propagate
      await invoke("set_wallpaper", { path: imagePath, mode: wallpaperMode });

      isTrying = true;
      countdown = 10;
      countdownInterval = setInterval(() => {
        countdown -= 1;
        if (countdown <= 0) {
          this.cancelTry();
        }
      }, 1000);
    },

    async approveTry() {
      if (countdownInterval) clearInterval(countdownInterval);
      isTrying = false;
      countdown = 0;
    },

    async cancelTry() {
      if (countdownInterval) clearInterval(countdownInterval);
      isTrying = false;
      countdown = 0;
      try {
        await invoke("restore_wallpaper");
      } catch {
        // Best effort restore
      }
    },
  };
}
