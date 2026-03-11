<!--
  PillMode.svelte — Compact pill-shaped miniplayer showing the current tracking state.

  LAYOUT (left to right):
  ● Activity Name  01:23:45  ■  ▾
  
  STATES:
  - Tracking: colored dot + activity name + live timer + stop (■) icon + expand (▾) chevron
  - Activity selected but not tracking: hollow dot + activity name + 00:00:00 + play (▶) icon + expand (▾)
  - Idle: hollow dot + "Select..." + 00:00:00 + no action icon + expand (▾)

  SVELTE 5 CONCEPTS USED:
  - `$state()`: Reactive local state (e.g. activities list, loading flags)
  - `$effect()`: Side effects that react to state changes (polling timer, window resize)
  - `$derived()`: Computed values that auto-update when dependencies change
  - Store auto-subscribe `$tracking`: reads from the tracking store reactively
  - `{#if}` / `{#each}`: Conditional and list rendering blocks
  - `onclick`: Svelte 5 uses lowercase DOM event attributes
-->
<script lang="ts">
  import { tracking, type TrackingState } from '$lib/stores/tracking';
  import { activities, type Activity } from '$lib/stores/activities';
  import { selectedActivity } from '$lib/stores/selection';
  import { connection } from '$lib/stores/connection';
  import { getActivities, isBenignStopTrackingError, stopTracking, startTracking } from '$lib/api';
  import ActivitySwitcher from './ActivitySwitcher.svelte';
  import Timer from './Timer.svelte';
  import ConnectionIndicator from './ConnectionIndicator.svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  /** Loading flag to prevent double-clicks on stop/play */
  let actionLoading = $state(false);

  /** Is the user currently tracking time? */
  let isTracking = $derived($tracking.isTracking);

  // Fetch activities on mount if not already loaded
  $effect(() => {
    if ($activities.length === 0) {
      getActivities()
        .then((data) => {
          const list = Array.isArray(data) ? data : data.activities || [];
          activities.set(list.map((a: any) => ({ id: String(a.id), name: a.name, color: a.color })));
        })
        .catch(() => {});
    }
  });

  async function handleStop() {
    if (actionLoading) return;
    actionLoading = true;
    const previous = $tracking;
    const optimisticStopState: TrackingState = {
      isTracking: false,
      activityId: null,
      activityName: null,
      activityColor: null,
      startedAt: null,
      noteText: null,
      notePending: false,
      notePendingUntil: null,
      actionPending: true,
      actionPendingUntil: Date.now() + 15000,
      actionDesiredIsTracking: false,
      actionPendingActivityId: null,
    };
    tracking.set(optimisticStopState);
    try {
      await stopTracking();
    } catch (err) {
      console.error('Failed to stop tracking:', err);
      if (isBenignStopTrackingError(err)) {
        return;
      }
      tracking.set({
        ...previous,
        actionPending: false,
        actionPendingUntil: null,
        actionDesiredIsTracking: null,
        actionPendingActivityId: null,
      });
    } finally {
      actionLoading = false;
    }
  }

  async function handlePlay() {
    if (actionLoading || !$selectedActivity) return;
    actionLoading = true;
    const activity = $selectedActivity;
    if (!activity) {
      actionLoading = false;
      return;
    }
    const optimisticStartedAt = new Date().toISOString();
    const optimisticStartState: TrackingState = {
      isTracking: true,
      activityId: String(activity.id),
      activityName: activity.name,
      activityColor: activity.color,
      startedAt: optimisticStartedAt,
      noteText: null,
      notePending: false,
      notePendingUntil: null,
      actionPending: true,
      actionPendingUntil: Date.now() + 15000,
      actionDesiredIsTracking: true,
      actionPendingActivityId: String(activity.id),
    };
    tracking.set(optimisticStartState);
    try {
      const result = await startTracking(activity.id);
      tracking.update((t) => ({
        ...t,
        startedAt: result?.startedAt || t.startedAt,
        noteText: result?.note?.text || t.noteText,
      }));
      selectedActivity.set(null);
    } catch (err) {
      console.error('Failed to start tracking:', err);
      tracking.set({
        isTracking: false,
        activityId: null,
        activityName: null,
        activityColor: null,
        startedAt: null,
        noteText: null,
        notePending: false,
        notePendingUntil: null,
        actionPending: false,
        actionPendingUntil: null,
        actionDesiredIsTracking: null,
        actionPendingActivityId: null,
      });
    } finally {
      actionLoading = false;
    }
  }

  function handleActivitySelect(activity: Activity) {
    if (isTracking) {
      actionLoading = true;
      const previous = $tracking;
      const optimisticStartedAt = new Date().toISOString();
      const optimisticSwitchState: TrackingState = {
        isTracking: true,
        activityId: String(activity.id),
        activityName: activity.name,
        activityColor: activity.color,
        startedAt: optimisticStartedAt,
        noteText: null,
        notePending: false,
        notePendingUntil: null,
        actionPending: true,
        actionPendingUntil: Date.now() + 15000,
        actionDesiredIsTracking: true,
        actionPendingActivityId: String(activity.id),
      };
      tracking.set(optimisticSwitchState);
      (async () => {
        try {
          try {
            await stopTracking();
          } catch (err) {
            if (!isBenignStopTrackingError(err)) throw err;
          }
          const result = await startTracking(activity.id);
          tracking.update((t) => ({
            ...t,
            startedAt: result?.startedAt || t.startedAt,
            noteText: result?.note?.text || null,
          }));
        } catch (err) {
          console.error('Failed to switch activity:', err);
          tracking.set({
            ...previous,
            actionPending: false,
            actionPendingUntil: null,
            actionDesiredIsTracking: null,
            actionPendingActivityId: null,
          });
        } finally {
          actionLoading = false;
          selectedActivity.set(null);
        }
      })();
    } else {
      selectedActivity.set(activity);
    }
  }

  function handleMouseDown(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (target.closest('button') || target.closest('.dropdown') ||
        target.closest('input') || target.closest('textarea')) return;
    e.preventDefault();
    getCurrentWindow().startDragging();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="pill" onmousedown={handleMouseDown}>
  <!-- Activity switcher (dot + name + chevron) -->
  <ActivitySwitcher onSelect={handleActivitySelect} selectedActivity={$selectedActivity} />

  <!-- Live timer -->
  <Timer startedAt={$tracking.startedAt} isTracking={isTracking} activityColor={$tracking.activityColor} />

  <!-- Stop / Play button -->
  {#if isTracking}
    <button class="action stop" onclick={handleStop} disabled={actionLoading || !$connection.online} title="Stop">■</button>
  {:else if $selectedActivity}
    <button class="action play" onclick={handlePlay} disabled={actionLoading || !$connection.online} title="Start">▶</button>
  {/if}
  <ConnectionIndicator />
</div>

<style>
  .pill {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 12px;
    height: 48px;
    background: var(--bg, #1e1e1e);
    border-radius: 24px;
    cursor: grab;
    position: relative;
    user-select: none;
    min-width: 0;
    width: 100%;
    box-sizing: border-box;
    border: 1px solid var(--border, rgba(255, 255, 255, 0.08));
  }

  .action {
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    font-size: 12px;
    line-height: 1;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .action:hover {
    background: var(--action-hover, rgba(255, 255, 255, 0.1));
  }

  .action:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .stop { color: #ff6b6b; }
  .play { color: #4CAF50; }
</style>
