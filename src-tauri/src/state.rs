/// App-wide shared state, managed by Tauri's state management system.
///
/// Tauri injects this into commands via `State<'_, AppState>`.
/// The Mutex ensures safe concurrent access from multiple async commands.
use std::sync::Mutex;

pub struct AppState {
    /// Cached bearer token from the Early API. None if not yet authenticated.
    pub auth_token: Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            auth_token: Mutex::new(None),
        }
    }
}
