<!--
  ExpandedMode.svelte — Full-size miniplayer view (~320×260px).

  LAYOUT:
  ┌──────────────────────────────────────┐
  │ 🟢 Deep Work ▾       01:23:45  ■   │
  │──────────────────────────────────────│
  │ Description:                         │
  │ ┌──────────────────────────────────┐ │
  │ │ Working on auth module for the   │ │
  │ │ Early Mini project...            │ │
  │ └──────────────────────────────────┘ │
  │──────────────────────────────────────│
  │  Today: 6h 23m tracked              │
  └──────────────────────────────────────┘

  Reuses: ActivitySwitcher, Timer, DescriptionEditor (full mode), DaySummary.
-->
<script lang="ts">
  import { tracking, type TrackingState } from '$lib/stores/tracking';
  import { isBenignStopTrackingError, stopTracking, startTracking, getActivities } from '$lib/api';
  import { activities, type Activity } from '$lib/stores/activities';
  import { selectedActivity } from '$lib/stores/selection';
  import { connection } from '$lib/stores/connection';
  import ActivitySwitcher from './ActivitySwitcher.svelte';
  import Timer from './Timer.svelte';
  import DescriptionEditor from './DescriptionEditor.svelte';
  import DaySummary from './DaySummary.svelte';
  import ConnectionIndicator from './ConnectionIndicator.svelte';
  import TopControls from './TopControls.svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  let actionLoading = $state(false);
  let isTracking = $derived($tracking.isTracking);

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
      console.error('Failed to stop:', err);
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
      console.error('Failed to start:', err);
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
          console.error('Failed to switch:', err);
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
    if (target.closest('button') || target.closest('.dropdown') || target.closest('.utility-controls') ||
        target.closest('input') || target.closest('textarea')) return;
    e.preventDefault();
    getCurrentWindow().startDragging();
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="expanded" onmousedown={handleMouseDown}>
  <!-- Top row: activity switcher + timer + stop/play -->
  <div class="top-row">
    <ActivitySwitcher onSelect={handleActivitySelect} selectedActivity={$selectedActivity} />

    <TopControls />
    <Timer startedAt={$tracking.startedAt} isTracking={isTracking} activityColor={$tracking.activityColor} />

    {#if isTracking}
      <button class="action stop" onclick={handleStop} disabled={actionLoading || !$connection.online} title="Stop">■</button>
    {:else if $selectedActivity}
      <button class="action play" onclick={handlePlay} disabled={actionLoading || !$connection.online} title="Start">▶</button>
    {/if}
    <ConnectionIndicator />
  </div>

  <!-- Full description editor -->
  <DescriptionEditor mode="full" />

  <!-- Today's summary -->
  <DaySummary />
</div>

<style>
  .expanded {
    display: flex;
    flex-direction: column;
    background: var(--bg, #1e1e1e);
    border-radius: 12px;
    border: 1px solid var(--border, rgba(255, 255, 255, 0.08));
    width: 100%;
    height: 100%;
    box-sizing: border-box;
    cursor: grab;
    user-select: none;
    overflow: hidden;
  }

  .top-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    min-height: 36px;
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
