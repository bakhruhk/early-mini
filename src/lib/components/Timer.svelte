<!--
  Timer.svelte — Displays elapsed time since tracking started.

  SVELTE 5 CONCEPTS USED:
  - `$props()`: Declares component props. The parent passes data in like HTML attributes:
      <Timer startedAt={someValue} />
  - `$state()`: Creates reactive local state. When it changes, the DOM auto-updates.
  - `$effect()`: Runs a side effect (here, a setInterval) that auto-tracks its dependencies.
    It re-runs whenever `startedAt` changes, and the return function is cleanup (like React useEffect).
-->
<script lang="ts">
  import { formatElapsed } from '$lib/utils';

  // Props passed from parent component.
  // `startedAt` is the ISO timestamp of when tracking began, or null if idle.
  let {
    startedAt,
    isTracking = false,
    activityColor = null,
  }: {
    startedAt: string | null;
    isTracking?: boolean;
    activityColor?: string | null;
  } = $props();

  // Reactive local state for the formatted time string.
  // $state() makes this reactive — changing it automatically updates the DOM.
  let display = $state('00:00:00');

  // $effect() is Svelte 5's way to run side effects.
  // It tracks which reactive values are read inside it (here: `startedAt`)
  // and re-runs whenever they change.
  $effect(() => {
    if (!startedAt) {
      display = '00:00:00';
      return;
    }

    // Update immediately on mount/change
    display = formatElapsed(startedAt);

    // Then tick every second for a smooth counting display
    const interval = setInterval(() => {
      display = formatElapsed(startedAt);
    }, 1000);

    // Cleanup: returning a function from $effect is like React's useEffect cleanup.
    // Called when the effect re-runs (startedAt changes) or the component unmounts.
    return () => clearInterval(interval);
  });

  function colorToRgba(color: string | null, alpha: number): string {
    if (!color) return `rgba(255, 255, 255, ${alpha})`;

    const normalized = color.trim();
    const fullHex = normalized.match(/^#([0-9a-fA-F]{6})$/);
    if (fullHex) {
      const hex = fullHex[1];
      const r = parseInt(hex.slice(0, 2), 16);
      const g = parseInt(hex.slice(2, 4), 16);
      const b = parseInt(hex.slice(4, 6), 16);
      return `rgba(${r}, ${g}, ${b}, ${alpha})`;
    }

    const shortHex = normalized.match(/^#([0-9a-fA-F]{3})$/);
    if (shortHex) {
      const hex = shortHex[1];
      const r = parseInt(hex[0] + hex[0], 16);
      const g = parseInt(hex[1] + hex[1], 16);
      const b = parseInt(hex[2] + hex[2], 16);
      return `rgba(${r}, ${g}, ${b}, ${alpha})`;
    }

    const rgb = normalized.match(/^rgb\((\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})\)$/i);
    if (rgb) {
      return `rgba(${rgb[1]}, ${rgb[2]}, ${rgb[3]}, ${alpha})`;
    }

    return `rgba(255, 255, 255, ${alpha})`;
  }
</script>
<span
  class="timer"
  class:active={isTracking}
  style:background-color={isTracking ? colorToRgba(activityColor, 0.22) : 'transparent'}
>
  {display}
</span>

<style>
  .timer {
    font-family: 'SF Mono', 'Menlo', 'Monaco', 'Consolas', monospace;
    font-size: 13px;
    /* tabular-nums ensures all digits are the same width, preventing layout shifts */
    font-variant-numeric: tabular-nums;
    color: var(--text-secondary, #aaa);
    white-space: nowrap;
    border-radius: 999px;
    padding: 0;
    transition: background-color 0.2s ease, color 0.2s ease, padding 0.2s ease;
  }

  .timer.active {
    color: var(--text, #e0e0e0);
    padding: 3px 8px;
    border: 1px solid var(--border, rgba(255, 255, 255, 0.12));
  }
</style>
