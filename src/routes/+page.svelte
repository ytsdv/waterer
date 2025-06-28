<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface Sip {
    id: number;
    amount: number;
    created_at: string;
    notified_user: boolean;
  }

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

  function formatDate(dateString: string): string {
    const date = new Date(dateString);
    return date.toLocaleString();
  }

  function getTotalAmount(): number {
    return sips.reduce((total, sip) => total + sip.amount, 0);
  }

  onMount(() => {
    loadSips();
  });
</script>

<main class="container">
  <h1>💧 Water Tracker</h1>

  <div class="stats">
    <div class="stat-card">
      <h3>Total Sips</h3>
      <p class="stat-number">{sips.length}</p>
    </div>
    <div class="stat-card">
      <h3>Total Amount</h3>
      <p class="stat-number">{getTotalAmount()}ml</p>
    </div>
  </div>

  <div class="controls">
    <button class="btn-primary" onclick={loadSips} disabled={loading}>
      {loading ? "Loading..." : "Refresh"}
    </button>
  </div>

  {#if error}
    <div class="error">
      {error}
    </div>
  {/if}

  <div class="sips-container">
    <h2>Previous Sips</h2>
    
    {#if loading}
      <div class="loading">Loading sips...</div>
    {:else if sips.length === 0}
      <div class="empty">No sips recorded yet.</div>
    {:else}
      <div class="sips-list">
        {#each sips as sip (sip.id)}
          <div class="sip-card">
            <div class="sip-amount">{sip.amount}ml</div>
            <div class="sip-date">{formatDate(sip.created_at)}</div>
            {#if sip.notified_user}
              <div class="notification-badge">✓</div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
</main>

<style>
</style>


