/// System tray integration — provides a tray icon with a context menu.
///
/// Menu items:
/// - Current status (disabled label showing tracking state)
/// - Start/Stop tracking
/// - Show/Hide miniplayer
/// - Toggle always-on-top
/// - Quit
///
/// Left-click toggles miniplayer visibility.
use log::info;
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use crate::state::AppState;

/// Sets up the system tray icon and menu. Must be called from `.setup()`.
pub fn setup_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Menu items — each has a unique ID for event handling
    let status_item =
        MenuItem::with_id(app, "status", "Not tracking", false, None::<&str>)?;
    let toggle_tracking =
        MenuItem::with_id(app, "toggle_tracking", "Start Tracking", true, None::<&str>)?;
    let show_hide =
        MenuItem::with_id(app, "show_hide", "Hide Miniplayer", true, None::<&str>)?;
    let always_on_top =
        MenuItem::with_id(app, "always_on_top", "✓ Always on Top", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit Early Mini", true, None::<&str>)?;

    // Build the context menu
    let menu = MenuBuilder::new(app)
        .item(&status_item)
        .separator()
        .item(&toggle_tracking)
        .separator()
        .item(&show_hide)
        .item(&always_on_top)
        .separator()
        .item(&quit)
        .build()?;

    // Build the tray icon using the app's default icon
    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(Image::from_bytes(include_bytes!("../icons/tray-icon.png"))?)
        .icon_as_template(true)
        .tooltip("Early Mini")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| {
            match event.id().as_ref() {
                "quit" => {
                    info!("Tray action: quit requested");
                    app.exit(0);
                }
                "show_hide" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let state = app.state::<AppState>();
                        if window.is_visible().unwrap_or(false) {
                            info!("Tray action: hiding main window");
                            let _ = window.hide();
                            if let Ok(mut v) = state.window_visible.lock() {
                                *v = false;
                            }
                            // Update menu item text — we'll update on next tray open
                        } else {
                            info!("Tray action: showing main window");
                            let _ = window.show();
                            let _ = window.set_focus();
                            if let Ok(mut v) = state.window_visible.lock() {
                                *v = true;
                            }
                        }
                    }
                }
                "always_on_top" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let currently_on_top =
                            window.is_always_on_top().unwrap_or(true);
                        let new_value = !currently_on_top;
                        info!("Tray action: set always-on-top to {new_value}");
                        let _ = window.set_always_on_top(new_value);

                        // Update settings in memory
                        let state = app.state::<AppState>();
                        let mut s = state.settings.lock().unwrap();
                        s.always_on_top = new_value;
                    }
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            // Left-click: toggle miniplayer visibility
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let state = app.state::<AppState>();
                    if window.is_visible().unwrap_or(false) {
                        info!("Tray icon click: hiding main window");
                        let _ = window.hide();
                        if let Ok(mut v) = state.window_visible.lock() {
                            *v = false;
                        }
                    } else {
                        info!("Tray icon click: showing main window");
                        let _ = window.show();
                        let _ = window.set_focus();
                        if let Ok(mut v) = state.window_visible.lock() {
                            *v = true;
                        }
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
