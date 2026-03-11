<script lang="ts">
  import { authenticate, loadSettings, saveSettings } from '$lib/api';
  import { applyTheme, themePreference, type ThemePreference } from '$lib/stores/theme';
  import { get } from 'svelte/store';

  let open = $state(false);
  let loading = $state(false);
  let saving = $state(false);
  let message = $state('');
  let error = $state('');

  let pollingInterval = $state(3);
  let apiKey = $state('');
  let apiSecret = $state('');

  let cachedSettings = $state<any>(null);

  function getResolvedTheme(): 'dark' | 'light' {
    return document.documentElement.getAttribute('data-theme') === 'light' ? 'light' : 'dark';
  }

  async function ensureSettings() {
    if (cachedSettings) return cachedSettings;
    loading = true;
    try {
      cachedSettings = await loadSettings();
      return cachedSettings;
    } finally {
      loading = false;
    }
  }

  async function toggleSettings() {
    open = !open;
    message = '';
    error = '';
    if (!open) return;

    try {
      const settings = await ensureSettings();
      pollingInterval = Number(settings?.polling_interval_secs ?? 3);
    } catch (err: any) {
      error = typeof err === 'string' ? err : err?.message || 'Failed to load settings.';
    }
  }

  async function handleThemeToggle() {
    message = '';
    error = '';

    const currentResolved = getResolvedTheme();
    const nextPref: ThemePreference = currentResolved === 'dark' ? 'light' : 'dark';

    themePreference.set(nextPref);
    applyTheme(nextPref, currentResolved);

    try {
      const settings = await ensureSettings();
      const updated = {
        ...settings,
        theme_preference: nextPref,
      };
      await saveSettings(updated);
      cachedSettings = updated;
    } catch (err: any) {
      error = typeof err === 'string' ? err : err?.message || 'Failed to persist theme.';
    }
  }

  async function saveForm() {
    if (saving) return;
    saving = true;
    message = '';
    error = '';

    try {
      const settings = await ensureSettings();
      const normalizedPolling = Math.max(1, Math.round(Number(pollingInterval) || 3));
      const updated = {
        ...settings,
        polling_interval_secs: normalizedPolling,
        theme_preference: get(themePreference),
      };

      await saveSettings(updated);
      cachedSettings = updated;

      const key = apiKey.trim();
      const secret = apiSecret.trim();
      if ((key && !secret) || (!key && secret)) {
        throw new Error('Enter both API key and API secret to update credentials.');
      }
      if (key && secret) {
        await authenticate(key, secret);
        apiKey = '';
        apiSecret = '';
      }

      message = 'Settings saved';
    } catch (err: any) {
      error = typeof err === 'string' ? err : err?.message || 'Failed to save settings.';
    } finally {
      saving = false;
    }
  }

  function handleWindowClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest('.utility-controls')) {
      open = false;
    }
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div class="utility-controls">
  <button class="icon-btn" onclick={toggleSettings} title="Settings">⚙</button>
  <button
    class="icon-btn"
    onclick={handleThemeToggle}
    title={getResolvedTheme() === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
  >
    {getResolvedTheme() === 'dark' ? '☀︎' : '☾'}
  </button>

  {#if open}
    <div class="popover">
      <div class="section-title">Settings</div>

      <label class="label">
        Polling interval (sec)
        <input
          type="number"
          min="1"
          max="60"
          step="1"
          bind:value={pollingInterval}
          disabled={loading || saving}
        />
      </label>

      <div class="section-title creds">Update API credentials</div>
      <input
        type="text"
        placeholder="API Key"
        bind:value={apiKey}
        disabled={loading || saving}
        autocomplete="off"
      />
      <input
        type="password"
        placeholder="API Secret"
        bind:value={apiSecret}
        disabled={loading || saving}
        autocomplete="off"
      />

      {#if error}
        <div class="error">{error}</div>
      {/if}
      {#if message}
        <div class="ok">{message}</div>
      {/if}

      <button class="save-btn" onclick={saveForm} disabled={loading || saving}>
        {saving ? 'Saving...' : 'Save'}
      </button>
    </div>
  {/if}
</div>

<style>
  .utility-controls {
    position: relative;
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
    z-index: 20;
  }

  .icon-btn {
    width: 26px;
    height: 26px;
    border-radius: 6px;
    border: none;
    background: transparent;
    color: var(--text-secondary, #888);
    font-size: 14px;
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    padding: 0;
  }

  .icon-btn:hover {
    background: var(--action-hover, rgba(255, 255, 255, 0.1));
    color: var(--text, #e0e0e0);
  }

  .popover {
    position: absolute;
    top: calc(100% + 6px);
    right: 0;
    width: 230px;
    padding: 10px;
    border-radius: 10px;
    border: 1px solid var(--border, rgba(255, 255, 255, 0.12));
    background: var(--bg, #1e1e1e);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.35);
    display: flex;
    flex-direction: column;
    gap: 8px;
    z-index: 50;
  }

  .section-title {
    font-size: 11px;
    color: var(--text-secondary, #888);
    font-weight: 600;
  }

  .section-title.creds {
    margin-top: 2px;
  }

  .label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 11px;
    color: var(--text-secondary, #888);
  }

  input {
    width: 100%;
    box-sizing: border-box;
    border-radius: 6px;
    border: 1px solid var(--border, rgba(255, 255, 255, 0.12));
    background: var(--input-bg, rgba(42, 42, 42, 0.9));
    color: var(--text, #e0e0e0);
    font-size: 12px;
    padding: 6px 8px;
    outline: none;
  }

  input:focus {
    border-color: var(--accent, #4CAF50);
  }

  .save-btn {
    border: none;
    border-radius: 6px;
    padding: 6px 8px;
    font-size: 12px;
    font-weight: 600;
    background: var(--accent, #4CAF50);
    color: white;
    cursor: pointer;
  }

  .save-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .error {
    color: #ff6b6b;
    font-size: 11px;
  }

  .ok {
    color: #4CAF50;
    font-size: 11px;
  }
</style>
