/**
 * api.ts — Typed wrappers around Tauri's invoke() function.
 *
 * HOW THIS WORKS:
 * Tauri's `invoke()` calls a Rust function defined with #[tauri::command].
 * The function name string must match the Rust function name exactly.
 * Arguments are passed as a single object and deserialized into the Rust function's parameters.
 * The return value is the Rust function's Ok() value, serialized as JSON.
 * If the Rust function returns Err(), invoke() throws an error.
 */
import { invoke } from '@tauri-apps/api/core';

/**
 * Authenticate with the Early API.
 * Calls the Rust `authenticate` command which signs in and stores credentials.
 */
export async function authenticate(apiKey: string, apiSecret: string): Promise<boolean> {
	return invoke('authenticate', { apiKey, apiSecret });
}

/**
 * Check if the user has stored credentials and can auto-authenticate.
 * Returns true if authenticated, false if setup is needed.
 */
export async function checkAuth(): Promise<boolean> {
	return invoke('check_auth');
}

/**
 * Fetch the current tracking state from the Early API.
 * Returns the raw JSON response from GET /tracking.
 */
export async function getCurrentTracking(): Promise<any> {
	return invoke('get_current_tracking');
}

/**
 * Fetch all activities from the Early API.
 * Returns the raw JSON response from GET /activities.
 */
export async function getActivities(): Promise<any> {
	return invoke('get_activities');
}

/**
 * Start tracking a specific activity.
 */
export async function startTracking(activityId: string): Promise<any> {
	return invoke('start_tracking', { activityId });
}

/**
 * Stop the currently running timer.
 */
export async function stopTracking(): Promise<any> {
	return invoke('stop_tracking');
}

/**
 * Update the note/description on the currently running tracker.
 */
export async function updateNote(text: string): Promise<any> {
	return invoke('update_note', { text });
}

/**
 * Fetch today's time entries for the daily summary.
 */
export async function getTodayEntries(): Promise<any> {
	return invoke('get_today_entries');
}

/**
 * Load persisted settings from the backend.
 */
export async function loadSettings(): Promise<any> {
	return invoke('load_settings');
}

/**
 * Save updated settings to the backend.
 */
export async function saveSettings(newSettings: any): Promise<boolean> {
	return invoke('save_settings', { newSettings });
}

/**
 * Save the current window position and size.
 */
export async function saveWindowState(x: number, y: number, width: number, height: number): Promise<boolean> {
	return invoke('save_window_state', { x, y, width, height });
}

/**
 * Load the saved window geometry.
 */
export async function loadWindowState(): Promise<any> {
	return invoke('load_window_state');
}

/**
 * Tell the backend whether the window is visible (affects polling rate).
 */
export async function setWindowVisible(visible: boolean): Promise<boolean> {
	return invoke('set_window_visible', { visible });
}
