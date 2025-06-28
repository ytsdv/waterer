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
</div>
