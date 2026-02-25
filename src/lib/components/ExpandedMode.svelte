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
  import { tracking } from '$lib/stores/tracking';
  import { stopTracking, startTracking, getActivities } from '$lib/api';
  import { activities, type Activity } from '$lib/stores/activities';
  import ActivitySwitcher from './ActivitySwitcher.svelte';
  import Timer from './Timer.svelte';
  import DescriptionEditor from './DescriptionEditor.svelte';
  import DaySummary from './DaySummary.svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  let selectedActivity = $state<Activity | null>(null);
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
    try {
      await stopTracking();
    } catch (err) {
      console.error('Failed to stop:', err);
    } finally {
      actionLoading = false;
    }
  }

  async function handlePlay() {
    if (actionLoading || !selectedActivity) return;
    actionLoading = true;
    try {
      const result = await startTracking(selectedActivity.id);
      tracking.set({
        isTracking: true,
        activityId: String(selectedActivity.id),
        activityName: selectedActivity.name,
        activityColor: selectedActivity.color,
        startedAt: result?.startedAt || new Date().toISOString(),
        noteText: result?.note?.text || null,
      });
      selectedActivity = null;
    } catch (err) {
      console.error('Failed to start:', err);
    } finally {
      actionLoading = false;
    }
  }

  function handleActivitySelect(activity: Activity) {
    if (isTracking) {
      actionLoading = true;
      stopTracking()
        .then(() => startTracking(activity.id))
        .then((result) => {
          tracking.set({
            isTracking: true,
            activityId: String(activity.id),
            activityName: activity.name,
            activityColor: activity.color,
            startedAt: result?.startedAt || new Date().toISOString(),
            noteText: null,
          });
        })
        .catch((err) => console.error('Failed to switch:', err))
        .finally(() => {
          actionLoading = false;
          selectedActivity = null;
        });
    } else {
      selectedActivity = activity;
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
<div class="expanded" onmousedown={handleMouseDown}>
  <!-- Top row: activity switcher + timer + stop/play -->
  <div class="top-row">
    <ActivitySwitcher onSelect={handleActivitySelect} {selectedActivity} />

    <Timer startedAt={$tracking.startedAt} />

    {#if isTracking}
      <button class="action stop" onclick={handleStop} disabled={actionLoading} title="Stop">■</button>
    {:else if selectedActivity}
      <button class="action play" onclick={handlePlay} disabled={actionLoading} title="Start">▶</button>
    {/if}
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
    border: 1px solid rgba(255, 255, 255, 0.08);
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
    background: rgba(255, 255, 255, 0.1);
  }

  .action:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .stop { color: #ff6b6b; }
  .play { color: #4CAF50; }
</style>
