# WallCraft

A desktop app that generates AI-powered wallpapers perfectly sized for your screen. Supports multi-monitor setups with spanning wallpapers across all displays.

## What It Does

1. **Detects your monitors** — resolution, position, and scale factor for each display
2. **Generates wallpapers with AI** — choose from 10 built-in categories (Nature, Space, Cyberpunk, etc.), add your own prompt, or go fully custom
3. **Fits your screen perfectly** — generates at your exact resolution, with optional AI upscaling for 4K+ displays
4. **Multi-monitor spanning** — create a single wallpaper that stretches seamlessly across all your monitors
5. **Try before you commit** — preview the wallpaper on your actual desktop with a 10-second "keep or revert" countdown

## How It Works

- **Free by default** — uses [Pollinations.ai](https://pollinations.ai) for image generation (no account or API key needed)
- **Optional premium** — plug in your own OpenAI API key for DALL-E 3 generation
- **AI upscaling** — bundled Real-ESRGAN upscaler can enhance images to 4K+ from lower resolution AI output

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Desktop framework | [Tauri 2](https://tauri.app) | Lightweight cross-platform app (~10MB vs Electron's ~100MB) |
| Frontend | [SvelteKit](https://svelte.dev) + [Tailwind CSS v4](https://tailwindcss.com) | UI with reactive state |
| Backend | Rust | Monitor detection, API calls, wallpaper management, file ops |
| AI Generation | Pollinations.ai / OpenAI DALL-E 3 | Image generation APIs |
| AI Upscaling | [Real-ESRGAN](https://github.com/xinntao/Real-ESRGAN) | Upscale to 4K+ resolution |

## Getting Started

### Prerequisites

**Rust** (install via [rustup](https://rustup.rs)):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Node.js** (v18+):
```bash
# Check your version
node --version
```

**System libraries** (Linux only):
```bash
sudo apt-get install -y pkg-config libglib2.0-dev libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### Setup

```bash
# Clone and install dependencies
cd ~/Development/smart-desktop-wallpaper
npm install

# Download Real-ESRGAN binary (for AI upscaling, optional)
# Linux:
curl -sL "https://github.com/xinntao/Real-ESRGAN/releases/download/v0.2.5.0/realesrgan-ncnn-vulkan-20220424-ubuntu.zip" -o /tmp/realesrgan.zip
unzip /tmp/realesrgan.zip -d /tmp/realesrgan
cp /tmp/realesrgan/realesrgan-ncnn-vulkan src-tauri/binaries/realesrgan-ncnn-vulkan-x86_64-unknown-linux-gnu
chmod +x src-tauri/binaries/realesrgan-ncnn-vulkan-x86_64-unknown-linux-gnu
mkdir -p src-tauri/binaries/models
cp /tmp/realesrgan/models/realesrgan-x4plus.{bin,param} src-tauri/binaries/models/
```

### Run

```bash
# Development (hot reload)
npx tauri dev

# Production build
npx tauri build
```

Built packages will be in `src-tauri/target/release/bundle/`:
- `.deb` (Debian/Ubuntu)
- `.rpm` (Fedora/RHEL)
- `.AppImage` (portable Linux)

## App Flow

### 1. Home Screen
Detects all connected monitors and shows a visual layout diagram. Pick whether you want a wallpaper for a single monitor or a spanning wallpaper across all displays.

### 2. Generate Screen
Two modes:
- **Categories** — pick one or more from: Nature, Abstract, Space, Cityscape, Fantasy, Minimalist, Ocean, Mountains, Cyberpunk, Seasons. Optionally add a custom prompt on top.
- **Direct Prompt** — write exactly what you want, no category influence.

Hit "Generate" and wait for the AI to create your wallpaper.

### 3. Preview Screen
See your generated wallpaper with three options:
- **Save** — save the image file to disk
- **Try** — sets it as your actual desktop wallpaper for 10 seconds. A countdown dialog lets you **Keep** it or **Revert** to your previous wallpaper.
- **Scrap** — delete the image and go back to generate another

## Settings

Open Settings from the navbar to configure:
- **AI Provider** — Pollinations (free) or OpenAI (requires API key)
- **OpenAI API Key** — only shown when OpenAI is selected
- **Default Save Directory** — where wallpapers are saved
- **AI Upscaling** — enable/disable Real-ESRGAN, choose 2x or 4x scale factor

## Project Structure

```
src/                          # SvelteKit frontend
  routes/+page.svelte         # Main page (view switching)
  lib/
    views/                    # HomeView, GenerateView, PreviewView, SettingsView
    components/               # Navbar, MonitorLayout, CategorySelector, etc.
    stores/                   # Reactive state (Svelte 5 runes)
    utils/                    # Categories, prompt builder, navigation

src-tauri/                    # Rust backend
  src/
    commands/                 # Tauri commands (what the frontend calls)
    services/                 # Business logic (API clients, wallpaper manager)
    models/                   # Data types
  binaries/                   # Real-ESRGAN sidecar (not in git, download separately)
```

## Still To Do

- [ ] macOS and Windows Real-ESRGAN binaries (currently Linux only)
- [ ] Automated tests (Rust unit tests + Vitest frontend tests)
- [ ] Per-monitor wallpaper support (set different wallpapers per display)
- [ ] Exact resolution resize/crop before setting wallpaper
- [ ] Custom app icon
- [ ] Better error messages in the UI

## IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).