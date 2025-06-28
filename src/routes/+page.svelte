<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { StatCard, Button, ErrorMessage, SipsList, type Sip } from "$lib/components";
  import { listen } from '@tauri-apps/api/event';
  import { getAppState } from "$lib/AppState.svelte";

  let sips = $state<Sip[]>([]);
  let loading = $state(true);
  let error = $state("");

  async function loadSips() {
    try {
      loading = true;
      error = "";
      const result = await invoke("get_sips");
      sips = result as Sip[];
    } catch (err) {
      error = `Failed to load sips: ${err}`;
      console.error("Error loading sips:", err);
    } finally {
      loading = false;
    }
  }

  function getTotalAmount(): number {
    return sips.reduce((total, sip) => total + sip.amount, 0);
  }

  async function updateSips() {
    try {
      const result = await invoke("get_sips");
      sips = result as Sip[];
    } catch (err) {
      error = `Failed to load sips: ${err}`;
      console.error("Error loading sips:", err);
    } finally {
    }
  }

  let interval = $state<number | null>(null);

  $effect(() => {
    loadSips();
    interval = setInterval(updateSips, 1000);
  });


  const appState = getAppState();

</script>

<main class="mx-auto px-8 max-w-4xl h-screen sm:px-4 overflow-y-auto flex flex-col">
  <h1 class="text-center mb-8 text-4xl font-semibold">💧 Water Tracker</h1>

  <div class="flex gap-4 mb-8 justify-center items-center">
    <StatCard title="Total Sips" value={sips.length} />
    <StatCard title="Total Amount" value="{getTotalAmount()}ml" />
  </div>

  <div class="flex gap-4 mb-8 justify-center items-center">
    <Button onclick={() => {
      invoke("toggle_timer");
    }}>
      {#if appState.timerStarted}
        Stop Timer
      {:else}
        Start Timer
      {/if}
    </Button>
    <Button onclick={() => {
      invoke("take_sip");
    }}>
      Take a sip
    </Button>
  </div>

  {#if error}
    <ErrorMessage message={error} />
  {/if}

  <SipsList {sips} {loading} />
</main>


