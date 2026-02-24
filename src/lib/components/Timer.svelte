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
  let { startedAt }: { startedAt: string | null } = $props();

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
</script>

<span class="timer">{display}</span>

<style>
  .timer {
    font-family: 'SF Mono', 'Menlo', 'Monaco', 'Consolas', monospace;
    font-size: 13px;
    /* tabular-nums ensures all digits are the same width, preventing layout shifts */
    font-variant-numeric: tabular-nums;
    color: var(--text-secondary, #aaa);
    white-space: nowrap;
  }
</style>
