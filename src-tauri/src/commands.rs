/// Tauri commands — the bridge between the Svelte frontend and the Rust backend.
///
/// Each `#[tauri::command]` function can be called from the frontend via:
///   `import { invoke } from '@tauri-apps/api/core';`
///   `const result = await invoke('command_name', { arg1: 'value' });`
///
/// Commands receive Tauri-managed state (AppState) and the app handle
/// for accessing plugins like the store.
use serde_json::json;
use tauri::State;
use tauri_plugin_store::StoreExt;

use crate::api;
use crate::state::AppState;

/// Authenticate with the Early API and persist credentials.
///
/// Called from the SetupScreen when the user enters their API key and secret.
/// On success, stores credentials in the local store and caches the token in memory.
#[tauri::command]
pub async fn authenticate(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    api_key: String,
    api_secret: String,
) -> Result<bool, String> {
    // Attempt to sign in with the Early API
    let token = api::auth::sign_in(&api_key, &api_secret).await?;

    // Persist credentials to the local store (survives app restarts).
    // In Phase 3, this will be moved to the OS keychain for better security.
    let store = app.store("credentials.json").map_err(|e| e.to_string())?;
    store.set("api_key", json!(api_key));
    store.set("api_secret", json!(api_secret));

    // Cache the token in memory for use by other commands
    let mut t = state.auth_token.lock().map_err(|e| e.to_string())?;
    *t = Some(token);

    Ok(true)
}

/// Check if the user is already authenticated (has stored credentials).
///
/// Called on app startup to decide whether to show SetupScreen or PillMode.
/// If credentials exist in the store, attempts to re-authenticate silently.
#[tauri::command]
pub async fn check_auth(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    // If we already have a cached token, we're good
    let has_token = state
        .auth_token
        .lock()
        .map_err(|e| e.to_string())?
        .is_some();
    if has_token {
        return Ok(true);
    }

    // Try to load stored credentials and re-authenticate
    let store = app.store("credentials.json").map_err(|e| e.to_string())?;
    let api_key = store.get("api_key").and_then(|v| v.as_str().map(String::from));
    let api_secret = store
        .get("api_secret")
        .and_then(|v| v.as_str().map(String::from));

    if let (Some(key), Some(secret)) = (api_key, api_secret) {
        match api::auth::sign_in(&key, &secret).await {
            Ok(token) => {
                let mut t = state.auth_token.lock().map_err(|e| e.to_string())?;
                *t = Some(token);
                Ok(true)
            }
            // Stored credentials are invalid — user needs to re-enter them
            Err(_) => Ok(false),
        }
    } else {
        Ok(false)
    }
}

/// Get the current tracking state from the Early API.
///
/// Returns the full JSON response: activity info, start time, note, etc.
/// If nothing is being tracked, the API returns an error (which we pass through).
#[tauri::command]
pub async fn get_current_tracking(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let token = state
        .auth_token
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or("Not authenticated")?;

    api::tracking::get_current_tracking(&token).await
}

/// Get all activities from the Early API.
///
/// Returns the full JSON response with the activities array.
/// Used to populate the activity switcher dropdown.
#[tauri::command]
pub async fn get_activities(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let token = state
        .auth_token
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or("Not authenticated")?;

    api::activities::get_activities(&token).await
}

/// Start tracking a specific activity.
#[tauri::command]
pub async fn start_tracking(
    state: State<'_, AppState>,
    activity_id: String,
) -> Result<serde_json::Value, String> {
    let token = state
        .auth_token
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or("Not authenticated")?;

    api::tracking::start_tracking(&token, &activity_id).await
}

/// Stop the currently running timer.
#[tauri::command]
pub async fn stop_tracking(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let token = state
        .auth_token
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or("Not authenticated")?;

    api::tracking::stop_tracking(&token).await
}
