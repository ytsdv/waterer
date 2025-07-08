<script lang="ts">
  import { getUpdateState } from "$lib/UpdateState.svelte";
  import { Button } from ".";
  import { Download, X, AlertCircle } from "@lucide/svelte";

  const updateState = getUpdateState();

  async function installNow() {
    try {
      await updateState.installUpdate();
    } catch (err) {
      // Error is handled by updateState
    }
  }

  function installLater() {
    updateState.dismissUpdate();
  }
</script>

{#if updateState.isUpdateReady}
  <div 
    class="fixed top-4 right-4 z-50 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg max-w-md"
    role="alert"
  >
    <div class="p-4">
      <div class="flex items-start gap-3">
        <div class="flex-shrink-0">
          <AlertCircle class="w-5 h-5 text-blue-500" />
        </div>
        <div class="flex-1 min-w-0">
          <h3 class="text-sm font-medium text-gray-900 dark:text-gray-100">
            Update Available
          </h3>
          <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
            Version {updateState.currentVersion} is ready to install.
            {#if updateState.updateDescription}
              <br />
              <span class="text-xs">{updateState.updateDescription}</span>
            {/if}
          </p>
          
          {#if updateState.isInstalling || updateState.isDownloading}
            <div class="mt-3">
              <div class="text-xs text-gray-600 dark:text-gray-400 mb-1">
                {updateState.isDownloading ? 'Downloading...' : 'Installing...'}
                {#if updateState.downloadProgress.progress > 0}
                  ({updateState.downloadProgress.progress}%)
                {/if}
              </div>
              <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                <div 
                  class="bg-blue-500 h-2 rounded-full transition-all duration-300"
                  style="width: {updateState.downloadProgress.progress}%"
                ></div>
              </div>
            </div>
          {:else}
            <div class="flex gap-2 mt-3">
              <Button
                size="sm"
                onclick={installNow}
                disabled={updateState.isInstalling}
                class="flex items-center gap-1"
              >
                <Download class="w-3 h-3" />
                Install Now
              </Button>
              <Button
                variant="outline"
                size="sm"
                onclick={installLater}
                disabled={updateState.isInstalling}
              >
                Later
              </Button>
            </div>
          {/if}

          {#if updateState.error}
            <p class="text-xs text-red-600 dark:text-red-400 mt-2">
              {updateState.error}
            </p>
          {/if}
        </div>
        {#if !updateState.isInstalling && !updateState.isDownloading}
          <button
            onclick={installLater}
            class="flex-shrink-0 text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300"
          >
            <X class="w-4 h-4" />
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}