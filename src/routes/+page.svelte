<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    AppHeader,
    StatCard,
    Button,
    ErrorMessage,
    SipsList,
    TimerIndicator,
    type Sip,
  } from "$lib/components";
  import { getAppState } from "$lib/AppState.svelte";
  import { getSipState } from "$lib/SipState.svelte";
  import { getSettingsState } from "$lib/SettingsState.svelte";

  const sipState = getSipState();
  const appState = getAppState();
  const settingsState = getSettingsState();
</script>

<main
  class="mx-auto px-8 max-w-4xl h-screen sm:px-4 overflow-y-auto flex flex-col"
>
  <AppHeader />
  
  <div class="flex gap-4 mb-8 justify-center items-center">
    <StatCard title="Total Sips" value={sipState.sips.length} />
    <StatCard title="Total Amount" value="{sipState.totalAmount}ml" />
  </div>

  <div class="flex gap-4 mb-8 justify-center items-center">
    <Button
      onclick={async () => {
        try {
          await invoke("take_sip", {
            amount: settingsState.settings.sipAmountMl,
          });
          sipState.updateSips();
        } catch (error) {
          console.error("Error taking sip:", error);
        }
      }}
    >
      Take a sip ({settingsState.settings.sipAmountMl}ml)
    </Button>
  </div>

  {#if sipState.error}
    <ErrorMessage message={sipState.error} />
  {/if}

  <SipsList sips={sipState.sips} loading={sipState.loading} />
</main>
