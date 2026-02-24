/**
 * utils.ts — Utility functions used across the app.
 *
 * Currently contains time formatting for the elapsed timer.
 */

/**
 * Format the elapsed time since a given ISO timestamp as HH:MM:SS.
 *
 * @param startedAt - ISO 8601 timestamp string (e.g. "2026-02-24T10:30:00.000")
 * @returns Formatted string like "01:23:45", or "00:00:00" if startedAt is null
 */
export function formatElapsed(startedAt: string | null): string {
	if (!startedAt) return '00:00:00';

	const start = new Date(startedAt);
	const now = new Date();
	const diffMs = now.getTime() - start.getTime();

	if (diffMs < 0) return '00:00:00';

	const hours = Math.floor(diffMs / 3_600_000);
	const minutes = Math.floor((diffMs % 3_600_000) / 60_000);
	const seconds = Math.floor((diffMs % 60_000) / 1_000);

	return [hours, minutes, seconds].map((n) => String(n).padStart(2, '0')).join(':');
}
