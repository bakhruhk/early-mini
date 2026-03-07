/**
 * selection.ts — Shared store for the currently selected activity.
 *
 * When the user is NOT tracking, they can select an activity via:
 * - The ActivitySwitcher dropdown (click)
 * - Arrow keys (keyboard navigation)
 *
 * Once tracking starts, the selection is cleared.
 * This store is shared so +page.svelte can manage keyboard navigation
 * while mode components read and display the selection.
 */
import { writable } from 'svelte/store';
import type { Activity } from './activities';

export const selectedActivity = writable<Activity | null>(null);
