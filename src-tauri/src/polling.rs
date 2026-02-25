/// Background polling loop — periodically fetches tracking state from the Early API
/// and emits `tracking-update` events to the Svelte frontend.
///
/// Spawned as a tokio task in `.setup()`. Reads the polling interval from AppState
/// and adjusts rate based on whether the window is visible (active) or hidden (tray).
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

use crate::api;
use crate::state::AppState;

/// Payload emitted to the frontend on each poll.
/// The frontend updates the tracking store from this data.
#[derive(Clone, Serialize)]
pub struct TrackingUpdatePayload {
    /// Whether there's an active tracking session
    pub is_tracking: bool,
    /// Full tracking data if active, null/empty if idle
    pub data: Option<serde_json::Value>,
}

/// Starts the background polling loop. Must be called from `.setup()`.
pub fn start_polling(app: &AppHandle) {
    let handle = app.clone();

    tauri::async_runtime::spawn(async move {
        loop {
            // Read current settings and auth state
            let (interval_secs, has_token) = {
                let state = handle.state::<AppState>();
                let token_exists = state
                    .auth_token
                    .lock()
                    .map(|t| t.is_some())
                    .unwrap_or(false);

                if !token_exists {
                    // Not authenticated yet — wait and retry
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }

                let settings = state.settings.lock().unwrap().clone();

                // If polling is disabled, just sleep and check again later
                if !settings.polling_enabled {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }

                let visible = state.window_visible.lock().map(|v| *v).unwrap_or(true);
                let interval = if visible {
                    settings.polling_interval_secs
                } else {
                    settings.tray_polling_interval_secs
                };

                (interval, token_exists)
            };

            if !has_token {
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }

            // Fetch tracking state
            let token = {
                let state = handle.state::<AppState>();
                state
                    .auth_token
                    .lock()
                    .ok()
                    .and_then(|t| t.clone())
            };

            if let Some(token) = token {
                let payload = match api::tracking::get_current_tracking(&token).await {
                    Ok(data) => TrackingUpdatePayload {
                        is_tracking: true,
                        data: Some(data),
                    },
                    Err(_) => TrackingUpdatePayload {
                        is_tracking: false,
                        data: None,
                    },
                };

                // Emit to all frontend listeners
                let _ = handle.emit("tracking-update", &payload);
            }

            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
    });
}
