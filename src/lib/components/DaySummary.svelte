<!--
  DaySummary.svelte — Shows "Today: Xh Ym tracked" for the Expanded Mode.

  Fetches today's time entries from the Early API and sums their durations.
  Refreshes on mount and whenever the tracking state changes (stop/start).

  SVELTE 5 CONCEPTS:
  - `$state()`: Local reactive state for the computed summary.
  - `$effect()`: Re-fetches when the tracking store's isTracking changes.
-->
<script lang="ts">
  import { getTodayEntries } from '$lib/api';
  import { tracking } from '$lib/stores/tracking';

  let summary = $state('0h 0m');
  let loaded = $state(false);

  // Re-fetch whenever tracking starts or stops
  $effect(() => {
    // Access isTracking so this effect re-runs on change
    const _trigger = $tracking.isTracking;
    fetchSummary();
  });

  async function fetchSummary() {
    try {
      const data = await getTodayEntries();
      // Early API returns an array of time entries, each with a duration object
      const entries = Array.isArray(data) ? data : data.timeEntries || [];

      let totalMinutes = 0;
      for (const entry of entries) {
        if (entry.duration) {
          // Duration may be in seconds or as startedAt/stoppedAt
          if (typeof entry.duration === 'number') {
            totalMinutes += entry.duration / 60;
          } else if (entry.duration.startedAt && entry.duration.stoppedAt) {
            const start = new Date(entry.duration.startedAt).getTime();
            const end = new Date(entry.duration.stoppedAt).getTime();
            totalMinutes += (end - start) / 60_000;
          }
        }
      }

      const hours = Math.floor(totalMinutes / 60);
      const mins = Math.round(totalMinutes % 60);
      summary = `${hours}h ${mins}m`;
    } catch {
      if (!loaded) summary = '—';
    } finally {
      loaded = true;
    }
  }
</script>

<div class="summary">
  Today: {!loaded ? '...' : summary} tracked
</div>

<style>
  .summary {
    padding: 8px 12px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
    font-size: 12px;
    color: var(--text-secondary, #888);
  }
</style>
