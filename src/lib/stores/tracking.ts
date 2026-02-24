/**
 * tracking.ts — Svelte store for the current tracking state.
 *
 * SVELTE STORES EXPLAINED:
 * A "store" is Svelte's way to share reactive state between components.
 * - `writable()` creates a store you can read AND write to.
 * - In .svelte files, prefix with $ to auto-subscribe: `$tracking.isTracking`
 * - The $ prefix auto-subscribes on mount and auto-unsubscribes on destroy,
 *   so you never have to worry about memory leaks.
 * - To update from TypeScript: `tracking.set(newValue)` or `tracking.update(fn)`
 */
import { writable } from 'svelte/store';

/** Shape of the tracking state used throughout the app. */
export interface TrackingState {
	isTracking: boolean;
	activityId: string | null;
	activityName: string | null;
	activityColor: string | null;
	startedAt: string | null;
	noteText: string | null;
}

/** The global tracking store, initialized to idle state. */
export const tracking = writable<TrackingState>({
	isTracking: false,
	activityId: null,
	activityName: null,
	activityColor: null,
	startedAt: null,
	noteText: null
});
