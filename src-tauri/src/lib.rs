/// Early Mini — Tauri application entry point.
///
/// This is the main Rust entry point that configures:
/// - Plugins (store for credential persistence)
/// - Shared app state (auth token cache)
/// - Tauri commands (the API bridge to the Svelte frontend)
mod api;
mod commands;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugin: persistent key-value store for credentials and settings
        .plugin(tauri_plugin_store::Builder::new().build())
        // Shared state: injected into commands via State<'_, AppState>
        .manage(AppState::new())
        // Register all Tauri commands that the frontend can invoke
        .invoke_handler(tauri::generate_handler![
            commands::authenticate,
            commands::check_auth,
            commands::get_current_tracking,
            commands::get_activities,
            commands::start_tracking,
            commands::stop_tracking,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
