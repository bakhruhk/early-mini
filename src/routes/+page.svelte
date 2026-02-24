<!--
  +page.svelte — Root page that routes between SetupScreen and PillMode.

  On mount, checks for stored credentials via check_auth.
  - If authenticated: shows PillMode (the mini-player pill)
  - If not: shows SetupScreen (API key/secret input)

  SVELTE 5 CONCEPTS:
  - `$state()`: Reactive local state for the current view.
  - `$effect()`: Runs the initial auth check on mount.
  - `{#if}`: Conditional rendering — only one view is shown at a time.
-->
<script lang="ts">
  import { checkAuth } from '$lib/api';
  import SetupScreen from '$lib/components/SetupScreen.svelte';
  import PillMode from '$lib/components/PillMode.svelte';

  // Which view to show: 'loading' | 'setup' | 'pill'
  let view = $state<'loading' | 'setup' | 'pill'>('loading');

  // On mount, check if user has stored credentials
  $effect(() => {
    checkAuth()
      .then((authed) => {
        view = authed ? 'pill' : 'setup';
      })
      .catch(() => {
        view = 'setup';
      });
  });

  // Called by SetupScreen after successful authentication
  function handleAuthenticated() {
    view = 'pill';
  }
</script>

{#if view === 'loading'}
  <div class="loading">...</div>
{:else if view === 'setup'}
  <SetupScreen onAuthenticated={handleAuthenticated} />
{:else}
  <PillMode />
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
