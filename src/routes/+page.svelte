<!--
  +page.svelte — Root page that handles:
  1. Auth check → SetupScreen or main UI
  2. Mode detection: Pill / Card / Expanded based on window size
  3. Listening to Rust-emitted 'tracking-update' events
  4. Window position/size persistence

  SVELTE 5 CONCEPTS:
  - `$state()`: Reactive state for view, mode, and window dimensions.
  - `$effect()`: Auth check, event listener setup, window resize observer.
  - `$derived()`: Mode computed from current dimensions.
  - `{#if}`: Conditional rendering for view + mode switching.
-->
<script lang="ts">
  import { checkAuth, loadWindowState, saveWindowState } from '$lib/api';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWindow, PhysicalPosition, PhysicalSize } from '@tauri-apps/api/window';
  import { tracking } from '$lib/stores/tracking';
  import SetupScreen from '$lib/components/SetupScreen.svelte';
  import PillMode from '$lib/components/PillMode.svelte';
  import CardMode from '$lib/components/CardMode.svelte';
  import ExpandedMode from '$lib/components/ExpandedMode.svelte';

  // --- View state ---
  let view = $state<'loading' | 'setup' | 'main'>('loading');

  // --- Window dimensions for mode detection ---
  let winWidth = $state(280);
  let winHeight = $state(52);

  // --- Mode breakpoints ---
  // Pill:     height < 80
  // Card:     80 ≤ height < 200
  // Expanded: height ≥ 200
  type Mode = 'pill' | 'card' | 'expanded';
  let mode: Mode = $derived(
    winHeight < 80 ? 'pill' : winHeight < 200 ? 'card' : 'expanded'
  );

  // --- Auth check on mount ---
  $effect(() => {
    checkAuth()
      .then((authed) => {
        view = authed ? 'main' : 'setup';
      })
      .catch(() => {
        view = 'setup';
      });
  });

  // --- Listen for Rust polling events ---
  $effect(() => {
    let unlisten: (() => void) | undefined;

    listen<{ is_tracking: boolean; data: any }>('tracking-update', (event) => {
      const { is_tracking, data } = event.payload;
      if (is_tracking && data?.activity) {
        tracking.set({
          isTracking: true,
          activityId: data.activity.id,
          activityName: data.activity.name,
          activityColor: data.activity.color,
          startedAt: data.startedAt,
          noteText: data.note?.text || null,
        });
      } else {
        tracking.set({
          isTracking: false,
          activityId: null,
          activityName: null,
          activityColor: null,
          startedAt: null,
          noteText: null,
        });
      }
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) unlisten();
    };
  });

  // --- Track window size for mode detection ---
  $effect(() => {
    let unlistenResize: (() => void) | undefined;
    let unlistenMove: (() => void) | undefined;
    let saveTimeout: ReturnType<typeof setTimeout> | undefined;

    const win = getCurrentWindow();

    // Get initial size
    win.innerSize().then((size) => {
      winWidth = size.width;
      winHeight = size.height;
    });

    // Restore saved position/size
    loadWindowState().then((geo) => {
      if (geo) {
        win.setPosition(new PhysicalPosition(geo.x, geo.y));
        win.setSize(new PhysicalSize(geo.width, geo.height));
        winWidth = geo.width;
        winHeight = geo.height;
      }
    }).catch(() => {});

    // Listen for resize
    win.onResized((size) => {
      winWidth = size.payload.width;
      winHeight = size.payload.height;
      debounceSave();
    }).then((fn) => { unlistenResize = fn; });

    // Listen for move
    win.onMoved(() => {
      debounceSave();
    }).then((fn) => { unlistenMove = fn; });

    function debounceSave() {
      if (saveTimeout) clearTimeout(saveTimeout);
      saveTimeout = setTimeout(async () => {
        try {
          const size = await win.innerSize();
          const pos = await win.innerPosition();
          saveWindowState(pos.x, pos.y, size.width, size.height);
        } catch {}
      }, 500);
    }

    return () => {
      if (unlistenResize) unlistenResize();
      if (unlistenMove) unlistenMove();
      if (saveTimeout) clearTimeout(saveTimeout);
    };
  });

  function handleAuthenticated() {
    view = 'main';
  }
</script>

{#if view === 'loading'}
  <div class="loading">...</div>
{:else if view === 'setup'}
  <SetupScreen onAuthenticated={handleAuthenticated} />
{:else if mode === 'pill'}
  <PillMode />
{:else if mode === 'card'}
  <CardMode />
{:else}
  <ExpandedMode />
{/if}

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: transparent;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }

  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 48px;
    color: #888;
    font-size: 13px;
  }
</style>
