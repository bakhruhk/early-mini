<!--
  ActivitySwitcher.svelte — Shared dropdown for switching the active activity.

  Used by PillMode (inline chevron), CardMode and ExpandedMode (clickable activity name).
  Renders the activity color dot + name + chevron, and opens a dropdown on click.

  SVELTE 5 CONCEPTS:
  - `$props()`: Receives callbacks and display mode config from the parent.
  - `$state()`: Local dropdown open/close state.
  - Store auto-subscribe `$activities`: reads the global activities list.
-->
<script lang="ts">
  import { activities, type Activity } from '$lib/stores/activities';
  import { tracking } from '$lib/stores/tracking';

  let {
    onSelect,
    selectedActivity = null,
  }: {
    /** Called when the user picks an activity from the dropdown. */
    onSelect: (activity: Activity) => void;
    /** Currently selected (but not yet tracking) activity. */
    selectedActivity?: Activity | null;
  } = $props();

  let open = $state(false);

  /** Is the user currently tracking? */
  let isTracking = $derived($tracking.isTracking);

  /** Color for the dot */
  let dotColor = $derived(
    $tracking.activityColor || selectedActivity?.color || null
  );

  /** Display name */
  let displayName = $derived(
    $tracking.activityName || selectedActivity?.name || 'Select...'
  );

  function toggle() {
    open = !open;
  }

  function select(activity: Activity) {
    open = false;
    onSelect(activity);
  }

  // Close dropdown when clicking outside
  function handleWindowClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest('.switcher')) {
      open = false;
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<svelte:window onclick={handleWindowClick} />

<div class="switcher">
  <!-- Dot + name: non-interactive, allows window dragging through -->
  <span
    class="dot"
    class:hollow={!isTracking && !selectedActivity}
    style:background-color={dotColor || 'transparent'}
    style:border-color={dotColor || '#666'}
  ></span>
  <span class="name" class:idle={!isTracking && !selectedActivity}>
    {displayName}
  </span>
  <!-- Only the chevron is a button — click target for the dropdown -->
  <button class="chevron" onclick={toggle} title="Switch activity">
    {open ? '▴' : '▾'}
  </button>

  {#if open}
    <div class="dropdown">
      {#each $activities as activity (activity.id)}
        <button
          class="dropdown-item"
          class:active={$tracking.activityId === activity.id}
          onclick={() => select(activity)}
        >
          <span class="dropdown-dot" style:background-color={activity.color}></span>
          {activity.name}
        </button>
      {:else}
        <span class="dropdown-empty">No activities</span>
      {/each}
    </div>
  {/if}
</div>

<style>
  .switcher {
    position: relative;
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
    flex: 1;
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
    min-width: 0;
  }

  .name.idle {
    color: var(--text-secondary, #888);
  }

  .chevron {
    font-size: 11px;
    color: var(--text-secondary, #888);
    flex-shrink: 0;
    background: none;
    border: none;
    cursor: pointer;
    padding: 2px 4px;
    border-radius: 4px;
  }

  .chevron:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text, #e0e0e0);
  }

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
    min-width: 140px;
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

  .dropdown-item.active {
    background: rgba(255, 255, 255, 0.05);
    font-weight: 500;
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
