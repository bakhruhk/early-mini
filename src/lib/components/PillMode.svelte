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
  import { tracking } from '$lib/stores/tracking';
  import { activities, type Activity } from '$lib/stores/activities';
  import { selectedActivity } from '$lib/stores/selection';
  import { connection } from '$lib/stores/connection';
  import { getActivities, stopTracking, startTracking } from '$lib/api';
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
    try {
      await stopTracking();
    } catch (err) {
      console.error('Failed to stop tracking:', err);
    } finally {
      actionLoading = false;
    }
  }

  async function handlePlay() {
    if (actionLoading || !$selectedActivity) return;
    actionLoading = true;
    try {
      const result = await startTracking($selectedActivity.id);
      tracking.set({
        isTracking: true,
        activityId: String($selectedActivity.id),
        activityName: $selectedActivity.name,
        activityColor: $selectedActivity.color,
        startedAt: result?.startedAt || new Date().toISOString(),
        noteText: result?.note?.text || null,
      });
      selectedActivity.set(null);
    } catch (err) {
      console.error('Failed to start tracking:', err);
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
        .catch((err) => console.error('Failed to switch activity:', err))
        .finally(() => {
          actionLoading = false;
          selectedActivity.set(null);
        });
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
  <Timer startedAt={$tracking.startedAt} />

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
