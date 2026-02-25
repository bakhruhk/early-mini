/// Early Mini — Tauri application entry point.
///
/// This is the main Rust entry point that configures:
/// - Plugins (store for credential persistence)
/// - Shared app state (auth token cache, settings)
/// - System tray (show/hide, quit, always-on-top toggle)
/// - Background polling loop (tracking state sync)
/// - Tauri commands (the API bridge to the Svelte frontend)
mod api;
mod commands;
mod polling;
mod state;
mod tray;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugin: persistent key-value store for credentials and settings
        .plugin(tauri_plugin_store::Builder::new().build())
        // Shared state: injected into commands via State<'_, AppState>
        .manage(AppState::new())
        // Setup hook: runs after the app is initialized but before the event loop starts
        .setup(|app| {
            // Initialize the system tray
            tray::setup_tray(app)?;

            // Start the background polling loop
            polling::start_polling(app.handle());

            Ok(())
        })
        // Register all Tauri commands that the frontend can invoke
        .invoke_handler(tauri::generate_handler![
            commands::authenticate,
            commands::check_auth,
            commands::get_current_tracking,
            commands::get_activities,
            commands::start_tracking,
            commands::stop_tracking,
            commands::update_note,
            commands::get_today_entries,
            commands::get_settings,
            commands::save_settings,
            commands::load_settings,
            commands::save_window_state,
            commands::load_window_state,
            commands::set_window_visible,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
