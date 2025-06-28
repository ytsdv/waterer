<script lang="ts">
  import SipCard from './SipCard.svelte';
  import LoadingState from './LoadingState.svelte';
  import EmptyState from './EmptyState.svelte';

  interface Sip {
    id: number;
    amount: number;
    created_at: string;
    notified_user: boolean;
  }

  interface Props {
    sips: Sip[];
    loading: boolean;
  }

  let { sips, loading }: Props = $props();
</script>

<div class="bg-white rounded-xl p-6 shadow-lg mb-8 dark:bg-gray-800 dark:text-gray-100 flex-1 overflow-y-auto min-h-[200px]">
  <h2 class="mb-4 text-xl font-medium text-gray-800 dark:text-gray-100">Previous Sips</h2>
  
  {#if loading}
    <LoadingState message="Loading sips..." />
  {:else if sips.length === 0}
    <EmptyState message="No sips recorded yet." />
  {:else}
    <div class="flex flex-col gap-3 overflow-y-auto pb-4">
      {#each sips as sip (sip.id)}
        <SipCard {sip} />
      {/each}
    </div>
  {/if}
</div>
