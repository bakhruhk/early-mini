/**
 * connection.ts — Tracks API connectivity state.
 *
 * Updated by the polling event listeners in +page.svelte:
 * - 'tracking-update' success → online = true
 * - 'tracking-error' event   → online = false, lastError set
 *
 * Read by ConnectionIndicator and mode components to disable
 * buttons and show status when the API is unreachable.
 */
import { writable } from 'svelte/store';

export interface ConnectionState {
	online: boolean;
	lastError: string | null;
	lastSuccessAt: string | null;
}

export const connection = writable<ConnectionState>({
	online: true,
	lastError: null,
	lastSuccessAt: null,
});
