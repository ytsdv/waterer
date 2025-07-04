<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button } from ".";
  import { getAppState } from "$lib/AppState.svelte";

  const appState = getAppState();

  interface Props {
    isRunning: boolean;
  }

  let { isRunning }: Props = $props();
</script>

<div class="flex items-center gap-1">
  <div class="relative">
    <div
      class="w-2 h-2 rounded-full {isRunning
        ? 'bg-success-500'
        : 'bg-gray-400'} transition-colors duration-300"
    ></div>
    {#if isRunning}
      <div
        class="absolute inset-0 w-2 h-2 rounded-full bg-success-500 animate-ping opacity-60"
      ></div>
    {/if}
  </div>
  <span
    class="text-xs font-medium {isRunning
      ? 'text-success-600 dark:text-success-400'
      : 'text-gray-600 dark:text-gray-400'} transition-colors duration-300"
  >
    {isRunning ? "Timer Running" : "Timer Stopped"}
  </span>
</div>
