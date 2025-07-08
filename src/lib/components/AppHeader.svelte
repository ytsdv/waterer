<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { TimerIndicator } from ".";
  import { getAppState } from "$lib/AppState.svelte";
  import { getUpdateState } from "$lib/UpdateState.svelte";
  import { Settings, Download } from "@lucide/svelte";
  import { page } from "$app/state";

  const appState = getAppState();
  const updateState = getUpdateState();

  function toggleTimer() {
    invoke("toggle_timer");
  }

  async function installUpdate() {
    try {
      await updateState.installUpdate();
    } catch (err) {
      // Error is handled by updateState
    }
  }

  const isSettingsPage = $derived(page.url.pathname === "/settings");
</script>

<div
  class="w-full flex flex-row items-center gap-2 px-2 bg-white/80 dark:bg-gray-800/80 border-b border-gray-200 dark:border-gray-700 rounded mb-4 min-h-8 justify-between"
>
  <div class="flex flex-row items-center gap-2">
    <TimerIndicator isRunning={appState.timerStarted} />
    <button
      class="text-xs font-medium text-primary-700 dark:text-primary-100 cursor-pointer hover:underline select-none ml-1"
      onclick={toggleTimer}
      tabindex="0"
    >
      {#if appState.timerStarted}
        Stop Timer
      {:else}
        Start Timer
      {/if}
    </button>
  </div>
  
  <div class="flex flex-row items-center gap-2">
    {#if updateState.hasUpdateAvailable}
      <button
        class="flex items-center gap-1 text-xs font-medium text-blue-600 dark:text-blue-400 cursor-pointer hover:underline select-none px-2 py-1 bg-blue-50 dark:bg-blue-900/20 rounded transition-colors"
        onclick={installUpdate}
        disabled={updateState.isInstalling || updateState.isDownloading}
        title="Update to version {updateState.currentVersion} available"
      >
        <Download class="w-3 h-3" />
        {#if updateState.isInstalling || updateState.isDownloading}
          {updateState.isDownloading ? 'Downloading...' : 'Installing...'}
        {:else}
          Update Available
        {/if}
      </button>
    {/if}
    
    {#if !isSettingsPage}
      <a
        class="text-xs font-medium text-primary-700 dark:text-primary-100 cursor-pointer hover:underline select-none"
        href="/settings"
      >
        <Settings class="w-4 h-4" />
      </a>
    {/if}
  </div>
</div>
