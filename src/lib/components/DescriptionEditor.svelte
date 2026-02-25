<!--
  DescriptionEditor.svelte — Inline plain-text editor for the tracking note.

  Two display modes:
  - `compact`: Shows a single-line truncated preview with a pencil icon (Card Mode).
  - `full`: Shows a multi-line textarea (Expanded Mode).

  SVELTE 5 CONCEPTS:
  - `$props()`: Receives mode ('compact'|'full') and the current note text.
  - `$state()`: Local editing state and draft text.
  - `$effect()`: Syncs the draft with the prop when external updates arrive.
-->
<script lang="ts">
  import { updateNote } from '$lib/api';
  import { tracking } from '$lib/stores/tracking';

  let {
    mode = 'compact',
  }: {
    /** Display mode: 'compact' for Card, 'full' for Expanded. */
    mode?: 'compact' | 'full';
  } = $props();

  /** Whether the user is actively editing. */
  let editing = $state(false);

  /** Draft text while editing. */
  let draft = $state('');

  /** The current note from the tracking store. */
  let noteText = $derived($tracking.noteText || '');

  // Sync draft with store when not editing
  $effect(() => {
    if (!editing) {
      draft = noteText;
    }
  });

  function startEdit() {
    draft = noteText;
    editing = true;
  }

  async function saveEdit() {
    editing = false;
    if (draft !== noteText) {
      try {
        await updateNote(draft);
        // Update the store optimistically
        tracking.update((t) => ({ ...t, noteText: draft }));
      } catch (err) {
        console.error('Failed to update note:', err);
        // Revert on failure
        draft = noteText;
      }
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (mode === 'compact' && e.key === 'Enter') {
      e.preventDefault();
      saveEdit();
    }
    if (e.key === 'Escape') {
      editing = false;
      draft = noteText;
    }
  }
</script>

{#if mode === 'compact'}
  <!-- Card Mode: single-line preview with edit icon -->
  <div class="compact">
    {#if editing}
      <input
        class="edit-input"
        type="text"
        bind:value={draft}
        onblur={saveEdit}
        onkeydown={handleKeydown}
        autofocus
      />
    {:else}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <span class="preview" class:placeholder={!noteText} onclick={startEdit}>
        {noteText || 'No description'}
      </span>
      <button class="edit-btn" onclick={startEdit} title="Edit description">✏️</button>
    {/if}
  </div>
{:else}
  <!-- Expanded Mode: multi-line textarea -->
  <div class="full">
    <label class="label">Description:</label>
    <textarea
      class="edit-textarea"
      bind:value={draft}
      onfocus={() => { editing = true; }}
      onblur={saveEdit}
      onkeydown={handleKeydown}
      placeholder="Add a description..."
      rows="3"
    ></textarea>
  </div>
{/if}

<style>
  .compact {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
    min-height: 28px;
  }

  .preview {
    font-size: 12px;
    color: var(--text, #e0e0e0);
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    cursor: text;
  }

  .preview.placeholder {
    color: var(--text-secondary, #666);
    font-style: italic;
  }

  .edit-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 12px;
    padding: 2px;
    flex-shrink: 0;
    opacity: 0.6;
  }

  .edit-btn:hover {
    opacity: 1;
  }

  .edit-input {
    flex: 1;
    padding: 4px 6px;
    border-radius: 4px;
    border: 1px solid var(--border, #333);
    background: var(--input-bg, #2a2a2a);
    color: var(--text, #e0e0e0);
    font-size: 12px;
    outline: none;
  }

  .edit-input:focus {
    border-color: var(--accent, #4CAF50);
  }

  .full {
    padding: 8px 12px;
    border-top: 1px solid rgba(255, 255, 255, 0.06);
  }

  .label {
    display: block;
    font-size: 11px;
    color: var(--text-secondary, #888);
    margin-bottom: 4px;
  }

  .edit-textarea {
    width: 100%;
    padding: 6px 8px;
    border-radius: 6px;
    border: 1px solid var(--border, #333);
    background: var(--input-bg, #2a2a2a);
    color: var(--text, #e0e0e0);
    font-size: 12px;
    font-family: inherit;
    outline: none;
    resize: vertical;
    box-sizing: border-box;
  }

  .edit-textarea:focus {
    border-color: var(--accent, #4CAF50);
  }

  .edit-textarea::placeholder {
    color: var(--text-secondary, #666);
    font-style: italic;
  }
</style>
