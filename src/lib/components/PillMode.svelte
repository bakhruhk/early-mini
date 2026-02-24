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
  import { getCurrentTracking, getActivities, stopTracking, startTracking } from '$lib/api';
  import Timer from './Timer.svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  // --- Reactive state ---

  /** Whether the activity switcher dropdown is open */
  let dropdownOpen = $state(false);

  /** Selected activity when not tracking (for play button) */
  let selectedActivity = $state<Activity | null>(null);

  /** Loading flag to prevent double-clicks on stop/play */
  let actionLoading = $state(false);

  // --- Derived values ---
  // $derived() creates a computed value that automatically re-evaluates
  // when any reactive dependency (here, $tracking) changes.

  /** Is the user currently tracking time? */
  let isTracking = $derived($tracking.isTracking);

  /** The color to show on the activity dot */
  let dotColor = $derived(
    $tracking.activityColor || selectedActivity?.color || null
  );

  /** The name to display */
  let displayName = $derived(
    $tracking.activityName || selectedActivity?.name || 'Select...'
  );

  // --- Polling: fetch tracking state every 5 seconds ---
  $effect(() => {
    fetchTrackingState();

    const interval = setInterval(fetchTrackingState, 5000);
    return () => clearInterval(interval);
  });

  // --- Fetch activities on mount ---
  $effect(() => {
    fetchActivities();
  });

  // --- Functions ---

  async function fetchTrackingState() {
    try {
      const data = await getCurrentTracking();
      // The Early API returns the tracking entry directly, or errors if nothing is tracked
      if (data && data.activity) {
        tracking.set({
          isTracking: true,
          activityId: data.activity.id,
          activityName: data.activity.name,
          activityColor: data.activity.color,
          startedAt: data.startedAt,
          noteText: data.note?.text || null,
        });
      }
    } catch {
      // No active tracking or error — set idle state
      tracking.set({
        isTracking: false,
        activityId: null,
        activityName: null,
        activityColor: null,
        startedAt: null,
        noteText: null,
      });
    }
  }

  async function fetchActivities() {
    try {
      const data = await getActivities();
      // Early API returns { activities: [...] } or the array directly
      const list = Array.isArray(data) ? data : data.activities || [];
      activities.set(
        list.map((a: any) => ({
          id: a.id,
          name: a.name,
          color: a.color,
        }))
      );
    } catch (err) {
      console.error('Failed to fetch activities:', err);
    }
  }

  async function handleStop() {
    if (actionLoading) return;
    actionLoading = true;
    try {
      await stopTracking();
      await fetchTrackingState();
    } catch (err) {
      console.error('Failed to stop tracking:', err);
    } finally {
      actionLoading = false;
    }
  }

  async function handlePlay() {
    if (actionLoading || !selectedActivity) return;
    actionLoading = true;
    try {
      await startTracking(selectedActivity.id);
      await fetchTrackingState();
      selectedActivity = null;
    } catch (err) {
      console.error('Failed to start tracking:', err);
    } finally {
      actionLoading = false;
    }
  }

  function selectActivity(activity: Activity) {
    selectedActivity = activity;
    dropdownOpen = false;

    // If not currently tracking, just select it (user can press play).
    // If tracking, switch to this activity immediately.
    if (isTracking) {
      actionLoading = true;
      stopTracking()
        .then(() => startTracking(activity.id))
        .then(() => fetchTrackingState())
        .catch((err) => console.error('Failed to switch activity:', err))
        .finally(() => {
          actionLoading = false;
          selectedActivity = null;
        });
    }
  }

  function toggleDropdown() {
    dropdownOpen = !dropdownOpen;
  }

  // --- Window dragging (frameless window) ---
  // Allow dragging the pill to move the window
  async function handleMouseDown(e: MouseEvent) {
    // Don't drag when clicking on interactive elements
    const target = e.target as HTMLElement;
    if (target.closest('button') || target.closest('.dropdown')) return;

    try {
      await getCurrentWindow().startDragging();
    } catch {
      // Dragging may fail in dev mode, that's fine
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="pill" onmousedown={handleMouseDown}>
  <!-- Activity color dot -->
  <span
    class="dot"
    class:hollow={!isTracking && !selectedActivity}
    style:background-color={dotColor || 'transparent'}
    style:border-color={dotColor || '#666'}
  ></span>

  <!-- Activity name -->
  <span class="name" class:idle={!isTracking && !selectedActivity}>
    {displayName}
  </span>

  <!-- Live timer -->
  <Timer startedAt={$tracking.startedAt} />

  <!-- Stop / Play button -->
  {#if isTracking}
    <button
      class="action stop"
      onclick={handleStop}
      disabled={actionLoading}
      title="Stop tracking"
    >■</button>
  {:else if selectedActivity}
    <button
      class="action play"
      onclick={handlePlay}
      disabled={actionLoading}
      title="Start tracking {selectedActivity.name}"
    >▶</button>
  {/if}

  <!-- Expand chevron / Activity switcher toggle -->
  <button class="chevron" onclick={toggleDropdown} title="Switch activity">
    ▾
  </button>

  <!-- Activity switcher dropdown -->
  {#if dropdownOpen}
    <div class="dropdown">
      {#each $activities as activity (activity.id)}
        <button
          class="dropdown-item"
          onclick={() => selectActivity(activity)}
        >
          <span
            class="dropdown-dot"
            style:background-color={activity.color}
          ></span>
          {activity.name}
        </button>
      {:else}
        <span class="dropdown-empty">No activities</span>
      {/each}
    </div>
  {/if}
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
    /* Subtle border so the pill is visible against dark backgrounds */
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
    border: 2px solid transparent;
  }

  .dot.hollow {
    background-color: transparent !important;
    border: 2px solid #666;
  }

  .name {
    font-size: 13px;
    color: var(--text, #e0e0e0);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    min-width: 0;
  }

  .name.idle {
    color: var(--text-secondary, #888);
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

  .stop {
    color: #ff6b6b;
  }

  .play {
    color: #4CAF50;
  }

  .chevron {
    background: none;
    border: none;
    color: var(--text-secondary, #888);
    cursor: pointer;
    padding: 4px;
    font-size: 12px;
    line-height: 1;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .chevron:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text, #e0e0e0);
  }

  /* --- Activity dropdown --- */

  .dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    margin-top: 4px;
    background: var(--bg, #1e1e1e);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    max-height: 200px;
    overflow-y: auto;
    z-index: 100;
    padding: 4px;
  }

  .dropdown-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px;
    background: none;
    border: none;
    color: var(--text, #e0e0e0);
    font-size: 12px;
    cursor: pointer;
    border-radius: 4px;
    text-align: left;
  }

  .dropdown-item:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .dropdown-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .dropdown-empty {
    display: block;
    padding: 8px;
    color: var(--text-secondary, #888);
    font-size: 12px;
    text-align: center;
  }
</style>
