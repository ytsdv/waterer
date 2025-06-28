<script lang="ts">
  interface Props {
    message: string;
  }

  let { message }: Props = $props();
</script>

<div class="bg-destructive-50 text-destructive-600 p-4 rounded-lg mb-4 text-center dark:bg-destructive-900 dark:text-destructive-300">
  {message}
</div>
