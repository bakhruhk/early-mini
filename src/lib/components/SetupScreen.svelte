<!--
  SetupScreen.svelte — First-run screen for entering Early API credentials.

  Shown when the app has no stored credentials or they're invalid.
  On successful authentication, dispatches an 'authenticated' event to the parent.

  SVELTE 5 CONCEPTS USED:
  - `$state()`: Local reactive state for form fields and loading/error states.
  - `$props()`: Receives an `onAuthenticated` callback from the parent.
  - `onclick` / `onsubmit`: Svelte 5 uses lowercase DOM event attributes (not on:click).
-->
<script lang="ts">
  import { authenticate } from '$lib/api';

  // Callback prop: called when authentication succeeds.
  // The parent (+page.svelte) passes this in to switch from SetupScreen to PillMode.
  let { onAuthenticated }: { onAuthenticated: () => void } = $props();

  // Form state — $state() makes these reactive so the UI updates when they change
  let apiKey = $state('');
  let apiSecret = $state('');
  let error = $state('');
  let loading = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    error = '';
    loading = true;

    try {
      await authenticate(apiKey, apiSecret);
      // Success — tell the parent to switch to the main view
      onAuthenticated();
    } catch (err: any) {
      // Show the error from the Rust backend (e.g. "Authentication failed (401): ...")
      error = typeof err === 'string' ? err : err.message || 'Authentication failed';
    } finally {
      loading = false;
    }
  }
</script>

<div class="setup">
  <h2>Early Mini</h2>
  <p class="subtitle">Connect your Early account</p>

  <form onsubmit={handleSubmit}>
    <input
      type="text"
      placeholder="API Key"
      bind:value={apiKey}
      disabled={loading}
      autocomplete="off"
    />
    <input
      type="password"
      placeholder="API Secret"
      bind:value={apiSecret}
      disabled={loading}
      autocomplete="off"
    />

    {#if error}
      <p class="error">{error}</p>
    {/if}

    <button type="submit" disabled={loading || !apiKey || !apiSecret}>
      {loading ? 'Connecting...' : 'Connect'}
    </button>
  </form>

  <p class="hint">
    Generate your API Key & Secret at
    <span class="link">app.early.app → Settings</span>
  </p>
</div>

<style>
  .setup {
    padding: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    background: var(--bg, #1e1e1e);
    color: var(--text, #e0e0e0);
    height: 100vh;
    box-sizing: border-box;
  }

  h2 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
  }

  .subtitle {
    font-size: 12px;
    color: var(--text-secondary, #888);
    margin: 0;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 100%;
    max-width: 260px;
    margin-top: 8px;
  }

  input {
    padding: 8px 10px;
    border-radius: 6px;
    border: 1px solid var(--border, #333);
    background: var(--input-bg, #2a2a2a);
    color: var(--text, #e0e0e0);
    font-size: 13px;
    outline: none;
  }

  input:focus {
    border-color: var(--accent, #4CAF50);
  }

  button {
    padding: 8px;
    border-radius: 6px;
    border: none;
    background: var(--accent, #4CAF50);
    color: white;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .error {
    color: #ff6b6b;
    font-size: 11px;
    margin: 0;
  }

  .hint {
    font-size: 10px;
    color: var(--text-secondary, #666);
    margin-top: 8px;
    text-align: center;
  }

  .link {
    color: var(--accent, #4CAF50);
  }
</style>
