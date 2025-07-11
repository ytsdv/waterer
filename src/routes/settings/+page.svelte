<script lang="ts">
  import { Button, ErrorMessage, Input, Select } from "$lib/components";
  import { goto } from "$app/navigation";
  import { ArrowLeft, Save } from "@lucide/svelte";
  import { getSettingsState } from "$lib/SettingsState.svelte";
  import { getThemeState, type Theme } from "$lib/ThemeState.svelte";

  const settingsState = getSettingsState();
  const themeState = getThemeState();

  let sipAmount = $state(settingsState.settings.sipAmountMl);
  let notificationInterval = $state(
    settingsState.settings.notificationIntervalMinutes
  );
  let selectedTheme = $state<Theme>(themeState.theme);
  let success = $state("");
  let dailyGoal = $state(settingsState.settings.dailyGoalMl);
  const themeOptions = [
    { value: "light", label: "Light" },
    { value: "dark", label: "Dark" },
    { value: "system", label: "System" },
  ];

  // Sync local state with settings state
  $effect(() => {
    sipAmount = settingsState.settings.sipAmountMl;
    notificationInterval = settingsState.settings.notificationIntervalMinutes;
    dailyGoal = settingsState.settings.dailyGoalMl;
    selectedTheme = themeState.theme;
  });

  // Watch for theme changes and apply them
  $effect(() => {
    if (selectedTheme !== themeState.theme) {
      themeState.setTheme(selectedTheme);
    }
  });

  async function saveSettings() {
    try {
      success = "";

      await settingsState.saveSettings({
        sipAmountMl: sipAmount,
        notificationIntervalMinutes: notificationInterval,
        dailyGoalMl: dailyGoal,
      });

      success = "Settings saved successfully!";
      setTimeout(() => {
        success = "";
      }, 3000);
    } catch (err) {
      // Error is handled by the settings state
    }
  }

  function goBack() {
    goto("/");
  }
</script>

<main
  class="mx-auto px-8 max-w-2xl h-screen sm:px-4 overflow-y-auto flex flex-col"
>
  <div class="flex items-center gap-4 mb-6">
    <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
      Settings
    </h1>
    <button
      onclick={goBack}
      class="flex items-center gap-2 text-primary-700 dark:text-primary-100 hover:underline"
    >
      <ArrowLeft class="w-4 h-4" />
      Back
    </button>
  </div>

  {#if settingsState.error}
    <ErrorMessage message={settingsState.error} />
  {/if}

  {#if success}
    <div
      class="bg-success-500/10 text-success-600 dark:text-success-400 p-4 rounded-lg mb-6 text-center"
    >
      {success}
    </div>
  {/if}

  <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 mb-6">
    <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
      Hydration Settings
    </h2>

    <div class="space-y-6">
      <!-- Default Sip Amount -->
      <Input
        label="Default Sip Amount (ml)"
        type="number"
        min={1}
        max={500}
        bind:value={sipAmount}
        placeholder="Enter sip amount in ml"
        helpText="Amount of water recorded when you take a sip (1-500 ml)"
      />
      <Input
        label="Daily Goal (ml)"
        type="number"
        min={1000}
        max={5000}
        bind:value={dailyGoal}
        placeholder="Enter daily goal in ml"
        helpText="How much water you want to drink daily (1-5000 ml)"
      />
      <!-- Notification Interval -->
      <Input
        label="Notification Interval (minutes)"
        type="number"
        min={1}
        max={180}
        disabled
        bind:value={notificationInterval}
        placeholder="Enter interval in minutes"
        helpText="How often you want to be reminded to drink water (1-180 minutes)"
      />
    </div>
  </div>

  <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg p-6 mb-6">
    <h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-4">
      Appearance Settings
    </h2>

    <div class="space-y-6">
      <!-- Theme Selection -->
      <Select
        label="Theme"
        options={themeOptions}
        bind:value={selectedTheme}
        helpText="Choose your preferred theme or follow system preference"
      />
    </div>
  </div>

  <!-- Save Button -->
  <div class="flex justify-end">
    <Button
      onclick={saveSettings}
      disabled={settingsState.loading}
      class="flex items-center gap-2"
    >
      <Save class="w-4 h-4" />
      {settingsState.loading ? "Saving..." : "Save Settings"}
    </Button>
  </div>
</main>
