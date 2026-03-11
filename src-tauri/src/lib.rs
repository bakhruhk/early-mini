/// Early Mini — Tauri application entry point.
///
/// This is the main Rust entry point that configures:
/// - Single-instance handling (focus existing window on re-launch)
/// - Logging (stdout + file logs)
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
use log::{error, info, warn};

use state::AppState;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugin: enforce single app instance and focus existing window on relaunch
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            info!(
                "Second instance launch detected. args={:?}, cwd={}",
                argv, cwd
            );

            if let Some(window) = app.get_webview_window("main") {
                if let Err(err) = window.unminimize() {
                    warn!("Failed to unminimize main window on second launch: {err}");
                }
                if let Err(err) = window.show() {
                    warn!("Failed to show main window on second launch: {err}");
                }
                if let Err(err) = window.set_focus() {
                    warn!("Failed to focus main window on second launch: {err}");
                }
                #[cfg(windows)]
                if let Err(err) =
                    window.request_user_attention(Some(tauri::UserAttentionType::Critical))
                {
                    warn!("Failed to request user attention on second launch: {err}");
                }
            } else {
                warn!("Single-instance callback could not find main window");
            }
        }))
        // Plugin: structured logging (stdout + app log directory file)
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                ])
                .build(),
        )
        // Plugin: persistent key-value store for credentials and settings
        .plugin(tauri_plugin_store::Builder::new().build())
        // Shared state: injected into commands via State<'_, AppState>
        .manage(AppState::new())
        // Global window events (Windows close policy)
        .on_window_event(|window, event| {
            #[cfg(windows)]
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                info!(
                    "Close requested for window '{}' on Windows; exiting application",
                    window.label()
                );
                api.prevent_close();
                window.app_handle().exit(0);
            }
            #[cfg(not(windows))]
            {
                let _ = window;
                let _ = event;
            }
        })
        // Setup hook: runs after the app is initialized but before the event loop starts
        .setup(|app| {
            info!("Application setup starting");
            // Initialize the system tray
            if let Err(err) = tray::setup_tray(app) {
                error!("Tray initialization failed: {err}");
                return Err(err);
            }
            info!("System tray initialized");

            // Start the background polling loop
            polling::start_polling(app.handle());
            info!("Background polling loop started");

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
                let global_shortcut_plugin = app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut, event| {
                            if shortcut == &toggle_shortcut
                                && event.state() == ShortcutState::Pressed
                            {
                                if let Some(window) = app.get_webview_window("main") {
                                    let state = app.state::<AppState>();
                                    if window.is_visible().unwrap_or(false) {
                                        info!("Global shortcut toggle: hiding main window");
                                        let _ = window.hide();
                                        if let Ok(mut v) = state.window_visible.lock() {
                                            *v = false;
                                        }
                                    } else {
                                        info!("Global shortcut toggle: showing main window");
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
                );

                if let Err(err) = global_shortcut_plugin {
                    warn!(
                        "Global shortcut plugin failed to initialize (continuing without global shortcut): {err}"
                    );
                } else {
                    match app.global_shortcut().register(toggle_shortcut) {
                        Ok(_) => info!("Registered global shortcut: super+shift+E"),
                        Err(err) => warn!(
                            "Global shortcut registration failed (continuing without global shortcut): {err}"
                        ),
                    }
                }
            }

            info!("Application setup completed");
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
