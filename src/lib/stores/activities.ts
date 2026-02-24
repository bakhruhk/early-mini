/**
 * activities.ts — Svelte store for the user's activity list.
 *
 * Activities are fetched once on authentication and cached here.
 * The activity switcher dropdown reads from this store.
 */
import { writable } from 'svelte/store';

/** Shape of a single Early activity. */
export interface Activity {
	id: string;
	name: string;
	color: string;
}

/** The global activities store. Empty until fetched after authentication. */
export const activities = writable<Activity[]>([]);
