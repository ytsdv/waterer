<script lang="ts">
  interface Props {
    message?: string;
  }

  let { message = "No data available." }: Props = $props();
</script>

<div class="text-center py-8 text-gray-600 italic dark:text-gray-400">
  {message}
</div>
