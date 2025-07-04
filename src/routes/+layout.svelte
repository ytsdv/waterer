<script lang="ts">
  import { setAppState } from "$lib/AppState.svelte";
  import AppHeader from "$lib/components/AppHeader.svelte";
  import { setSettingsState } from "$lib/SettingsState.svelte";
  import { setSipState } from "$lib/SipState.svelte";
  import { setThemeState } from "$lib/ThemeState.svelte";
  import "../app.css";

  let { children } = $props();

  setAppState();
  setSipState();
  setSettingsState();
  setThemeState();
</script>

<AppHeader />
{@render children()}
