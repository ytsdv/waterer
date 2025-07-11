<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { TimerIndicator } from ".";
  import { getAppState } from "$lib/AppState.svelte";
  import { Settings } from "@lucide/svelte";
  import { page } from "$app/state";

  const appState = getAppState();

  function toggleTimer() {
    invoke("toggle_timer");
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
    <span class="text-xs font-medium text-primary-700 dark:text-primary-100">
      {new Date(appState.sessionStart).toLocaleString()}
    </span>
  </div>
  {#if !isSettingsPage}
    <div>
      <a
        class="text-xs font-medium text-primary-700 dark:text-primary-100 cursor-pointer hover:underline select-none"
        href="/settings"
      >
        <Settings class="w-4 h-4" />
      </a>
    </div>
  {/if}
</div>
