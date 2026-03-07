/// Background polling loop — periodically fetches tracking state from the Early API
/// and emits `tracking-update` events to the Svelte frontend.
///
/// Spawned as a tokio task in `.setup()`. Reads the polling interval from AppState
/// and adjusts rate based on whether the window is visible (active) or hidden (tray).
///
/// Error handling:
/// - 404 (not tracking): emits `tracking-update` with is_tracking=false
/// - 401 (token expired): attempts re-auth with stored credentials, retries once
/// - Network/other errors: emits `tracking-error` to trigger offline UI
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;

use crate::api;
use crate::state::AppState;

/// Payload emitted to the frontend on each successful poll.
/// The frontend updates the tracking store from this data.
#[derive(Clone, Serialize)]
pub struct TrackingUpdatePayload {
    /// Whether there's an active tracking session
    pub is_tracking: bool,
    /// Full tracking data if active, null/empty if idle
    pub data: Option<serde_json::Value>,
}

/// Payload emitted when polling encounters an error.
/// The frontend updates the connection store to show offline state.
#[derive(Clone, Serialize)]
pub struct TrackingErrorPayload {
    /// Human-readable error description
    pub error: String,
    /// Whether this is an authentication error (vs network/server)
    pub is_auth_error: bool,
}

/// Attempt to re-authenticate using stored credentials.
/// Returns a fresh bearer token on success.
async fn try_reauth(handle: &AppHandle) -> Result<String, String> {
    let store = handle
        .store("credentials.json")
        .map_err(|e| e.to_string())?;

    let api_key = store
        .get("api_key")
        .and_then(|v| v.as_str().map(String::from))
        .ok_or("No stored API key")?;

    let api_secret = store
        .get("api_secret")
        .and_then(|v| v.as_str().map(String::from))
        .ok_or("No stored API secret")?;

    api::auth::sign_in(&api_key, &api_secret).await
}

/// Handle the result of a tracking API call, emitting the appropriate event.
fn emit_tracking_result(
    handle: &AppHandle,
    result: Result<serde_json::Value, String>,
) {
    match result {
        Ok(data) => {
            let _ = handle.emit("tracking-update", &TrackingUpdatePayload {
                is_tracking: true,
                data: Some(data),
            });
        }
        Err(e) if e.contains("404") => {
            // 404 = no active tracking session (normal idle state)
            let _ = handle.emit("tracking-update", &TrackingUpdatePayload {
                is_tracking: false,
                data: None,
            });
        }
        Err(e) => {
            let _ = handle.emit("tracking-error", &TrackingErrorPayload {
                error: e,
                is_auth_error: false,
            });
        }
    }
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
                match api::tracking::get_current_tracking(&token).await {
                    Ok(data) => {
                        let _ = handle.emit("tracking-update", &TrackingUpdatePayload {
                            is_tracking: true,
                            data: Some(data),
                        });
                    }
                    Err(e) if e == "UNAUTHORIZED" => {
                        // Token expired — attempt re-auth with stored credentials
                        match try_reauth(&handle).await {
                            Ok(new_token) => {
                                {
                                    let state = handle.state::<AppState>();
                                    let mut t = state.auth_token.lock().unwrap();
                                    *t = Some(new_token.clone());
                                }
                                // Retry once with the fresh token
                                let retry = api::tracking::get_current_tracking(&new_token).await;
                                emit_tracking_result(&handle, retry);
                            }
                            Err(auth_err) => {
                                let _ = handle.emit("tracking-error", &TrackingErrorPayload {
                                    error: format!("Re-authentication failed: {auth_err}"),
                                    is_auth_error: true,
                                });
                            }
                        }
                    }
                    Err(e) if e.contains("404") => {
                        // 404 = no active tracking session (normal idle state)
                        let _ = handle.emit("tracking-update", &TrackingUpdatePayload {
                            is_tracking: false,
                            data: None,
                        });
                    }
                    Err(e) => {
                        // Network or other API error — signal offline state
                        let _ = handle.emit("tracking-error", &TrackingErrorPayload {
                            error: e,
                            is_auth_error: false,
                        });
                    }
                }
            }

            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
    });
}
