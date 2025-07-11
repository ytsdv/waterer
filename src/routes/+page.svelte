<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
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
  <div class="flex gap-4 mb-8 justify-center items-center">
    <StatCard title="Total Sips" value={sipState.sips.length} />
    <StatCard title="Daily Progress">
      <div
        class={sipState.totalAmount >= settingsState.settings.dailyGoalMl
          ? "text-green-600 dark:text-green-400"
          : "text-primary-600 dark:text-primary-400"}
      >
        <span>
          {sipState.totalAmount}ml
        </span>
        <span> / {settingsState.settings.dailyGoalMl}ml</span>
      </div>
    </StatCard>
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
