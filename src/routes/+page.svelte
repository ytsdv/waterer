<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { StatCard, Button, ErrorMessage, SipsList, type Sip } from "$lib/components";

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
    console.log("updating sips");
    
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

    return () => {
      if (interval) {
        clearInterval(interval);
      }
    };
  });

</script>

<main class="mx-auto px-8 max-w-4xl max-h-screen overflow-y-auto">
  <h1 class="text-center mb-8 text-4xl font-semibold">💧 Water Tracker</h1>

  <div class="flex gap-4 mb-8 justify-center">
    <StatCard title="Total Sips" value={sips.length} />
    <StatCard title="Total Amount" value="{getTotalAmount()}ml" />
  </div>

  {#if error}
    <ErrorMessage message={error} />
  {/if}

  <SipsList {sips} {loading} />
</main>


