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
let useCustomResolution = $state(false);
let customWidth = $state(1920);
let customHeight = $state(1080);

export function getMonitorStore() {
  return {
    get layout() { return layout; },
    get loading() { return loading; },
    get error() { return error; },
    get selectedMode() { return selectedMode; },
    set selectedMode(v: "individual" | "spanning") { selectedMode = v; },
    get selectedMonitorIndex() { return selectedMonitorIndex; },
    set selectedMonitorIndex(v: number) { selectedMonitorIndex = v; },
    get useCustomResolution() { return useCustomResolution; },
    set useCustomResolution(v: boolean) { useCustomResolution = v; },
    get customWidth() { return customWidth; },
    set customWidth(v: number) { customWidth = v; },
    get customHeight() { return customHeight; },
    set customHeight(v: number) { customHeight = v; },

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

    get monitorResolution(): { width: number; height: number } {
      if (!layout) return { width: 1920, height: 1080 };
      if (selectedMode === "spanning") {
        return { width: layout.total_width, height: layout.total_height };
      }
      const m = layout.monitors[selectedMonitorIndex];
      return m ? { width: m.width, height: m.height } : { width: 1920, height: 1080 };
    },

    get targetResolution(): { width: number; height: number } {
      if (useCustomResolution) {
        return { width: customWidth, height: customHeight };
      }
      return this.monitorResolution;
    },
  };
}
