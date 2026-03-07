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
  import { checkAuth, loadSettings, loadWindowState, saveWindowState, startTracking, stopTracking } from '$lib/api';
  import { activities } from '$lib/stores/activities';
  import { selectedActivity } from '$lib/stores/selection';
  import { connection } from '$lib/stores/connection';
  import { themePreference, applyTheme, type ThemePreference } from '$lib/stores/theme';
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

  // --- Current system theme ---
  let systemTheme = $state<'dark' | 'light'>('dark');

  // --- Idle opacity from settings ---
  let idleOpacity = $state(0.85);

  // --- Auth check + settings load on mount ---
  $effect(() => {
    checkAuth()
      .then(async (authed) => {
        view = authed ? 'main' : 'setup';
        // Load persisted settings (theme, opacity, etc.)
        try {
          const s = await loadSettings();
          if (s.theme_preference) {
            const pref = s.theme_preference as ThemePreference;
            themePreference.set(pref);
            applyTheme(pref, systemTheme);
          }
          if (typeof s.idle_opacity === 'number') {
            idleOpacity = s.idle_opacity;
            document.documentElement.style.opacity = String(idleOpacity);
          }
        } catch {}
      })
      .catch(() => {
        view = 'setup';
      });
  });

  // --- Detect system theme and listen for changes ---
  $effect(() => {
    const win = getCurrentWindow();
    let unlistenTheme: (() => void) | undefined;

    win.theme().then((t) => {
      if (t) {
        systemTheme = t as 'dark' | 'light';
        applyTheme($themePreference, systemTheme);
      }
    }).catch(() => {});

    win.onThemeChanged(({ payload: t }) => {
      systemTheme = (t as 'dark' | 'light') || 'dark';
      applyTheme($themePreference, systemTheme);
    }).then((fn) => { unlistenTheme = fn; });

    return () => {
      if (unlistenTheme) unlistenTheme();
    };
  });

  // --- Idle opacity: fade on mouse enter/leave ---
  $effect(() => {
    const root = document.documentElement;
    root.style.transition = 'opacity 0.2s ease';
    root.style.opacity = String(idleOpacity);

    function handleEnter() {
      root.style.opacity = '1';
    }
    function handleLeave() {
      root.style.opacity = String(idleOpacity);
    }

    root.addEventListener('mouseenter', handleEnter);
    root.addEventListener('mouseleave', handleLeave);

    return () => {
      root.removeEventListener('mouseenter', handleEnter);
      root.removeEventListener('mouseleave', handleLeave);
    };
  });

  // --- Listen for Rust polling events ---
  $effect(() => {
    let unlistenUpdate: (() => void) | undefined;
    let unlistenError: (() => void) | undefined;

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

      // Successful poll — mark connection as online
      connection.set({
        online: true,
        lastError: null,
        lastSuccessAt: new Date().toISOString(),
      });
    }).then((fn) => {
      unlistenUpdate = fn;
    });

    listen<{ error: string; is_auth_error: boolean }>('tracking-error', (event) => {
      connection.update((c) => ({
        ...c,
        online: false,
        lastError: event.payload.error,
      }));
    }).then((fn) => {
      unlistenError = fn;
    });

    return () => {
      if (unlistenUpdate) unlistenUpdate();
      if (unlistenError) unlistenError();
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

  function isEditableTarget(target: EventTarget | null): boolean {
    if (!(target instanceof HTMLElement)) return false;
    if (target.isContentEditable) return true;
    return !!target.closest('input, textarea, select, [contenteditable="true"]');
  }

  async function handleKeydown(e: KeyboardEvent) {
    if (view !== 'main') return;
    if (isEditableTarget(e.target)) return;

    // Activity cycling only when idle (no live tracking session)
    if ((e.key === 'ArrowDown' || e.key === 'ArrowUp') && !$tracking.isTracking) {
      const list = $activities;
      if (!list || list.length === 0) return;

      const currentId = $selectedActivity?.id ?? null;
      let currentIndex = currentId ? list.findIndex((a) => a.id === currentId) : -1;
      if (currentIndex < 0) currentIndex = 0;

      const delta = e.key === 'ArrowDown' ? 1 : -1;
      const nextIndex = (currentIndex + delta + list.length) % list.length;
      selectedActivity.set(list[nextIndex]);
      e.preventDefault();
      return;
    }

    // Enter toggles start/stop
    if (e.key === 'Enter') {
      e.preventDefault();

      // If tracking: stop
      if ($tracking.isTracking) {
        try {
          await stopTracking();
          tracking.set({
            isTracking: false,
            activityId: null,
            activityName: null,
            activityColor: null,
            startedAt: null,
            noteText: null,
          });
        } catch (err) {
          console.error('Failed to stop tracking (Enter):', err);
        }
        return;
      }

      // If idle and we have a selection: start
      const activity = $selectedActivity;
      if (!activity) return;
      try {
        const result = await startTracking(activity.id);
        tracking.set({
          isTracking: true,
          activityId: String(activity.id),
          activityName: activity.name,
          activityColor: activity.color,
          startedAt: result?.startedAt || new Date().toISOString(),
          noteText: result?.note?.text || null,
        });
        selectedActivity.set(null);
      } catch (err) {
        console.error('Failed to start tracking (Enter):', err);
      }
    }
  }
</script>

<!-- Keyboard shortcuts (only active when the window is focused) -->
<svelte:window onkeydown={handleKeydown} />

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

  /* --- Dark theme (default) --- */
  :global(:root),
  :global(:root[data-theme="dark"]) {
    --bg: rgba(30, 30, 30, 0.78);
    --text: #e0e0e0;
    --text-secondary: #888;
    --border: rgba(255, 255, 255, 0.08);
    --input-bg: rgba(42, 42, 42, 0.9);
    --accent: #4CAF50;
    --action-hover: rgba(255, 255, 255, 0.1);
  }

  /* --- Light theme --- */
  :global(:root[data-theme="light"]) {
    --bg: rgba(255, 255, 255, 0.78);
    --text: #1a1a1a;
    --text-secondary: #666;
    --border: rgba(0, 0, 0, 0.1);
    --input-bg: rgba(245, 245, 245, 0.9);
    --accent: #2E7D32;
    --action-hover: rgba(0, 0, 0, 0.06);
  }

  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 48px;
    color: var(--text-secondary, #888);
    font-size: 13px;
  }
</style>
