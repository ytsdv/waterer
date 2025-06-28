<script lang="ts">
  interface Sip {
    id: number;
    amount: number;
    created_at: string;
    notified_user: boolean;
  }

  interface Props {
    sip: Sip;
  }

  let { sip }: Props = $props();

  function formatDate(dateString: string): string {
    const date = new Date(dateString);
    return date.toLocaleString();
  }
</script> 

<div class="flex items-center justify-between p-4 bg-gray-50 rounded-lg border-l-4 border-primary-500 relative dark:bg-gray-700">
  <div class="font-bold text-primary-800 text-lg dark:text-primary-400">{sip.amount}ml</div>
  <div class="text-gray-600 text-sm dark:text-gray-400">{formatDate(sip.created_at)}</div>
  {#if sip.notified_user}
    <div class="absolute top-2 right-2 bg-success-500 text-white rounded-full w-5 h-5 flex items-center justify-center text-xs">✓</div>
  {/if}
</div>
