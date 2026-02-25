/**
 * settings.ts — Svelte store for user-configurable preferences.
 *
 * Settings are loaded from the Rust backend (which persists them via tauri-plugin-store)
 * and cached here for reactive UI updates.
 */
import { writable } from 'svelte/store';

/** Shape of user settings — mirrors the Rust `Settings` struct. */
export interface AppSettings {
	pollingEnabled: boolean;
	pollingIntervalSecs: number;
	trayPollingIntervalSecs: number;
	alwaysOnTop: boolean;
}

/** Default settings matching the Rust defaults. */
const defaults: AppSettings = {
	pollingEnabled: true,
	pollingIntervalSecs: 3,
	trayPollingIntervalSecs: 10,
	alwaysOnTop: true
};

/** The global settings store. */
export const settings = writable<AppSettings>(defaults);
