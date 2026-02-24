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
