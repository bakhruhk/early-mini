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

            // Register global hotkey: Cmd+Shift+E (macOS) / Ctrl+Shift+E (Windows/Linux)
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let toggle_shortcut = Shortcut::new(
                    Some(Modifiers::SUPER | Modifiers::SHIFT),
                    Code::KeyE,
                );

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut, event| {
                            use tauri::Manager;
                            if shortcut == &toggle_shortcut
                                && event.state() == ShortcutState::Pressed
                            {
                                if let Some(window) = app.get_webview_window("main") {
                                    let state = app.state::<AppState>();
                                    if window.is_visible().unwrap_or(false) {
                                        let _ = window.hide();
                                        if let Ok(mut v) = state.window_visible.lock() {
                                            *v = false;
                                        }
                                    } else {
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                        if let Ok(mut v) = state.window_visible.lock() {
                                            *v = true;
                                        }
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(toggle_shortcut)?;
            }

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
