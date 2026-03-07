/// App-wide shared state, managed by Tauri's state management system.
///
/// Tauri injects this into commands via `State<'_, AppState>`.
/// The Mutex ensures safe concurrent access from multiple async commands.
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

/// User-configurable settings, persisted via tauri-plugin-store.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    /// Whether continuous background polling is enabled (default: true)
    pub polling_enabled: bool,
    /// Polling interval in seconds when the window is visible (default: 3)
    pub polling_interval_secs: u64,
    /// Polling interval in seconds when minimized to tray (default: 10)
    pub tray_polling_interval_secs: u64,
    /// Whether the window stays on top of all other windows (default: true)
    pub always_on_top: bool,
    /// Theme preference: "auto" (follow system), "dark", or "light" (default: "auto")
    #[serde(default = "default_theme")]
    pub theme_preference: String,
    /// Window opacity when idle / mouse not hovering (0.0–1.0, default: 0.85)
    #[serde(default = "default_opacity")]
    pub idle_opacity: f64,
}

fn default_theme() -> String {
    "auto".to_string()
}

fn default_opacity() -> f64 {
    0.85
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            polling_enabled: true,
            polling_interval_secs: 3,
            tray_polling_interval_secs: 10,
            always_on_top: true,
            theme_preference: default_theme(),
            idle_opacity: default_opacity(),
        }
    }
}

/// Saved window geometry, restored on launch.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WindowGeometry {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct AppState {
    /// Cached bearer token from the Early API. None if not yet authenticated.
    pub auth_token: Mutex<Option<String>>,
    /// User-configurable settings.
    pub settings: Mutex<Settings>,
    /// Whether the miniplayer window is currently visible (for tray polling rate).
    pub window_visible: Mutex<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            auth_token: Mutex::new(None),
            settings: Mutex::new(Settings::default()),
            window_visible: Mutex::new(true),
        }
    }
}
