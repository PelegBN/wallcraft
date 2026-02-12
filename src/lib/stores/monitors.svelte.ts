import { invoke } from "@tauri-apps/api/core";

export interface MonitorInfo {
  name: string;
  width: number;
  height: number;
  x: number;
  y: number;
  scale_factor: number;
  is_primary: boolean;
}

export interface MonitorLayout {
  monitors: MonitorInfo[];
  total_width: number;
  total_height: number;
}

let layout = $state<MonitorLayout | null>(null);
let loading = $state(false);
let error = $state<string | null>(null);
let selectedMode = $state<"individual" | "spanning">("individual");
let selectedMonitorIndex = $state(0);

export function getMonitorStore() {
  return {
    get layout() { return layout; },
    get loading() { return loading; },
    get error() { return error; },
    get selectedMode() { return selectedMode; },
    set selectedMode(v: "individual" | "spanning") { selectedMode = v; },
    get selectedMonitorIndex() { return selectedMonitorIndex; },
    set selectedMonitorIndex(v: number) { selectedMonitorIndex = v; },

    async detectMonitors() {
      loading = true;
      error = null;
      try {
        layout = await invoke<MonitorLayout>("get_monitors");
      } catch (e) {
        error = String(e);
      } finally {
        loading = false;
      }
    },

    get targetResolution(): { width: number; height: number } {
      if (!layout) return { width: 1920, height: 1080 };
      if (selectedMode === "spanning") {
        return { width: layout.total_width, height: layout.total_height };
      }
      const m = layout.monitors[selectedMonitorIndex];
      return m ? { width: m.width, height: m.height } : { width: 1920, height: 1080 };
    },
  };
}
