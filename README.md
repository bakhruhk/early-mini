# Early Mini

[![Release](https://github.com/bakhruhk/early-mini/actions/workflows/release.yml/badge.svg)](https://github.com/bakhruhk/early-mini/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Latest Release](https://img.shields.io/github/v/release/bakhruhk/early-mini?include_prereleases)](https://github.com/bakhruhk/early-mini/releases)

A lightweight, always-on-top desktop miniplayer for [Early (Timeular)](https://timeular.com) time tracking — inspired by the Spotify Miniplayer.

<!-- TODO: Add screenshot/GIF of the three modes -->

## Features

- **Three resizable modes** — Pill (compact), Card (with description), and Expanded (full controls + daily summary)
- **Start, stop, and switch** activities directly from the miniplayer
- **Live elapsed time** counter with smooth per-second ticking between API polls
- **Activity switcher** dropdown with color indicators
- **Inline description editor** (Card & Expanded modes)
- **Today's tracked time** summary (Expanded mode)
- **System tray** with status display, quick actions, and left-click toggle
- **Global hotkey** (`Cmd/Ctrl+Shift+E`) to toggle window visibility
- **Keyboard navigation** — Arrow keys cycle activities, Enter starts/stops tracking
- **Dark/light theme** that follows system preference (with manual override)
- **macOS vibrancy** and **Windows Mica** backdrop effects
- **Configurable idle opacity** with smooth fade on hover
- **Offline detection** with connection indicator and automatic recovery
- **Auto re-authentication** on token expiry
- **Always-on-top** by default (toggleable via tray menu)
- **Window position & size** remembered across restarts

## Download

Download the latest installer from the [Releases](https://github.com/bakhruhk/early-mini/releases) page:

| Platform | Installer |
|----------|----------|
| macOS (Apple Silicon) | `.dmg` |
| macOS (Intel) | `.dmg` |
| Windows | `.msi` / `.exe` |

> **Note:** Unsigned builds may trigger OS security warnings. On macOS, right-click the app → Open. On Windows, click "More info" → "Run anyway."

## Build from Source

### Prerequisites

- [Node.js](https://nodejs.org) LTS (v20+)
- [Rust](https://rustup.rs) stable
- [Tauri CLI](https://v2.tauri.app/start/create-project/) (`npm install` installs it as a dev dependency)
- **macOS:** Xcode Command Line Tools (`xcode-select --install`)
- **Windows:** [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (pre-installed on Windows 11)

### Setup

```bash
git clone https://github.com/bakhruhk/early-mini.git
cd early-mini
npm install
npm run tauri dev
```

To create a production build:

```bash
npm run tauri build
```

## Configuration

Early Mini connects to the [Early API v4](https://early.app). On first launch, a setup screen prompts for your API credentials:

1. Log in to your Early/Timeular account
2. Navigate to **Settings → Developer** to find your API Key and API Secret
3. Enter them in the Early Mini setup screen

Credentials are stored securely in the OS keychain (macOS Keychain / Windows Credential Manager) — never in plaintext.

### Polling

The app polls `GET /tracking` to stay in sync with the Early cloud (and the physical Timeular tracker):

- **3 seconds** when the miniplayer is visible (default)
- **10 seconds** when minimized to tray

Polling intervals are configurable in Settings.

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd/Ctrl+Shift+E` | Toggle miniplayer visibility (global) |
| `↑` / `↓` | Cycle through activities (when idle) |
| `Enter` | Start/stop tracking |

## Architecture

- **Tauri v2** (Rust backend) + **SvelteKit** + **TypeScript** frontend
- All API calls made from Rust via `reqwest` — no CORS issues, token stays out of WebView
- Background polling loop runs in Rust, pushes updates to frontend via Tauri events
- Window state persisted via `tauri-plugin-store`
- Credentials stored in OS keychain
- ~10MB installer, ~30-50MB RAM at idle

## License

[MIT](LICENSE) © 2026 Harshit Bakhru
