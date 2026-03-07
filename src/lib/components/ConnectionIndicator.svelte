<!--
  ConnectionIndicator.svelte — Small status dot showing API connectivity.

  - Green pulse: connected (last poll succeeded)
  - Red: disconnected (last poll failed)
  - Tooltip shows the last error message when offline
-->
<script lang="ts">
  import { connection } from '$lib/stores/connection';
</script>

<span
  class="status-dot"
  class:online={$connection.online}
  class:offline={!$connection.online}
  title={$connection.online ? 'Connected' : ($connection.lastError || 'Disconnected')}
></span>

<style>
  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .status-dot.online {
    background: #4CAF50;
    animation: pulse 2s ease-in-out infinite;
  }

  .status-dot.offline {
    background: #ff6b6b;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }
</style>
