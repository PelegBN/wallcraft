# WallCraft - Project Context for Claude Sessions

## What This Is

WallCraft is a cross-platform desktop app (Windows, macOS, Linux) that generates AI-powered wallpapers tailored to the user's screen resolution, including multi-monitor spanning support. Built with Tauri 2 + SvelteKit + Rust.

## Tech Stack & Why

- **Tauri 2.x** (not Electron) — chosen for lightweight bundle size (~10MB vs ~100MB), cross-platform support, and Rust backend for system-level operations
- **SvelteKit** with `adapter-static` in SPA mode — scaffolded via `npm create tauri-app`, uses file-based routing disabled (`ssr = false`), plain view-switching via `$state` rune in navigation store
- **Svelte 5** with runes (`$state`, `$derived`, `$effect`) — NOT the old `writable`/`derived` store API. Reactive state lives in `.svelte.ts` files
- **Tailwind CSS v4** — configured via `@tailwindcss/vite` plugin (not PostCSS), imported as `@import "tailwindcss"` in `app.css`
- **Rust backend** — handles monitor detection, AI API calls, wallpaper set/restore, file operations, Real-ESRGAN sidecar management
- **Pollinations.ai** (free, no API key) as default image generation backend; OpenAI DALL-E 3 as optional paid alternative
- **Real-ESRGAN ncnn-vulkan** — bundled as Tauri sidecar binary for AI upscaling. Binary naming follows Tauri convention: `realesrgan-ncnn-vulkan-{target-triple}`

## Project Structure

```
src-tauri/
  src/
    lib.rs              # Tauri app setup, plugin registration, command handlers
    main.rs             # Entry point (calls lib::run)
    error.rs            # AppError enum (implements Serialize for Tauri)
    state.rs            # AppState (Mutex-wrapped previous wallpaper, temp dir)
    commands/           # Tauri #[tauri::command] functions
      monitor.rs        # get_monitors — uses window.available_monitors()
      generation.rs     # generate_image — dispatches to pollinations/openai service
      upscale.rs        # upscale_image — spawns Real-ESRGAN sidecar
      wallpaper.rs      # get_current/set/restore wallpaper
      settings.rs       # get/save settings (JSON file in config dir)
      files.rs          # save_image_to_disk, delete_temp_image
    services/           # Business logic (called by commands)
      pollinations.rs   # GET https://image.pollinations.ai/prompt/{prompt}?width=...
      openai.rs         # POST /v1/images/generations (DALL-E 3)
      upscaler.rs       # Spawns realesrgan-ncnn-vulkan sidecar via tauri-plugin-shell
      wallpaper_manager.rs  # Wraps `wallpaper` crate (v3) for cross-platform support
    models/             # Serde structs
      monitor.rs        # MonitorInfo, MonitorLayout
      generation.rs     # AiProvider, GenerationRequest, GenerationResult
      settings.rs       # AppSettings with defaults
  binaries/             # Real-ESRGAN sidecar (gitignored, download separately)
  capabilities/default.json  # Tauri permissions (core, dialog, fs, shell, opener)
  tauri.conf.json       # Bundle config with externalBin and resources

src/
  app.html              # SvelteKit shell
  app.css               # Tailwind import
  routes/
    +layout.svelte      # Imports app.css, renders children
    +layout.ts          # ssr = false
    +page.svelte        # Root — Navbar + view switching (home/generate/preview/settings)
  lib/
    components/         # Reusable UI components
      Navbar.svelte
      MonitorLayout.svelte    # Visual monitor diagram (scaled CSS rectangles)
      CategorySelector.svelte # Grid of toggle-able category cards
      PromptInput.svelte      # Textarea for prompts
      GenerationProgress.svelte # Spinner + status text
      ImagePreview.svelte     # Shows generated image via convertFileSrc
      TryCountdown.svelte     # 10-second circular countdown modal
    views/              # Page-level components (switched in +page.svelte)
      HomeView.svelte         # Monitor detection, mode selection
      GenerateView.svelte     # Category/prompt input, triggers generation
      PreviewView.svelte      # Image viewer with Save/Scrap/Try
      SettingsView.svelte     # Provider, API key, save dir, upscale toggle
    stores/             # Svelte 5 rune-based stores (.svelte.ts files)
      monitors.svelte.ts      # Monitor layout, selected mode/index, target resolution
      generation.svelte.ts    # Generation status, result, calls generate + optional upscale
      wallpaper.svelte.ts     # Try/approve/cancel wallpaper flow with countdown
      settings.svelte.ts      # App settings with load/save/update
    utils/
      categories.ts           # 10 built-in category definitions
      prompt-builder.ts       # Maps category IDs to prompt fragments
      navigation.svelte.ts    # Simple view-switching state (no router library)
```

## Key Architecture Decisions

1. **Two-step pipeline** (generate then upscale as separate Tauri commands) — gives frontend granular progress control and lets user skip upscaling
2. **`wallpaper` crate v3** (not v4, which doesn't exist on crates.io) for cross-platform wallpaper set/get/mode
3. **Simple `$state` view switching** instead of SvelteKit routing — only 4 views, avoids URL routing complexity in a desktop app
4. **Settings stored as JSON file** in OS config dir via `dirs::config_dir()` (not tauri-plugin-store) — simpler, no additional plugin dep
5. **Pollinations.ai as default free provider** — endpoint: `https://image.pollinations.ai/prompt/{encoded}?width={w}&height={h}&nologo=true&seed={random}`
6. **Real-ESRGAN as Tauri sidecar** — uses `tauri-plugin-shell` to spawn the binary, model files bundled as resources

## App Flow

1. **Home** — detect monitors via `get_monitors` command, show visual layout, pick individual or spanning mode
2. **Generate** — multi-select categories + optional prompt, OR direct prompt mode, then generate
3. **Preview** — in-app image viewer with Save (file dialog), Scrap (delete + go back), Try (set wallpaper + 10s countdown with Keep/Revert)

## What's Done

- Full Tauri 2 project with all Rust commands and services
- Complete SvelteKit frontend with all views, components, and stores
- Pollinations.ai integration (free, no key)
- OpenAI DALL-E 3 integration (settings page for API key)
- Real-ESRGAN sidecar integration (Linux x86_64 binary downloaded)
- Wallpaper set/restore with spanning mode support
- Monitor detection with multi-monitor layout visualization
- 10-second "Try" wallpaper approval countdown
- Builds successfully to .deb, .rpm, .AppImage

## What Still Needs Work

- **Cross-platform Real-ESRGAN binaries** — only Linux x86_64 binary is downloaded. Need macOS (aarch64-apple-darwin) and Windows (x86_64-pc-windows-msvc.exe) binaries for full cross-platform upscaling support
- **Testing** — no automated tests yet. Need Rust unit tests (prompt building, model serialization) and frontend tests (Vitest)
- **Per-monitor wallpaper** — currently only sets a single wallpaper globally. The `more-wallpapers` crate could enable per-monitor wallpapers on some platforms
- **Image resize/crop** — if the generated/upscaled image doesn't exactly match the target resolution, it should be resized using the Rust `image` crate before setting as wallpaper
- **Error UX** — error messages from the backend are shown as raw strings. Could be more user-friendly
- **App icon** — still using the default Tauri icon

## Build Commands

```bash
npm install                    # Install frontend deps
npm run build                  # Build frontend only
cargo check                    # Check Rust compilation (from src-tauri/)
npx tauri dev                  # Dev mode with hot reload
npx tauri build                # Production build with bundled packages
```

## System Dependencies (Linux)

```bash
sudo apt-get install -y pkg-config libglib2.0-dev libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

## Real-ESRGAN Binary Setup

Download from https://github.com/xinntao/Real-ESRGAN/releases/tag/v0.2.5.0 and place in `src-tauri/binaries/`:
- Linux: `realesrgan-ncnn-vulkan-x86_64-unknown-linux-gnu`
- macOS: `realesrgan-ncnn-vulkan-aarch64-apple-darwin`
- Windows: `realesrgan-ncnn-vulkan-x86_64-pc-windows-msvc.exe`

Model files go in `src-tauri/binaries/models/`:
- `realesrgan-x4plus.bin`
- `realesrgan-x4plus.param`