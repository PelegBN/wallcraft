import { invoke } from "@tauri-apps/api/core";

export interface AppSettings {
  ai_provider: string;
  openai_api_key: string | null;
  save_directory: string | null;
  upscale_enabled: boolean;
  upscale_factor: number;
}

const defaultSettings: AppSettings = {
  ai_provider: "pollinations",
  openai_api_key: null,
  save_directory: null,
  upscale_enabled: true,
  upscale_factor: 4,
};

let settings = $state<AppSettings>({ ...defaultSettings });
let loaded = $state(false);

export function getSettingsStore() {
  return {
    get settings() { return settings; },
    get loaded() { return loaded; },

    async load() {
      try {
        settings = await invoke<AppSettings>("get_settings");
      } catch {
        settings = { ...defaultSettings };
      }
      loaded = true;
    },

    async save() {
      await invoke("save_settings", { settings });
    },

    update(partial: Partial<AppSettings>) {
      settings = { ...settings, ...partial };
    },
  };
}
