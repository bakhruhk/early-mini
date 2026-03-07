/**
 * theme.ts — Manages the app theme (dark/light/auto).
 *
 * The `themePreference` store holds the user's choice: 'auto', 'dark', or 'light'.
 * The `applyTheme` helper sets a `data-theme` attribute on <html> so CSS
 * variables can switch between dark and light palettes.
 *
 * When preference is 'auto', the resolved theme comes from the system
 * (detected via Tauri's window.theme() API).
 */
import { writable } from 'svelte/store';

export type ThemePreference = 'auto' | 'dark' | 'light';

/** User's theme preference — persisted in Settings. */
export const themePreference = writable<ThemePreference>('auto');

/**
 * Apply the resolved theme to the document.
 * Sets `data-theme="dark"` or `data-theme="light"` on <html>.
 *
 * @param pref - The user preference ('auto', 'dark', 'light')
 * @param systemTheme - The current system theme ('dark' or 'light'), used when pref is 'auto'
 */
export function applyTheme(pref: ThemePreference, systemTheme: 'dark' | 'light') {
	const resolved = pref === 'auto' ? systemTheme : pref;
	document.documentElement.setAttribute('data-theme', resolved);
}
