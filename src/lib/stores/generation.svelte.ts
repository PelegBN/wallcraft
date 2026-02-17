import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type GenerationStatus = "idle" | "starting" | "generating" | "upscaling" | "complete" | "error";

export interface GenerationResult {
  image_path: string;
  original_width: number;
  original_height: number;
  final_width: number;
  final_height: number;
  was_upscaled: boolean;
}

interface GenerationRequest {
  styles: string[];
  color_schemes: string[];
  custom_prompt: string | null;
  width: number;
  height: number;
  provider: "Pollinations" | "OpenAi";
  target_width: number;
  target_height: number;
}

let status = $state<GenerationStatus>("idle");
let result = $state<GenerationResult | null>(null);
let errorMessage = $state<string | null>(null);
let lastRequest = $state<GenerationRequest | null>(null);
let lastUpscaleEnabled = $state(false);
let lastUpscaleFactor = $state(4);
let listenerSetup = false;

function setupListener() {
  if (listenerSetup) return;
  listenerSetup = true;
  listen<string>("generation-progress", (event) => {
    status = event.payload as GenerationStatus;
  });
}

export function getGenerationStore() {
  setupListener();

  return {
    get status() { return status; },
    get result() { return result; },
    get error() { return errorMessage; },

    get canRegenerate() { return lastRequest !== null; },

    async generate(request: GenerationRequest, upscaleEnabled: boolean = false, upscaleFactor: number = 4) {
      lastRequest = { ...request };
      lastUpscaleEnabled = upscaleEnabled;
      lastUpscaleFactor = upscaleFactor;

      status = "starting";
      result = null;
      errorMessage = null;
      try {
        result = await invoke<GenerationResult>("generate_image", { request });

        if (upscaleEnabled && result) {
          status = "upscaling";
          const upscaledPath = await invoke<string>("upscale_image", {
            inputPath: result.image_path,
            scale: upscaleFactor,
          });
          result = {
            ...result,
            image_path: upscaledPath,
            final_width: result.original_width * upscaleFactor,
            final_height: result.original_height * upscaleFactor,
            was_upscaled: true,
          };
        }

        status = "complete";
      } catch (e) {
        errorMessage = String(e);
        status = "error";
      }
    },

    async regenerate() {
      if (!lastRequest) return;
      await this.generate(lastRequest, lastUpscaleEnabled, lastUpscaleFactor);
    },

    reset() {
      status = "idle";
      result = null;
      errorMessage = null;
    },
  };
}
