<script lang="ts">
  import "../app.css";
  import { setAppState } from "$lib/AppState.svelte";
  import { setSettingsState } from "$lib/SettingsState.svelte";
  import { setSipState } from "$lib/SipState.svelte";
  import { setThemeState } from "$lib/ThemeState.svelte";
  import { setUpdateState } from "$lib/UpdateState.svelte";
  import { UpdateNotification } from "$lib/components";

  // Initialize all states
  setAppState();
  setSipState();
  setSettingsState();
  setThemeState();
  setUpdateState();
</script>

<main class="min-h-screen bg-gradient-to-br from-primary-50 to-accent-50 dark:from-gray-900 dark:to-gray-800">
  <slot />
  <UpdateNotification />
</main>
